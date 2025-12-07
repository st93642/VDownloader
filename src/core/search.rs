/*****************************************************************************/
/*                                                                           */
/*  search.rs                                            TTTTTTTT SSSSSSS II */
/*                                                          TT    SS      II */
/*  By: st93642@students.tsi.lv                             TT    SSSSSSS II */
/*                                                          TT         SS II */
/*  Created: Dec 07 2025 16:17 st93642                      TT    SSSSSSS II */
/*  Updated: Dec 07 2025 18:40 st93642                                       */
/*                                                                           */
/*   Transport and Telecommunication Institute - Riga, Latvia                */
/*                       https://tsi.lv                                      */
/*****************************************************************************/

use crate::core::downloader::{Platform, VideoDownloader};
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use std::io::{BufRead, BufReader};
use thiserror::Error;
use tokio::process::Command;

#[derive(Error, Debug, Clone)]
pub enum SearchError {
    #[error("Invalid query: {0}")]
    InvalidQuery(String),

    #[error("Command failed: {0}")]
    CommandFailed(String),

    #[error("JSON parse error: {0}")]
    JsonParseError(String),

    #[error("yt-dlp not found or not installed")]
    MissingYtDlp,

    #[error("Rate limit exceeded (HTTP 429): {0}")]
    RateLimited(String),

    #[error("IO error: {0}")]
    IoError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub url: String,
    pub thumbnail: Option<String>,
    pub duration: Option<u64>,
    pub uploader: Option<String>,
    pub view_count: Option<u64>,
    pub platform: Platform,
}

#[derive(Debug, Clone, Copy)]
pub struct SearchService {
    default_limit: u32,
}

impl SearchService {
    pub fn new(default_limit: u32) -> Self {
        let default_limit = default_limit.max(1);
        info!(
            "Creating SearchService with default limit: {}",
            default_limit
        );
        Self { default_limit }
    }

    pub fn default_limit(&self) -> u32 {
        self.default_limit
    }

    pub async fn search(
        &self,
        query: &str,
        limit: Option<u32>,
    ) -> Result<Vec<SearchResult>, SearchError> {
        let trimmed = query.trim();
        if trimmed.is_empty() {
            return Err(SearchError::InvalidQuery(
                "Query cannot be empty".to_string(),
            ));
        }

        let limit = limit
            .filter(|value| *value > 0)
            .unwrap_or(self.default_limit);

        // Heuristic to determine if the query is a URL or a search term
        let is_url_like = !trimmed.contains(char::is_whitespace) && trimmed.contains('.');
        let candidate_url = if is_url_like && !trimmed.starts_with("http") {
            format!("https://{}", trimmed)
        } else {
            trimmed.to_string()
        };

        if VideoDownloader::validate_url(&candidate_url).is_ok() {
            debug!("Executing search with URL: {}", candidate_url);
            return Self::execute_search_command(
                &candidate_url,
                &["--dump-json", "--flat-playlist", "--skip-download"],
                Some(limit),
            )
            .await;
        }

        // It's a keyword search - aggregate results from supported platforms
        debug!("Executing multi-platform search for: {}", trimmed);
        let mut tasks = Vec::new();

        // 1. YouTube Search
        let yt_expr = format!("ytsearch{}:{}", limit, trimmed);
        tasks.push(tokio::spawn(async move {
            Self::execute_search_command(
                &yt_expr,
                &["--dump-json", "--flat-playlist", "--skip-download"],
                None, // Limit is embedded in ytsearch prefix
            )
            .await
        }));

        // 2. Dzen Search
        let dzen_url = format!("https://dzen.ru/search?query={}", trimmed);
        let dzen_limit = limit;
        tasks.push(tokio::spawn(async move {
            Self::execute_search_command(
                &dzen_url,
                &["--dump-json", "--flat-playlist", "--skip-download"],
                Some(dzen_limit),
            )
            .await
        }));

        // 3. Rutube Search (using public API)
        let rutube_query = trimmed.to_string();
        let rutube_limit = limit;
        tasks.push(tokio::spawn(async move {
            Self::search_rutube(&rutube_query, rutube_limit).await
        }));

        // Note: VK does not support search without authentication.
        // VK API requires access tokens which is beyond scope for a simple downloader.

        let mut aggregated_results = Vec::new();
        let mut errors = Vec::new();

        for task in tasks {
            match task.await {
                Ok(Ok(results)) => aggregated_results.extend(results),
                Ok(Err(e)) => errors.push(e),
                Err(e) => errors.push(SearchError::CommandFailed(format!("Task join error: {}", e))),
            }
        }

        // Filter out Dzen article URLs (/a/) as they trigger a broken extractor
        aggregated_results.retain(|result| {
            if result.platform == Platform::Dzen && result.url.contains("/a/") {
                debug!("Filtering out Dzen article URL (unsupported): {}", result.url);
                false
            } else {
                true
            }
        });

        if aggregated_results.is_empty() && !errors.is_empty() {
            // If we got no results and only errors, return the first error
            return Err(errors.remove(0));
        }

        Ok(aggregated_results)
    }

