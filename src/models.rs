use serde::{Serialize, Deserialize};


// Users table
#[derive(Debug, Serialize, Deserialize)]
#[derive(sqlx::FromRow)]
pub struct User {
    pub user_id: String,
    pub email: String,
    pub username: String,
    pub password_hash: String,
}
