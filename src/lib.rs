//! Shared wire types for the api0 platform.
//!
//! One definition per payload, used by the gateway, the store and the `api0`
//! SDK, so the three cannot drift. The crate is deliberately dependency-light:
//! serde, serde_json, chrono and uuid only — no web framework, no database
//! driver — so any api0 component can depend on it.
//!
//! [`ApiError`] defines the platform-wide failure contract; see its module for
//! the two rules every api0 service obeys.

pub mod catalog;
pub mod error;
pub mod keys;
pub mod mcp;
pub mod upload;
pub mod usage;

pub use catalog::{
    generate_uuid, AddApiGroupRequest, ApiGroup, ApiGroupWithEndpoints, ApiStorage, Endpoint,
    Parameter, UpdateApiGroupRequest, UpdatePreferenceRequest, UserPreferences,
};
pub use error::{ApiError, ErrorCode, Success};
pub use keys::{
    ApiKeyInfo, GeneratedKey, GenerateKeyRequest, KeyPreference, PlatformRole, ProviderInfo,
    SetClientIdRequest, SetRoleRequest, Tenant, TenantUser, UserRole, ValidateKeyRequest,
    ValidateKeyResponse,
};
pub use mcp::{
    JsonRpcError, JsonRpcRequest, JsonRpcResponse, McpTool, ToolCallParams, ToolCallResult,
    ToolContent, ToolsListResult, MCP_PROTOCOL_VERSION, PROTOCOL_VERSION_HEADER,
    SUPPORTED_PROTOCOL_VERSIONS,
};
pub use upload::{
    ReferenceData, StoreUploadResponse, UploadReferenceDataRequest, UploadReferenceDataResponse,
    UploadRequest, UploadResponse,
};
pub use usage::{
    ApiUsageLog, ConfirmPaymentRequest, CreatePaymentIntentRequest, CreditTransaction,
    LogApiUsageRequest, LogApiUsageResponse, TokenUsage, UpdateCreditRequest,
};

#[cfg(test)]
mod tests {
    use super::*;

    /// Every wire type implements `Default`, so consumers can build one with
    /// `..Default::default()` and a field added here does not break them.
    /// A new type missing from this list, or without `Default`, should fail review.
    #[test]
    fn every_wire_type_has_a_default() {
        fn has_default<T: Default>() {}
        has_default::<ApiError>();
        has_default::<ErrorCode>();
        has_default::<Success>();
        has_default::<Parameter>();
        has_default::<Endpoint>();
        has_default::<ApiGroup>();
        has_default::<ApiGroupWithEndpoints>();
        has_default::<ApiStorage>();
        has_default::<AddApiGroupRequest>();
        has_default::<UpdateApiGroupRequest>();
        has_default::<UserPreferences>();
        has_default::<UpdatePreferenceRequest>();
        has_default::<Tenant>();
        has_default::<TenantUser>();
        has_default::<ApiKeyInfo>();
        has_default::<KeyPreference>();
        has_default::<GenerateKeyRequest>();
        has_default::<GeneratedKey>();
        has_default::<ValidateKeyRequest>();
        has_default::<ValidateKeyResponse>();
        has_default::<ProviderInfo>();
        has_default::<SetClientIdRequest>();
        has_default::<PlatformRole>();
        has_default::<UserRole>();
        has_default::<SetRoleRequest>();
        has_default::<UploadRequest>();
        has_default::<UploadResponse>();
        has_default::<StoreUploadResponse>();
        has_default::<ReferenceData>();
        has_default::<UploadReferenceDataRequest>();
        has_default::<UploadReferenceDataResponse>();
        has_default::<JsonRpcRequest>();
        has_default::<JsonRpcResponse>();
        has_default::<JsonRpcError>();
        has_default::<McpTool>();
        has_default::<ToolsListResult>();
        has_default::<ToolCallParams>();
        has_default::<ToolContent>();
        has_default::<ToolCallResult>();
        has_default::<TokenUsage>();
        has_default::<LogApiUsageRequest>();
        has_default::<LogApiUsageResponse>();
        has_default::<ApiUsageLog>();
        has_default::<UpdateCreditRequest>();
        has_default::<CreditTransaction>();
        has_default::<CreatePaymentIntentRequest>();
        has_default::<ConfirmPaymentRequest>();
    }
}
