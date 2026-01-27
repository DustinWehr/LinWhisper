//! Error types for WhisperTray

use thiserror::Error;

/// Main error type for WhisperTray
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Audio error: {0}")]
    Audio(String),

    #[error("Transcription error: {0}")]
    Transcription(String),

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Mode not found: {0}")]
    ModeNotFound(String),

    #[error("Provider error: {0}")]
    Provider(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Keyring error: {0}")]
    Keyring(String),

    #[error("Clipboard error: {0}")]
    Clipboard(String),

    #[error("Tauri error: {0}")]
    Tauri(String),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Recording already in progress")]
    RecordingInProgress,

    #[error("No recording in progress")]
    NoRecordingInProgress,

    #[error("Model not loaded")]
    ModelNotLoaded,

    #[error("Operation cancelled")]
    Cancelled,
}

impl From<AppError> for String {
    fn from(error: AppError) -> Self {
        error.to_string()
    }
}

impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

pub type Result<T> = std::result::Result<T, AppError>;

/// Convert cpal errors
impl From<cpal::BuildStreamError> for AppError {
    fn from(err: cpal::BuildStreamError) -> Self {
        AppError::Audio(err.to_string())
    }
}

impl From<cpal::PlayStreamError> for AppError {
    fn from(err: cpal::PlayStreamError) -> Self {
        AppError::Audio(err.to_string())
    }
}

impl From<cpal::DevicesError> for AppError {
    fn from(err: cpal::DevicesError) -> Self {
        AppError::Audio(err.to_string())
    }
}

impl From<cpal::DeviceNameError> for AppError {
    fn from(err: cpal::DeviceNameError) -> Self {
        AppError::Audio(err.to_string())
    }
}

impl From<cpal::DefaultStreamConfigError> for AppError {
    fn from(err: cpal::DefaultStreamConfigError) -> Self {
        AppError::Audio(err.to_string())
    }
}

impl From<hound::Error> for AppError {
    fn from(err: hound::Error) -> Self {
        AppError::Audio(format!("WAV error: {}", err))
    }
}

impl From<tauri::Error> for AppError {
    fn from(err: tauri::Error) -> Self {
        AppError::Tauri(err.to_string())
    }
}
