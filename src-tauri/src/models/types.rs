use serde::{Deserialize, Serialize};

/// Pricing information for a model (values are USD costs expressed as strings).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelPricing {
    /// Price per input token in USD.
    pub prompt: String,
    /// Price per output token in USD.
    pub completion: String,
    /// Flat price per request in USD.
    #[serde(default)]
    pub request: String,
    /// Price per image-related unit in USD.
    #[serde(default)]
    pub image: String,
    /// Price per output image-related unit in USD.
    #[serde(default)]
    pub image_output: String,
    /// Price per web search
    #[serde(default)]
    pub web_search: String,
    /// Price per internal reasoning token
    #[serde(default)]
    pub internal_reasoning: String,
    /// Price per cached prompt token read in USD.
    #[serde(default)]
    pub input_cache_read: String,
    /// Price per cached prompt token write in USD.
    #[serde(default)]
    pub input_cache_write: String,
}

/// Cost calculation result for a single request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestCost {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64,
    /// Cost for prompt tokens
    pub prompt_cost: f64,
    /// Cost for completion tokens
    pub completion_cost: f64,
    /// Total cost in USD
    pub total_cost: f64,
}
