// api0-types/src/upload.rs — config and reference-data upload payloads.

use serde::{Deserialize, Serialize};

/// Body of `POST /api/upload` on both the gateway and the store.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct UploadRequest {
    pub email: String,
    pub file_name: String,
    /// Base64-encoded file content.
    pub file_content: String,
}

/// The gateway's `POST /api/upload` result — what SDK callers see.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UploadResponse {
    pub success: bool,
    pub message: String,
    pub groups_added: Option<usize>,
    pub endpoints_added: Option<usize>,
}

/// The store's own upload result. Shaped differently from [`UploadResponse`]:
/// the store reports raw import counts, the gateway reports what it added.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StoreUploadResponse {
    pub success: bool,
    pub message: String,
    pub imported_count: i32,
    pub group_count: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReferenceData {
    pub id: String,
    pub email: String,
    pub name: String,
    pub data: serde_json::Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct UploadReferenceDataRequest {
    pub email: String,
    pub file_name: String,
    /// Base64-encoded file content.
    pub file_content: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UploadReferenceDataResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<ReferenceData>,
}
