// api0-types/src/usage.rs — per-call usage logging, credits and payments.

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, PartialEq, Deserialize, Clone)]
pub struct TokenUsage {
    pub estimated: bool,
    pub input_tokens: i64,
    pub model: String,
    pub output_tokens: i64,
    pub total_tokens: i64,
}

#[derive(Debug, Serialize, PartialEq, Deserialize, Clone)]
pub struct LogApiUsageRequest {
    pub key_id: String,
    pub email: String,
    pub endpoint_path: String,
    pub method: String,
    pub status_code: Option<i32>,
    pub response_time_ms: Option<i64>,
    pub request_size_bytes: Option<i64>,
    pub response_size_bytes: Option<i64>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub usage: Option<TokenUsage>,
    /// Opaque end-consumer identifier forwarded by the tenant via X-Consumer-Id.
    /// api0 stores it verbatim — no PII assumed, no validation performed.
    /// Null when the tenant did not pass the header.
    pub consumer_id: Option<String>,
    /// Explicit tenant attribution (optional, defaults to key owner's tenant).
    pub tenant_id: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, PartialEq, Deserialize, Clone)]
pub struct LogApiUsageResponse {
    pub success: bool,
    pub message: String,
    pub log_id: Option<String>,
}

#[derive(Debug, Serialize, PartialEq, Deserialize, Clone)]
pub struct ApiUsageLog {
    pub id: String,
    pub key_id: String,
    pub email: String,
    pub endpoint_path: String,
    pub method: String,
    pub timestamp: String,
    pub response_status: Option<i32>,
    pub response_time_ms: Option<i64>,
    pub request_size: Option<i64>,
    pub response_size: Option<i64>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub usage_estimated: Option<bool>,
    pub input_tokens: Option<i64>,
    pub output_tokens: Option<i64>,
    pub total_tokens: Option<i64>,
    pub model_used: Option<String>,
    pub metadata: Option<serde_json::Value>,
    /// Opaque end-consumer identifier — null when the tenant did not supply X-Consumer-Id.
    pub consumer_id: Option<String>,
}

#[derive(Debug, Serialize, PartialEq, Deserialize, Clone)]
pub struct UpdateCreditRequest {
    pub email: String,
    /// Explicit tenant attribution (optional, defaults to the user's personal tenant).
    pub tenant_id: Option<String>,
    pub amount: i64,
    /// e.g. "cv_generation", "cover_letter", "optimize", "translate", "cv_import",
    /// "topup", "welcome".
    #[serde(default = "default_action_type")]
    pub action_type: String,
    #[serde(default)]
    pub description: Option<String>,
}

fn default_action_type() -> String {
    "unknown".to_string()
}

#[derive(Debug, Serialize, PartialEq, Deserialize, Clone)]
pub struct CreditTransaction {
    pub id: i64,
    pub tenant_id: String,
    pub email: String,
    pub amount: i64,
    pub balance_after: i64,
    pub action_type: String,
    pub description: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, PartialEq, Deserialize, Clone)]
pub struct CreatePaymentIntentRequest {
    pub amount: i64,
    pub currency: String,
}

#[derive(Debug, Serialize, PartialEq, Deserialize, Clone)]
pub struct ConfirmPaymentRequest {
    pub payment_intent_id: String,
    pub amount: i64,
}
