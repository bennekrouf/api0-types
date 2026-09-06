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
    ToolContent, ToolsListResult, MCP_PROTOCOL_VERSION,
};
pub use upload::{
    ReferenceData, StoreUploadResponse, UploadReferenceDataRequest, UploadReferenceDataResponse,
    UploadRequest, UploadResponse,
};
pub use usage::{
    ApiUsageLog, ConfirmPaymentRequest, CreatePaymentIntentRequest, CreditTransaction,
    LogApiUsageRequest, LogApiUsageResponse, TokenUsage, UpdateCreditRequest,
};
