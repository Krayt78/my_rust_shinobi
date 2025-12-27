//! Player-related database models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Player account - linked to wallet address
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Player {
    pub id: Uuid,
    pub wallet_address: String,
    pub username: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
}

/// Data for creating a new player
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePlayer {
    pub wallet_address: String,
    pub username: Option<String>,
}

