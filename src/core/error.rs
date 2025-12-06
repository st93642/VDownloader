use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum DownloadError {
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Download failed: {0}")]
    DownloadFailed(String),

    #[error("Platform not supported: {0}")]
    UnsupportedPlatform(String),

    #[error("IO error: {0}")]
    IoError(String),

    #[error("Video extraction error: {0}")]
    ExtractionError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Video not found or unavailable")]
    VideoNotFound,

    #[error("Output directory does not exist or is not writable")]
    InvalidOutputDirectory,

    #[error("Cancelled by user")]
    Cancelled,
}

pub type Result<T> = std::result::Result<T, DownloadError>;
