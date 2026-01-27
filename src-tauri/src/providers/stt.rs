//! Speech-to-Text provider implementations

use crate::error::{AppError, Result};
use crate::modes::SttProvider as SttProviderType;
use async_trait::async_trait;
use std::path::PathBuf;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

/// STT provider trait
#[async_trait]
pub trait SttProvider: Send + Sync {
    /// Transcribe audio samples to text
    async fn transcribe(&self, samples: &[f32], language: Option<&str>) -> Result<String>;

    /// Get the provider name
    fn name(&self) -> &str;
}

/// Local whisper.cpp provider
pub struct WhisperCppProvider {
    model_path: PathBuf,
}

impl WhisperCppProvider {
    /// Create a new whisper.cpp provider
    pub fn new(model_path: PathBuf) -> Self {
        Self { model_path }
    }
}

#[async_trait]
impl SttProvider for WhisperCppProvider {
    async fn transcribe(&self, samples: &[f32], language: Option<&str>) -> Result<String> {
        let model_path = self.model_path.clone();
        let samples = samples.to_vec();
        let language = language.map(|s| s.to_string());

        let result = tokio::task::spawn_blocking(move || {
            // Create context for transcription
            let params = WhisperContextParameters::default();
            let ctx = WhisperContext::new_with_params(model_path.to_str().unwrap(), params)
                .map_err(|e| AppError::Transcription(format!("Failed to create context: {}", e)))?;

            let mut state = ctx
                .create_state()
                .map_err(|e| AppError::Transcription(format!("Failed to create state: {}", e)))?;

            let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });

            // Set language if specified
            if let Some(lang) = language.as_deref() {
                params.set_language(Some(lang));
            } else {
                params.set_language(Some("en"));
            }

            // Disable timestamps for cleaner output
            params.set_print_special(false);
            params.set_print_progress(false);
            params.set_print_realtime(false);
            params.set_print_timestamps(false);

            // Run transcription
            state
                .full(params, &samples)
                .map_err(|e| AppError::Transcription(format!("Transcription failed: {}", e)))?;

            // Collect segments
            let num_segments = state.full_n_segments().map_err(|e| {
                AppError::Transcription(format!("Failed to get segments: {}", e))
            })?;

            let mut text = String::new();
            for i in 0..num_segments {
                if let Ok(segment) = state.full_get_segment_text(i) {
                    text.push_str(&segment);
                }
            }

            Ok::<String, AppError>(text.trim().to_string())
        })
        .await
        .map_err(|e| AppError::Transcription(format!("Task failed: {}", e)))??;

        Ok(result)
    }

    fn name(&self) -> &str {
        "whisper.cpp"
    }
}

/// Get the default models directory
pub fn get_models_dir() -> Result<PathBuf> {
    let data_dir = directories::ProjectDirs::from("com", "whispertray", "WhisperTray")
        .ok_or_else(|| AppError::Config("Could not determine data directory".to_string()))?
        .data_dir()
        .to_path_buf();

    Ok(data_dir.join("models"))
}

/// Get the path to a specific model
pub fn get_model_path(model_name: &str) -> Result<PathBuf> {
    let models_dir = get_models_dir()?;
    Ok(models_dir.join(format!("ggml-{}.bin", model_name)))
}

/// Download a whisper model if not present
pub async fn ensure_model(model_name: &str) -> Result<PathBuf> {
    let model_path = get_model_path(model_name)?;

    if model_path.exists() {
        log::info!("Model already exists: {:?}", model_path);
        return Ok(model_path);
    }

    // Create models directory
    let models_dir = get_models_dir()?;
    tokio::fs::create_dir_all(&models_dir).await?;

    // Download model
    let url = format!(
        "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-{}.bin",
        model_name
    );

    log::info!("Downloading model from: {}", url);

    let response = reqwest::get(&url).await?;

    if !response.status().is_success() {
        return Err(AppError::Transcription(format!(
            "Failed to download model: HTTP {}",
            response.status()
        )));
    }

    let bytes = response.bytes().await?;
    tokio::fs::write(&model_path, &bytes).await?;

    log::info!("Model downloaded successfully: {:?}", model_path);
    Ok(model_path)
}

/// Create an STT provider based on configuration
pub async fn create_stt_provider(
    provider_type: &SttProviderType,
    model: &str,
) -> Result<Box<dyn SttProvider>> {
    match provider_type {
        SttProviderType::WhisperCpp => {
            let model_path = ensure_model(model).await?;
            let provider = WhisperCppProvider::new(model_path);
            Ok(Box::new(provider))
        }
        SttProviderType::Deepgram => {
            Err(AppError::Provider("Deepgram not yet implemented".to_string()))
        }
        SttProviderType::OpenAI => {
            Err(AppError::Provider("OpenAI STT not yet implemented".to_string()))
        }
        SttProviderType::Custom(name) => {
            Err(AppError::Provider(format!("Unknown provider: {}", name)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_model_path() {
        let path = get_model_path("base.en").unwrap();
        assert!(path.to_str().unwrap().contains("ggml-base.en.bin"));
    }
}
