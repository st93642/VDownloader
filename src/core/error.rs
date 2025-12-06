use thiserror::Error;

#[derive(Error, Debug)]
pub enum DownloadError {
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Download failed: {0}")]
    DownloadFailed(String),

    #[error("Platform not supported: {0}")]
    UnsupportedPlatform(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Video extraction error: {0}")]
    ExtractionError(String),
}

pub type Result<T> = std::result::Result<T, DownloadError>;