    async fn execute_search_command(
        input: &str,
        args: &[&str],
        limit: Option<u32>,
    ) -> Result<Vec<SearchResult>, SearchError> {
        let mut cmd = Command::new("yt-dlp");
        cmd.arg(input).args(args);

        if let Some(l) = limit {
            cmd.arg("--playlist-items").arg(format!("1-{}", l));
        }

        let output = cmd.output().await.map_err(map_spawn_error)?;

        if !output.status.success() {
            let stderr_text = String::from_utf8_lossy(&output.stderr).to_string();
            // Don't log error for "Unsupported URL" as it might just mean the platform isn't supported for search
            if !stderr_text.contains("Unsupported URL") {
                error!(
                    "yt-dlp exited with {:?}; stderr: {}",
                    output.status.code(),
                    stderr_text
                );
            }
            return Err(interpret_command_failure(
                &stderr_text,
                output.status.code(),
            ));
        }

        parse_search_results(&output.stdout)
    }

    async fn search_rutube(
        query: &str,
        limit: u32,
    ) -> Result<Vec<SearchResult>, SearchError> {
        let url = format!(
            "https://rutube.ru/api/search/video/?query={}&page=1&per_page={}",
            urlencoding::encode(query),
            limit
        );

        debug!("Searching Rutube with URL: {}", url);

        let response = reqwest::get(&url)
            .await
            .map_err(|e| SearchError::IoError(format!("Rutube API request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(SearchError::CommandFailed(format!(
                "Rutube API returned status: {}",
                response.status()
            )));
        }

        let body = response
            .text()
            .await
            .map_err(|e| SearchError::IoError(format!("Failed to read Rutube response: {}", e)))?;

        parse_rutube_results(&body)
    }
}

fn map_spawn_error(err: std::io::Error) -> SearchError {
    if err.kind() == std::io::ErrorKind::NotFound {
        error!("yt-dlp executable is missing from PATH");
        SearchError::MissingYtDlp
    } else {
        error!("Failed to spawn yt-dlp: {}", err);
        SearchError::IoError(err.to_string())
    }
}

fn interpret_command_failure(stderr_text: &str, status_code: Option<i32>) -> SearchError {
    if is_rate_limit_message(stderr_text) {
        return SearchError::RateLimited(stderr_text.trim().to_string());
    }

    let status_msg = status_code
        .map(|code| format!("exit code {}", code))
        .unwrap_or_else(|| "terminated by signal".to_string());

    SearchError::CommandFailed(format!("{}: {}", status_msg, stderr_text.trim()))
}

fn is_rate_limit_message(message: &str) -> bool {
    let lower = message.to_lowercase();
    lower.contains("http error 429")
        || lower.contains("too many requests")
        || lower.contains("rate limit")
        || lower.contains("response code: 429")
}

#[derive(Debug, Deserialize)]
struct RutubeSearchResponse {
    results: Vec<RutubeVideo>,
}

