use crate::db::schema::*;
use rocket_okapi::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    Queryable,
    Identifiable,
    Insertable,
    AsChangeset,
    JsonSchema,
)]
#[table_name = "users"]
#[primary_key(username)]
pub struct User {
    pub username: String,
    pub email: String,
    pub password: String,
    pub is_admin: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PartialUser {
    pub username: String,
    pub email: String,
    pub is_admin: bool,
}

impl From<User> for PartialUser {
    fn from(user: User) -> Self {
        Self {
            username: user.username,
            email: user.email,
            is_admin: user.is_admin,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UserCredentials {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct NewPassword {
    pub current: String,
    pub new: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ApiKey {
    pub token: String,
}
