use crate::entity::user_entity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Plan {
    Free,
    Pro,
    Teams,
    Enterprise,
}

impl From<String> for Plan {
    fn from(s: String) -> Self {
        match s.as_str() {
            "PRO" => Plan::Pro,
            "TEAMS" => Plan::Teams,
            "ENTERPRISE" => Plan::Enterprise,
            _ => Plan::Free,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReadUserModel {
    pub id: String,
    pub name: String,
    pub email: String,
    pub image: Option<String>,
    pub plan: String,
    pub max_members: i32,
    pub max_workspaces: i32,
}

impl From<user_entity::Model> for ReadUserModel {
    fn from(entity: user_entity::Model) -> Self {
        Self {
            id: entity.id,
            name: entity.name,
            email: entity.email,
            image: entity.image,
            plan: entity.plan,
            max_members: entity.max_members,
            max_workspaces: entity.max_workspaces,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserModel {
    pub name: String,
    pub email: String,
    pub image: Option<String>,
    pub plan: String,
    pub max_members: i32,
    pub max_workspaces: i32,
}

impl CreateUserModel {
    pub fn from_read(u: &ReadUserModel) -> Self {
        Self {
            name: u.name.clone(),
            email: u.email.clone(),
            image: u.image.clone(),
            plan: u.plan.clone(),
            max_members: u.max_members,
            max_workspaces: u.max_workspaces,
        }
    }
}