#[derive(Debug, Deserialize)]
struct RutubeVideo {
    id: String,
    title: String,
    video_url: String,
    thumbnail_url: Option<String>,
    duration: Option<u64>,
    author: Option<RutubeAuthor>,
    hits: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct RutubeAuthor {
    name: String,
}

fn parse_rutube_results(json: &str) -> Result<Vec<SearchResult>, SearchError> {
    let response: RutubeSearchResponse = serde_json::from_str(json)
        .map_err(|e| SearchError::JsonParseError(format!("Failed to parse Rutube response: {}", e)))?;

    let results: Vec<SearchResult> = response
        .results
        .into_iter()
        .map(|video| SearchResult {
            id: video.id.clone(),
            title: video.title,
            url: video.video_url,
            thumbnail: video.thumbnail_url,
            duration: video.duration,
            uploader: video.author.map(|a| a.name),
            view_count: video.hits,
            platform: Platform::Rutube,
        })
        .collect();

    info!("Parsed {} Rutube search results", results.len());
    Ok(results)
}

fn parse_search_results(stdout: &[u8]) -> Result<Vec<SearchResult>, SearchError> {
    let cursor = Cursor::new(stdout);
    let reader = BufReader::new(cursor);
    let mut results = Vec::new();

    for (idx, line) in reader.lines().enumerate() {
        let line = line.map_err(|err| {
            SearchError::JsonParseError(format!("failed to read line {}: {}", idx + 1, err))
        })?;

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        match parse_single_result(trimmed) {
            Ok(result) => results.push(result),
            Err(err) => {
                warn!(
                    "Skipping malformed search result on line {}: {}",
                    idx + 1,
                    err
                );
            }
        }
    }

    info!("Parsed {} search results", results.len());
    Ok(results)
}

fn parse_single_result(line: &str) -> Result<SearchResult, SearchError> {
    let raw: RawSearchEntry =
        serde_json::from_str(line).map_err(|err| SearchError::JsonParseError(err.to_string()))?;
    SearchResult::try_from(raw)
}

#[derive(Debug, Deserialize)]
struct RawSearchEntry {
    id: Option<String>,
    title: Option<String>,
    url: Option<String>,
    webpage_url: Option<String>,
    extractor: Option<String>,
    extractor_key: Option<String>,
    uploader: Option<String>,
    channel: Option<String>,
    duration: Option<f64>,
    view_count: Option<f64>,
    thumbnail: Option<String>,
    thumbnails: Option<Vec<ThumbnailEntry>>,
}

#[derive(Debug, Deserialize)]
struct ThumbnailEntry {
    url: Option<String>,
}

impl TryFrom<RawSearchEntry> for SearchResult {
    type Error = SearchError;

