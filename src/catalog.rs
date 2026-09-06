// api0-types/src/catalog.rs — the tenant's API catalog: groups, endpoints, parameters.

use serde::{Deserialize, Deserializer, Serialize};

pub fn generate_uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}

fn default_verb() -> String {
    "GET".to_string()
}

fn default_false_string() -> String {
    "false".to_string()
}

/// Accepts either a JSON boolean or the strings "true"/"false", normalising to
/// a lowercase string. Uploaded OpenAPI configs use both spellings.
fn deserialize_flexible_bool<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum FlexibleBool {
        Bool(bool),
        String(String),
    }

    match FlexibleBool::deserialize(deserializer)? {
        FlexibleBool::Bool(b) => Ok(b.to_string()),
        FlexibleBool::String(s) => match s.to_lowercase().as_str() {
            "true" | "false" => Ok(s.to_lowercase()),
            _ => Err(Error::custom(
                "Invalid boolean string, must be 'true' or 'false'",
            )),
        },
    }
}

#[derive(Debug, Serialize, PartialEq, Deserialize, Clone)]
pub struct Parameter {
    pub name: String,
    #[serde(default = "String::new")]
    pub description: String,
    #[serde(
        default = "default_false_string",
        deserialize_with = "deserialize_flexible_bool"
    )]
    pub required: String,
    #[serde(default)]
    pub alternatives: Vec<String>,
}

#[derive(Debug, Serialize, PartialEq, Deserialize, Clone)]
pub struct Endpoint {
    #[serde(default = "String::new")] // Allow empty, will be auto-generated
    pub id: String,
    pub text: String,
    #[serde(default = "String::new")]
    pub description: String,
    #[serde(default)]
    pub parameters: Vec<Parameter>,
    #[serde(default = "default_verb")]
    #[serde(alias = "method")]
    pub verb: String,
    #[serde(default = "String::new")] // Allow empty, will inherit from group
    pub base: String,
    #[serde(default = "String::new")]
    pub path: String,
    #[serde(default = "String::new")]
    pub suggested_sentence: String,
    #[serde(default = "String::new")] // Allow empty, will be set by parent group
    pub group_id: String,
}

#[derive(Debug, Serialize, PartialEq, Deserialize, Clone)]
pub struct ApiGroup {
    #[serde(default = "generate_uuid")]
    pub id: String,
    pub name: String,
    #[serde(default = "String::new")]
    pub description: String,
    #[serde(default = "String::new")]
    pub base: String,
    #[serde(default = "String::new")]
    pub tenant_id: String,
}

#[derive(Debug, Serialize, PartialEq, Deserialize, Clone)]
pub struct ApiGroupWithEndpoints {
    #[serde(flatten)]
    pub group: ApiGroup,
    pub endpoints: Vec<Endpoint>,
}

#[derive(Debug, Serialize, PartialEq, Deserialize, Clone)]
pub struct ApiStorage {
    pub api_groups: Vec<ApiGroupWithEndpoints>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AddApiGroupRequest {
    pub email: String,
    pub api_group: ApiGroupWithEndpoints,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct UpdateApiGroupRequest {
    pub email: String,
    pub group_id: String,
    pub api_group: ApiGroupWithEndpoints,
}

#[derive(Debug, Serialize, PartialEq, Deserialize, Clone)]
pub struct UserPreferences {
    pub email: String,
    /// Ids of default endpoints the user has hidden.
    pub hidden_defaults: Vec<String>,
    pub default_tenant_id: Option<String>,
}

#[derive(Debug, Serialize, PartialEq, Deserialize, Clone)]
pub struct UpdatePreferenceRequest {
    pub email: String,
    /// "hide_default" or "show_default"
    pub action: String,
    pub endpoint_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn required_accepts_bool_and_string() {
        let from_bool: Parameter =
            serde_json::from_str(r#"{"name":"id","required":true}"#).unwrap();
        let from_string: Parameter =
            serde_json::from_str(r#"{"name":"id","required":"TRUE"}"#).unwrap();
        assert_eq!(from_bool.required, "true");
        assert_eq!(from_string.required, "true");
    }

    #[test]
    fn endpoint_accepts_method_as_an_alias_for_verb() {
        let e: Endpoint = serde_json::from_str(r#"{"text":"t","method":"POST"}"#).unwrap();
        assert_eq!(e.verb, "POST");
    }

    #[test]
    fn group_is_flattened_into_its_endpoints_wrapper() {
        let json = serde_json::to_value(ApiGroupWithEndpoints {
            group: ApiGroup {
                id: "g1".into(),
                name: "Billing".into(),
                description: String::new(),
                base: String::new(),
                tenant_id: "t1".into(),
            },
            endpoints: vec![],
        })
        .unwrap();
        // Flattened: `name` sits at the top level, not under `group`.
        assert_eq!(json["name"], serde_json::json!("Billing"));
        assert!(json.get("group").is_none());
    }
}
