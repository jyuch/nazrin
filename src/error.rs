use thiserror::Error;
use windows::Win32::Foundation::WIN32_ERROR;

#[cfg(windows)]
#[derive(Debug, Error)]
pub enum UnleashError {
    #[error("Win32Error: {0:?}")]
    Win32Error(WIN32_ERROR),
}