    fn try_from(entry: RawSearchEntry) -> Result<Self, Self::Error> {
        let RawSearchEntry {
            id,
            title,
            url,
            webpage_url,
            extractor,
            extractor_key,
            uploader,
            channel,
            duration,
            view_count,
            thumbnail,
            thumbnails,
        } = entry;

        let id = id.ok_or_else(|| SearchError::JsonParseError("Missing 'id' field".to_string()))?;
        let title = title
            .ok_or_else(|| SearchError::JsonParseError("Missing 'title' field".to_string()))?;

        let resolved_url = url
            .or(webpage_url)
            .unwrap_or_else(|| format!("https://www.youtube.com/watch?v={}", id));

        let resolved_thumbnail = thumbnail.or_else(|| {
            thumbnails.and_then(|collection| collection.into_iter().find_map(|thumb| thumb.url))
        });

        let uploader = uploader.or(channel);

        let platform = detect_platform_from_metadata(
            extractor.as_deref(),
            extractor_key.as_deref(),
            &resolved_url,
        );

        Ok(SearchResult {
            id,
            title,
            url: resolved_url,
            thumbnail: resolved_thumbnail,
            duration: duration.map(|d| d as u64),
            uploader,
            view_count: view_count.map(|v| v as u64),
            platform,
        })
    }
}

fn detect_platform_from_metadata(
    extractor: Option<&str>,
    extractor_key: Option<&str>,
    url: &str,
) -> Platform {
    extractor
        .and_then(platform_from_hint)
        .or_else(|| extractor_key.and_then(platform_from_hint))
        .unwrap_or_else(|| VideoDownloader::detect_platform(url))
}

fn platform_from_hint(hint: &str) -> Option<Platform> {
    let hint = hint.to_lowercase();

    if hint.contains("youtube") {
        Some(Platform::YouTube)
    } else if hint.contains("tiktok") {
        Some(Platform::TikTok)
    } else if hint.contains("twitter") || hint.contains("x.com") {
        Some(Platform::Twitter)
    } else if hint.contains("instagram") {
        Some(Platform::Instagram)
    } else if hint.contains("reddit") {
        Some(Platform::Reddit)
    } else if hint.contains("vk") {
        Some(Platform::Vk)
    } else if hint.contains("rutube") {
        Some(Platform::Rutube)
    } else if hint.contains("dzen") {
        Some(Platform::Dzen)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_yt_dlp_output() -> &'static str {
        r#"{"id":"dQw4w9WgXcQ","title":"Rick Astley - Never Gonna Give You Up","webpage_url":"https://www.youtube.com/watch?v=dQw4w9WgXcQ","duration":213,"view_count":1000000,"uploader":"Rick Astley","extractor":"youtube","thumbnails":[{"url":"https://i.ytimg.com/vi/dQw4w9WgXcQ/hqdefault.jpg"}]}
{"id":"7195700970146909482","title":"Sample TikTok","webpage_url":"https://www.tiktok.com/@sample/video/7195700970146909482","duration":34,"view_count":2500,"channel":"Sample Creator","extractor":"tiktok"}"#
    }

    #[test]
    fn test_parse_recorded_output() {
        let results = parse_search_results(sample_yt_dlp_output().as_bytes()).unwrap();
        assert_eq!(results.len(), 2);

        let first = &results[0];
        assert_eq!(first.id, "dQw4w9WgXcQ");
        assert_eq!(first.title, "Rick Astley - Never Gonna Give You Up");
        assert_eq!(first.url, "https://www.youtube.com/watch?v=dQw4w9WgXcQ");
        assert_eq!(first.duration, Some(213));
        assert_eq!(first.uploader, Some("Rick Astley".to_string()));
        assert_eq!(first.view_count, Some(1_000_000));
        assert!(matches!(first.platform, Platform::YouTube));
        assert_eq!(
            first.thumbnail.as_deref(),
            Some("https://i.ytimg.com/vi/dQw4w9WgXcQ/hqdefault.jpg")
        );

        let second = &results[1];
        assert_eq!(second.id, "7195700970146909482");
        assert_eq!(second.platform, Platform::TikTok);
        assert_eq!(
            second.url,
            "https://www.tiktok.com/@sample/video/7195700970146909482"
        );
        assert_eq!(second.uploader, Some("Sample Creator".to_string()));
    }

    #[test]
    fn test_parse_multiple_results() {
        let json_output = r#"{"id":"video1","title":"First Video","url":"https://www.youtube.com/watch?v=video1","extractor":"youtube"}
{"id":"video2","title":"Second Video","url":"https://www.youtube.com/watch?v=video2","extractor":"youtube"}"#;

        let results = parse_search_results(json_output.as_bytes()).unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].id, "video1");
        assert_eq!(results[1].id, "video2");
    }

    #[test]
    fn test_parse_optional_fields() {
        let json_line = r#"{"id":"test123","title":"Test Video","url":"https://www.youtube.com/watch?v=test123","extractor":"youtube"}"#;

        let results = parse_search_results(json_line.as_bytes()).unwrap();
        assert_eq!(results.len(), 1);

        let result = &results[0];
        assert_eq!(result.id, "test123");
        assert_eq!(result.title, "Test Video");
        assert_eq!(result.thumbnail, None);
        assert_eq!(result.duration, None);
        assert_eq!(result.uploader, None);
        assert_eq!(result.view_count, None);
    }

    #[test]
    fn test_parse_missing_required_field() {
        let json_line =
            r#"{"title":"Missing ID Video","url":"https://www.youtube.com/watch?v=test"}"#;

        let results = parse_search_results(json_line.as_bytes()).unwrap();
        assert_eq!(results.len(), 0);

        let parse_result = parse_single_result(json_line);
        assert!(matches!(parse_result, Err(SearchError::JsonParseError(_))));
    }

    #[test]
    fn test_parse_invalid_json() {
        let invalid_json = r#"{"id":"test","title":"Test" INVALID}"#;

        let err = parse_single_result(invalid_json).unwrap_err();
        assert!(matches!(err, SearchError::JsonParseError(_)));
    }

    #[test]
    fn test_parse_empty_output() {
        let results = parse_search_results(b"").unwrap();
        assert_eq!(results.len(), 0);

        let results = parse_search_results(b"\n\n\n").unwrap();
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_parse_mixed_valid_invalid() {
        let json_output = r#"{"id":"valid1","title":"Valid Video","url":"https://www.youtube.com/watch?v=valid1","extractor":"youtube"}
INVALID JSON LINE
{"id":"valid2","title":"Another Valid","url":"https://www.youtube.com/watch?v=valid2","extractor":"youtube"}
{"title":"Missing ID"}"#;

        let results = parse_search_results(json_output.as_bytes()).unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].id, "valid1");
        assert_eq!(results[1].id, "valid2");
    }

    #[test]
    fn test_detect_platform_youtube() {
        let json = serde_json::json!({
            "extractor": "youtube",
            "id": "test",
            "title": "Test",
            "url": "https://www.youtube.com/watch?v=test"
        });

        let platform = detect_platform_from_metadata(
            json.get("extractor").and_then(|v| v.as_str()),
            None,
            json.get("url").and_then(|v| v.as_str()).unwrap(),
        );
        assert!(matches!(platform, Platform::YouTube));
    }

    #[test]
    fn test_detect_platform_from_url_fallback() {
        let platform =
            detect_platform_from_metadata(None, None, "https://twitter.com/user/status/123");
        assert!(matches!(platform, Platform::Twitter));
    }

    #[test]
    fn test_detect_platform_vk() {
        let platform = detect_platform_from_metadata(Some("vk"), None, "https://vk.com/video123");
        assert!(matches!(platform, Platform::Vk));
    }

    #[test]
    fn test_interpret_command_failure_rate_limit() {
        let stderr = "ERROR: HTTP Error 429: Too Many Requests";
        let err = interpret_command_failure(stderr, Some(1));
        assert!(matches!(err, SearchError::RateLimited(_)));
    }

    #[test]
    fn test_interpret_command_failure_generic() {
        let stderr = "ERROR: Something unexpected";
        let err = interpret_command_failure(stderr, Some(1));
        match err {
            SearchError::CommandFailed(message) => {
                assert!(message.contains("exit code 1"));
                assert!(message.contains("Something unexpected"));
            }
            _ => panic!("Expected CommandFailed variant"),
        }
    }

    #[test]
    fn test_search_service_creation() {
        let service = SearchService::new(0);
        assert_eq!(service.default_limit(), 1);

        let service = SearchService::new(10);
        assert_eq!(service.default_limit(), 10);
    }

    #[tokio::test]
    async fn test_search_empty_query() {
        let service = SearchService::new(10);
        let result = service.search("", None).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            SearchError::InvalidQuery(_) => {}
            _ => panic!("Expected InvalidQuery error"),
        }
    }

    #[tokio::test]
    async fn test_search_whitespace_query() {
        let service = SearchService::new(10);
        let result = service.search("   ", None).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            SearchError::InvalidQuery(_) => {}
            _ => panic!("Expected InvalidQuery error"),
        }
    }

    #[test]
    fn test_url_heuristic() {
        // This test verifies the logic used inside search() method
        // We can't easily test search() directly without mocking Command, 
        // so we replicate the logic here for verification.
        
        fn check_heuristic(query: &str) -> String {
            let trimmed = query.trim();
            let is_url_like = !trimmed.contains(char::is_whitespace) && trimmed.contains('.');
            if is_url_like && !trimmed.starts_with("http") {
                format!("https://{}", trimmed)
            } else {
                trimmed.to_string()
            }
        }

        assert_eq!(check_heuristic("funny cats"), "funny cats");
        assert_eq!(check_heuristic("tiktok.com/@user/video"), "https://tiktok.com/@user/video");
        assert_eq!(check_heuristic("https://youtube.com"), "https://youtube.com");
        assert_eq!(check_heuristic("example.com"), "https://example.com");
        assert_eq!(check_heuristic("word"), "word"); // No dot, so treated as search
    }

    #[test]
    fn test_parse_url_fallback() {
        let json_line = r#"{"id":"dQw4w9WgXcQ","title":"Test Video","extractor":"youtube"}"#;

        let results = parse_search_results(json_line.as_bytes()).unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].url.contains("dQw4w9WgXcQ"));
    }

    #[test]
    fn test_parse_webpage_url() {
        let json_line = r#"{"id":"test","title":"Test","webpage_url":"https://www.youtube.com/watch?v=test","extractor":"youtube"}"#;

        let results = parse_search_results(json_line.as_bytes()).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].url, "https://www.youtube.com/watch?v=test");
    }

    #[test]
    fn test_parse_thumbnail_array() {
        let json_line = r#"{"id":"test","title":"Test","url":"https://www.youtube.com/watch?v=test","thumbnails":[{"url":"https://example.com/thumb.jpg"}],"extractor":"youtube"}"#;

        let results = parse_search_results(json_line.as_bytes()).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(
            results[0].thumbnail,
            Some("https://example.com/thumb.jpg".to_string())
        );
    }

    #[test]
    fn test_parse_channel_instead_of_uploader() {
        let json_line = r#"{"id":"test","title":"Test","url":"https://www.youtube.com/watch?v=test","channel":"Channel Name","extractor":"youtube"}"#;

        let results = parse_search_results(json_line.as_bytes()).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].uploader, Some("Channel Name".to_string()));
    }
}
