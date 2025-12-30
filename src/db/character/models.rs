//! Character-related database models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use crate::db::CharacterClass;

/// Character - the adventurer a player controls
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Character {
    pub id: Uuid,
    pub player_id: Uuid,
    pub name: String,
    pub level: i32,
    pub experience: i64,
    pub health: i32,
    pub max_health: i32,
    pub mana: i32,
    pub max_mana: i32,
    pub strength: i32,
    pub dexterity: i32,
    pub intelligence: i32,
    pub constitution: i32,
    pub wisdom: i32,
    pub charisma: i32,
    pub gold: i64,
    pub action_points: i32,
    pub max_action_points: i32,
    pub character_class: CharacterClass,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub location_id: Uuid,
}

/// Data for creating a new character
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCharacter {
    pub player_id: Uuid,
    pub name: String,
}

impl Default for CreateCharacter {
    fn default() -> Self {
        Self {
            player_id: Uuid::nil(),
            name: String::new(),
        }
    }
}

/// Tracks cooldowns for location actions
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ActionCooldown {
    pub id: Uuid,
    pub character_id: Uuid,
    pub action_id: Uuid,
    pub available_at: DateTime<Utc>,
}

/// Tracks completed actions (for one-time or progress tracking)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CompletedAction {
    pub id: Uuid,
    pub character_id: Uuid,
    pub action_id: Uuid,
    pub completed_at: DateTime<Utc>,
    pub times_completed: i32,
}
