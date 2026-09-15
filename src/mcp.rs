// api0-types/src/mcp.rs — the JSON-RPC 2.0 envelope and MCP payloads served at /mcp.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The newest MCP revision api0 speaks, and what its own clients request.
pub const MCP_PROTOCOL_VERSION: &str = "2025-06-18";

/// Every revision the gateway accepts, newest first.
///
/// The gateway serves only tools over Streamable HTTP, a subset these revisions
/// agree on. 2025-06-18 removed JSON-RPC batching (never accepted here) and made
/// clients send the `MCP-Protocol-Version` header after initialize. Older
/// revisions stay so a connector set up against them keeps working.
pub const SUPPORTED_PROTOCOL_VERSIONS: &[&str] = &["2025-06-18", "2025-03-26", "2024-11-05"];

/// The HTTP header a client sends on every request after `initialize`.
pub const PROTOCOL_VERSION_HEADER: &str = "MCP-Protocol-Version";

/// Version negotiation, as the spec defines it: answer with the revision the
/// client asked for when it is supported, otherwise with the newest one, and let
/// the client decide whether it can continue.
pub fn negotiate_protocol_version(requested: Option<&str>) -> &'static str {
    requested
        .and_then(|r| SUPPORTED_PROTOCOL_VERSIONS.iter().find(|v| **v == r))
        .copied()
        .unwrap_or(MCP_PROTOCOL_VERSION)
}

pub fn is_supported_protocol_version(version: &str) -> bool {
    SUPPORTED_PROTOCOL_VERSIONS.contains(&version)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Value>,
    pub method: String,
    #[serde(default)]
    pub params: Value,
}

impl JsonRpcRequest {
    pub fn new(id: impl Into<Value>, method: impl Into<String>, params: Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: Some(id.into()),
            method: method.into(),
            params,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
}

impl JsonRpcResponse {
    pub fn ok(id: Option<Value>, result: Value) -> Self {
        Self {
            jsonrpc: "2.0".into(),
            id,
            result: Some(result),
            error: None,
        }
    }

    pub fn err(id: Option<Value>, code: i32, message: impl Into<String>) -> Self {
        Self {
            jsonrpc: "2.0".into(),
            id,
            result: None,
            error: Some(JsonRpcError {
                code,
                message: message.into(),
            }),
        }
    }
}

/// Standard JSON-RPC 2.0 error codes, as used by the MCP endpoint.
pub mod rpc_error_codes {
    pub const PARSE_ERROR: i32 = -32700;
    pub const INVALID_REQUEST: i32 = -32600;
    pub const METHOD_NOT_FOUND: i32 = -32601;
    pub const INVALID_PARAMS: i32 = -32602;
    pub const INTERNAL_ERROR: i32 = -32603;
    /// Implementation-defined server error (the -32000..=-32099 range).
    /// api0 uses it for tool-execution failures: timeouts and backend errors.
    pub const SERVER_ERROR: i32 = -32000;
}

/// One entry of a `tools/list` result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct McpTool {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(rename = "inputSchema")]
    pub input_schema: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolsListResult {
    pub tools: Vec<McpTool>,
}

/// Params of a `tools/call` request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolCallParams {
    pub name: String,
    #[serde(default)]
    pub arguments: Value,
}

/// One content block of a `tools/call` result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolContent {
    #[serde(rename = "type")]
    pub content_type: String,
    #[serde(default)]
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolCallResult {
    pub content: Vec<ToolContent>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tool_input_schema_keeps_its_camel_case_wire_name() {
        let json = serde_json::to_value(McpTool {
            name: "send_invoice".into(),
            description: String::new(),
            input_schema: serde_json::json!({"type": "object"}),
        })
        .unwrap();
        assert!(json.get("inputSchema").is_some());
        assert!(json.get("input_schema").is_none());
    }

    #[test]
    fn a_supported_version_is_echoed_back() {
        assert_eq!(negotiate_protocol_version(Some("2025-03-26")), "2025-03-26");
        assert_eq!(negotiate_protocol_version(Some("2024-11-05")), "2024-11-05");
    }

    #[test]
    fn an_unknown_or_missing_version_gets_the_newest() {
        assert_eq!(negotiate_protocol_version(Some("2099-01-01")), MCP_PROTOCOL_VERSION);
        assert_eq!(negotiate_protocol_version(None), MCP_PROTOCOL_VERSION);
    }

    #[test]
    fn the_advertised_version_is_one_that_is_accepted() {
        assert!(is_supported_protocol_version(MCP_PROTOCOL_VERSION));
        assert_eq!(SUPPORTED_PROTOCOL_VERSIONS[0], MCP_PROTOCOL_VERSION);
    }

    #[test]
    fn responses_omit_the_unused_half_of_the_envelope() {
        let json = serde_json::to_value(JsonRpcResponse::ok(
            Some(serde_json::json!(1)),
            serde_json::json!({}),
        ))
        .unwrap();
        assert!(json.get("error").is_none());
    }
}
