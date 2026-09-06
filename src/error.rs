// api0-types/src/error.rs
//
// The uniform error envelope shared by the gateway, the store and the SDK.
//
// Contract — every api0 HTTP service MUST obey both halves:
//   1. The HTTP status code reflects the outcome. A failure is never 2xx.
//   2. A failure body is exactly `{"success": false, "error": "<message>", "code": "<code>"}`.
//
// `error` stays a plain string so existing clients that read `data.error` keep
// working; `code` is the machine-readable discriminant clients should branch on.

use serde::{Deserialize, Serialize};

/// Machine-readable failure discriminant.
///
/// Unknown values deserialize to [`ErrorCode::Unknown`] so a client built against
/// an older version of this crate never fails to parse a newer service's error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    /// Missing or invalid credentials.
    Unauthorized,
    /// Authenticated, but not permitted to perform this operation.
    Forbidden,
    /// The addressed resource does not exist.
    NotFound,
    /// Malformed or semantically invalid request payload.
    InvalidRequest,
    /// The request conflicts with current state (duplicate name, etc.).
    Conflict,
    /// Rate limit exceeded — the caller should back off and retry.
    RateLimited,
    /// The tenant does not have enough credits for this operation.
    InsufficientCredits,
    /// A downstream service the request depends on failed.
    UpstreamError,
    /// The store service failed or was unreachable.
    StoreError,
    /// OAuth: the authorization code is invalid, expired or already used.
    InvalidGrant,
    /// OAuth: the requested grant type is not supported.
    UnsupportedGrantType,
    /// Unhandled server-side failure.
    Internal,
    /// A code this client does not know about.
    #[serde(other)]
    Unknown,
}

impl ErrorCode {
    /// The HTTP status a service must return alongside this code.
    pub fn http_status(self) -> u16 {
        match self {
            ErrorCode::Unauthorized => 401,
            ErrorCode::Forbidden => 403,
            ErrorCode::NotFound => 404,
            ErrorCode::InvalidRequest
            | ErrorCode::InvalidGrant
            | ErrorCode::UnsupportedGrantType => 400,
            ErrorCode::Conflict => 409,
            ErrorCode::RateLimited => 429,
            ErrorCode::InsufficientCredits => 402,
            ErrorCode::UpstreamError | ErrorCode::StoreError => 502,
            ErrorCode::Internal | ErrorCode::Unknown => 500,
        }
    }

    /// Whether retrying the identical request could plausibly succeed.
    ///
    /// Auth failures are deliberately excluded: the store throttles repeated
    /// failed key validations, so a retrying client locks itself out.
    pub fn is_retryable(self) -> bool {
        matches!(
            self,
            ErrorCode::RateLimited
                | ErrorCode::UpstreamError
                | ErrorCode::StoreError
                | ErrorCode::Internal
        )
    }
}

/// The body of every failed api0 response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiError {
    /// Always `false`. Present so clients can branch on a single field.
    #[serde(default)]
    pub success: bool,
    /// Human-readable message. Safe to surface to end users.
    pub error: String,
    /// Machine-readable discriminant.
    #[serde(default = "unknown_code")]
    pub code: ErrorCode,
}

fn unknown_code() -> ErrorCode {
    ErrorCode::Unknown
}

impl ApiError {
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            success: false,
            error: message.into(),
            code,
        }
    }

    /// The HTTP status this error must be sent with.
    pub fn http_status(&self) -> u16 {
        self.code.http_status()
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::Unauthorized, message)
    }
    pub fn forbidden(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::Forbidden, message)
    }
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::NotFound, message)
    }
    pub fn invalid_request(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::InvalidRequest, message)
    }
    pub fn conflict(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::Conflict, message)
    }
    pub fn rate_limited(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::RateLimited, message)
    }
    pub fn insufficient_credits(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::InsufficientCredits, message)
    }
    pub fn upstream(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::UpstreamError, message)
    }
    pub fn store(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::StoreError, message)
    }
    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::Internal, message)
    }
    pub fn invalid_grant(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::InvalidGrant, message)
    }
    pub fn unsupported_grant_type(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::UnsupportedGrantType, message)
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {}", self.code, self.error)
    }
}

impl std::error::Error for ApiError {}

/// Marks a successful body that carries no payload of its own.
///
/// Serialises to `{"success": true}` — the shape existing dashboard code
/// already checks for.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Success {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl Default for Success {
    fn default() -> Self {
        Self {
            success: true,
            message: None,
        }
    }
}

impl Success {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_message(message: impl Into<String>) -> Self {
        Self {
            success: true,
            message: Some(message.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_serialises_to_the_documented_shape() {
        let json = serde_json::to_value(ApiError::not_found("no such tenant")).unwrap();
        assert_eq!(json["success"], serde_json::json!(false));
        assert_eq!(json["error"], serde_json::json!("no such tenant"));
        assert_eq!(json["code"], serde_json::json!("not_found"));
    }

    #[test]
    fn unknown_codes_do_not_break_older_clients() {
        let body = r#"{"success":false,"error":"boom","code":"some_future_code"}"#;
        let parsed: ApiError = serde_json::from_str(body).unwrap();
        assert_eq!(parsed.code, ErrorCode::Unknown);
        assert_eq!(parsed.http_status(), 500);
    }

    #[test]
    fn legacy_bodies_without_a_code_still_parse() {
        let parsed: ApiError =
            serde_json::from_str(r#"{"success":false,"error":"legacy"}"#).unwrap();
        assert_eq!(parsed.code, ErrorCode::Unknown);
    }

    #[test]
    fn auth_failures_are_never_retried() {
        assert!(!ErrorCode::Unauthorized.is_retryable());
        assert!(!ErrorCode::Forbidden.is_retryable());
        assert!(ErrorCode::RateLimited.is_retryable());
    }

    #[test]
    fn status_codes_match_the_contract() {
        assert_eq!(ErrorCode::InsufficientCredits.http_status(), 402);
        assert_eq!(ErrorCode::InvalidGrant.http_status(), 400);
        assert_eq!(ErrorCode::StoreError.http_status(), 502);
    }
}
