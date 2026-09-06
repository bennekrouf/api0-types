// api0-types/src/keys.rs — API keys, key validation, tenants and platform roles.

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tenant {
    pub id: String,
    pub name: String,
    pub credit_balance: i64,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TenantUser {
    pub tenant_id: String,
    pub email: String,
    /// "owner" or "member".
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiKeyInfo {
    pub id: String,
    pub key_prefix: String,
    pub key_name: String,
    pub generated_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_used: Option<String>,
    pub usage_count: i64,
}

/// The `/api/user/key/<tenant_id>` payload: keys plus the tenant's balance.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KeyPreference {
    pub has_keys: bool,
    pub active_key_count: usize,
    pub keys: Vec<ApiKeyInfo>,
    pub balance: i64,
    pub tenant_id: String,
    pub tenant_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenerateKeyRequest {
    pub email: String,
    pub key_name: String,
    /// Tenant to associate the key with. Defaults to the user's personal tenant.
    #[serde(default)]
    pub tenant_id: Option<String>,
    /// When set, the key is a consumer key scoped to this provider tenant.
    #[serde(default)]
    pub provider_tenant_id: Option<String>,
}

/// A freshly minted key. The raw `key` is returned exactly once.
///
/// `keyPrefix` is camelCase on the wire — that is the shape the dashboard and
/// the OAuth token exchange already consume.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeneratedKey {
    #[serde(default)]
    pub success: bool,
    #[serde(default)]
    pub message: Option<String>,
    pub key: String,
    #[serde(rename = "keyPrefix")]
    pub key_prefix: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ValidateKeyRequest {
    pub api_key: String,
    #[serde(default)]
    pub expected_tenant_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ValidateKeyResponse {
    pub valid: bool,
    pub email: Option<String>,
    pub key_id: Option<String>,
    /// The tenant that owns this API key (used for credit deduction).
    pub tenant_id: Option<String>,
    /// If set, this is a consumer key: tools come from this provider tenant,
    /// but credits are deducted from the consumer's tenant (`tenant_id` above).
    pub provider_tenant_id: Option<String>,
    pub message: String,
}

/// A provider tenant as resolved from an OAuth `client_id`
/// (`GET /api/tenant/by-client-id/<client_id>`).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProviderInfo {
    pub tenant_id: String,
    #[serde(default)]
    pub name: String,
    /// Standard Google OAuth 2.0 Web Client ID registered by the provider.
    /// When set, the authorize page uses Google Identity Services with this
    /// client_id and the gateway validates the resulting ID token against it.
    #[serde(default)]
    pub google_client_id: Option<String>,
}

/// Body of `PUT /api/user/mcp-client-id`.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetClientIdRequest {
    pub email: String,
    #[serde(default)]
    pub mcp_client_id: Option<String>,
    #[serde(default)]
    pub google_client_id: Option<String>,
}

/// Platform-level role. Users without a row are [`PlatformRole::User`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlatformRole {
    SuperAdmin,
    Admin,
    User,
}

impl PlatformRole {
    /// Whether this role may reach the admin surface.
    pub fn is_admin(self) -> bool {
        matches!(self, PlatformRole::SuperAdmin | PlatformRole::Admin)
    }

    /// Whether this role may grant or revoke other users' roles.
    pub fn is_super_admin(self) -> bool {
        matches!(self, PlatformRole::SuperAdmin)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserRole {
    pub email: String,
    pub role: PlatformRole,
    #[serde(default)]
    pub granted_by: Option<String>,
    #[serde(default)]
    pub granted_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetRoleRequest {
    pub email: String,
    pub role: PlatformRole,
    /// Injected by the gateway from the verified caller — never trusted from the body.
    pub granted_by: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roles_use_snake_case_on_the_wire() {
        assert_eq!(
            serde_json::to_value(PlatformRole::SuperAdmin).unwrap(),
            serde_json::json!("super_admin")
        );
    }

    #[test]
    fn admin_privileges_nest_correctly() {
        assert!(PlatformRole::SuperAdmin.is_admin());
        assert!(PlatformRole::Admin.is_admin());
        assert!(!PlatformRole::Admin.is_super_admin());
        assert!(!PlatformRole::User.is_admin());
    }
}
