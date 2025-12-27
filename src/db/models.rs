//! Database models for the game
//! 
//! These structs represent the database tables and are used
//! for queries with sqlx.

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

/// Character - the ninja character a player controls
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Character {
    pub id: Uuid,
    pub player_id: Uuid,
    pub name: String,
    pub level: i32,
    pub experience: i64,
    pub health: i32,
    pub max_health: i32,
    pub chakra: i32,
    pub max_chakra: i32,
    pub strength: i32,
    pub agility: i32,
    pub intelligence: i32,
    pub village: Option<String>,
    pub rank: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Item in the game (weapons, scrolls, consumables, etc.)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Item {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub item_type: String,
    pub rarity: String,
    pub base_price: i64,
    pub stats: Option<sqlx::types::Json<serde_json::Value>>,
}

/// Player inventory entry
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct InventoryItem {
    pub id: Uuid,
    pub character_id: Uuid,
    pub item_id: Uuid,
    pub quantity: i32,
    pub equipped: bool,
    pub slot: Option<String>,
}

/// Skill/Jutsu that characters can learn
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Skill {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub skill_type: String,
    pub element: Option<String>,
    pub chakra_cost: i32,
    pub cooldown_seconds: i32,
    pub base_damage: Option<i32>,
    pub required_level: i32,
}

/// Character's learned skills
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CharacterSkill {
    pub id: Uuid,
    pub character_id: Uuid,
    pub skill_id: Uuid,
    pub skill_level: i32,
    pub experience: i64,
}

/// Guild/Clan in the game
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Guild {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub leader_id: Uuid,
    pub village: Option<String>,
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

// ============================================================================
// DTOs for creating new records
// ============================================================================

/// Data for creating a new player
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePlayer {
    pub wallet_address: String,
    pub username: Option<String>,
}

/// Data for creating a new character
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCharacter {
    pub player_id: Uuid,
    pub name: String,
    pub village: Option<String>,
}

/// Default starting stats for a new character
impl Default for CreateCharacter {
    fn default() -> Self {
        Self {
            player_id: Uuid::nil(),
            name: String::new(),
            village: None,
        }
    }
}

