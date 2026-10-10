use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedisUrlRequest {
    pub redis_url: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedisVersionRequest {
    pub host: String,
    pub port: i32,
    pub username: Option<String>,
    pub password: Option<String>,
    pub db: i32,
    pub is_tls_enabled: bool,
}
