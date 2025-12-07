/*****************************************************************************/
/*                                                                           */
/*  error.rs                                             TTTTTTTT SSSSSSS II */
/*                                                          TT    SS      II */
/*  By: st93642@students.tsi.lv                             TT    SSSSSSS II */
/*                                                          TT         SS II */
/*  Created: Dec 07 2025 13:36 st93642                      TT    SSSSSSS II */
/*  Updated: Dec 07 2025 13:36 st93642                                       */
/*                                                                           */
/*   Transport and Telecommunication Institute - Riga, Latvia                */
/*                       https://tsi.lv                                      */
/*****************************************************************************/

use thiserror::Error;

#[derive(Error, Debug, Clone)]
#[allow(dead_code)]
pub enum DownloadError {
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Download failed: {0}")]
    DownloadFailed(String),

    #[error("Unsupported platform: {0}")]
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
