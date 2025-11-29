use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Clone)]
pub struct Config {
    pub model: String,
    pub endpoint: String,
    pub safe_mode: bool,
    pub cache_enabled: bool,
    pub copy_to_clipboard: bool,
    cache_path: PathBuf,
}

#[derive(Serialize, Deserialize, Default)]
struct CacheFile {
    entries: Vec<CacheEntry>,
}

#[derive(Serialize, Deserialize)]
struct CacheEntry {
    prompt: String,
    command: String,
}

impl Config {
    pub fn new(safe_mode: bool, cache_enabled: bool, copy_to_clipboard: bool) -> Self {
        let model = std::env::var("QWEN_MODEL").unwrap_or_else(|_| "qwen2.5-coder:7b".to_string());
        let endpoint =
            std::env::var("OLLAMA_ENDPOINT").unwrap_or_else(|_| "http://localhost:11434/api/chat".to_string());

        let cache_path = Self::default_cache_path();

        Self {
            model,
            endpoint,
            safe_mode,
            cache_enabled,
            copy_to_clipboard,
            cache_path,
        }
    }

    fn default_cache_path() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let mut path = PathBuf::from(home);
        path.push(".config");
        path.push("qwen_cli_assistant");
        path.push("cache.json");
        path
    }

    pub fn load_cached(&self, prompt: &str) -> Result<Option<String>> {
        if !self.cache_path.exists() {
            return Ok(None);
        }

        let data = fs::read_to_string(&self.cache_path)
            .with_context(|| format!("Failed to read cache file at {:?}", self.cache_path))?;

        let cache: CacheFile = serde_json::from_str(&data).unwrap_or_default();
        for entry in cache.entries {
            if entry.prompt == prompt {
                return Ok(Some(entry.command));
            }
        }

        Ok(None)
    }

    pub fn save_cached(&self, prompt: &str, command: &str) -> Result<()> {
        let mut cache = if self.cache_path.exists() {
            let data = fs::read_to_string(&self.cache_path).unwrap_or_default();
            serde_json::from_str::<CacheFile>(&data).unwrap_or_default()
        } else {
            CacheFile::default()
        };

        cache.entries.push(CacheEntry {
            prompt: prompt.to_string(),
            command: command.to_string(),
        });

        if let Some(parent) = self.cache_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let serialized = serde_json::to_string_pretty(&cache)?;
        fs::write(&self.cache_path, serialized)?;

        Ok(())
    }
}
