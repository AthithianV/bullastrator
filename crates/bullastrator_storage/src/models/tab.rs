use serde::{Deserialize, Serialize};
use sqlx::FromRow;

mod json_string {
    use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};
    use serde_json::Value;

    pub fn serialize<S>(value: &String, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match serde_json::from_str::<Value>(value) {
            Ok(json) => json.serialize(serializer),
            Err(_) => value.serialize(serializer),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        match value {
            Value::String(value) => Ok(value),
            value => serde_json::to_string(&value).map_err(D::Error::custom),
        }
    }
}

mod json_string_option {
    use serde::{Deserialize, Deserializer, Serializer, de::Error};
    use serde_json::Value;

    pub fn serialize<S>(value: &Option<String>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(value) => match serde_json::from_str::<Value>(value) {
                Ok(json) => serde::Serialize::serialize(&json, serializer),
                Err(_) => serde::Serialize::serialize(value, serializer),
            },
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Option::<Value>::deserialize(deserializer)?;
        value
            .map(|value| match value {
                Value::String(value) => Ok(value),
                value => serde_json::to_string(&value).map_err(D::Error::custom),
            })
            .transpose()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Tab {
    pub id: String,
    pub workspace_id: String,
    pub connection_id: Option<String>,
    pub user_id: Option<String>,
    pub title: String,
    #[serde(with = "json_string")]
    pub params: String,
    pub is_active: bool,
    pub is_dirty: bool,
    pub is_pinned: bool,
    pub is_preview: bool,
    pub rank: i32,

    pub connection_color: Option<String>,
    pub connection_label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTab {
    pub id: String,
    pub connection_id: Option<String>,
    pub user_id: Option<String>,
    pub title: String,
    #[serde(with = "json_string")]
    pub params: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTab {
    pub title: Option<String>,
    #[serde(default, with = "json_string_option")]
    pub params: Option<String>,
    pub rank: Option<i32>,
    pub is_active: Option<bool>,
    pub is_dirty: Option<bool>,
    pub is_pinned: Option<bool>,
    pub is_preview: Option<bool>,
}
