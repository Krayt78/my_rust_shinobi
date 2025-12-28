//! Guild-related database models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Guild/Clan in the game
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Guild {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub leader_id: Uuid,
    pub level: i32,
    pub created_at: DateTime<Utc>,
}

/// Guild membership
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct GuildMember {
    pub id: Uuid,
    pub guild_id: Uuid,
    pub character_id: Uuid,
    pub rank: String,
    pub joined_at: DateTime<Utc>,
}

/// Data for creating a new guild
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGuild {
    pub name: String,
    pub description: Option<String>,
    pub leader_id: Uuid,
}
