use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeminiConfig {
    pub api_key: Option<String>,
    pub default_model: Option<String>,
    pub max_cost_per_analysis: Option<f64>,
    pub auto_save_reports: Option<bool>,
    pub report_directory: Option<String>,
}

impl Default for GeminiConfig {
    fn default() -> Self {
        Self {
            api_key: None,
            default_model: Some("gemini-1.5-flash".to_string()),
            max_cost_per_analysis: Some(1.0), // $1 USD default limit
            auto_save_reports: Some(true),
            report_directory: Some("./gemini_reports".to_string()),
        }
    }
}

impl GeminiConfig {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: GeminiConfig = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn load_or_default<P: AsRef<Path>>(path: P) -> Self {
        Self::load_from_file(path).unwrap_or_default()
    }
}

pub fn estimate_cost(code_length: usize, model: &str) -> f64 {
    // Rough cost estimation based on token count and model pricing
    let estimated_tokens = (code_length / 4) + 1000; // Rough estimate: 4 chars per token + prompt overhead
    
    let cost_per_1k_tokens = match model {
        "gemini-1.5-flash" => 0.00015, // $0.15 per 1M tokens
        "gemini-1.5-pro" => 0.0035,    // $3.50 per 1M tokens  
        "gemini-1.0-pro" => 0.0005,    // $0.50 per 1M tokens
        _ => 0.001, // Default estimate
    };
    
    (estimated_tokens as f64 / 1000.0) * cost_per_1k_tokens
}

pub fn format_cost_estimate(cost: f64) -> String {
    if cost < 0.01 {
        format!("< $0.01")
    } else {
        format!("~${:.2}", cost)
    }
}