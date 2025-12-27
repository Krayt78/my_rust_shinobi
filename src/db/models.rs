//! Database models for the game
//! 
//! These structs represent the database tables and are used
//! for queries with sqlx.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ============================================================================
// PLAYER & CHARACTER
// ============================================================================

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
    pub character_class: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ============================================================================
// ITEMS & INVENTORY
// ============================================================================

/// Item in the game (weapons, armor, consumables, etc.)
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

// ============================================================================
// SKILLS
// ============================================================================

/// Skill/Spell that characters can learn
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Skill {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub skill_type: String,
    pub element: Option<String>,
    pub mana_cost: i32,
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

// ============================================================================
// GUILDS
// ============================================================================

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

// ============================================================================
// TOWNS & LOCATIONS
// ============================================================================

/// Type of location within a town
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "location_type", rename_all = "snake_case")]
pub enum LocationType {
    Shop,
    Training,
    Service,
    Social,
    Quest,
    Crafting,
    Combat,
    Travel,
    Special,
}

/// Type of action that can be performed
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "action_type", rename_all = "snake_case")]
pub enum ActionType {
    Instant,
    Timed,
    Dialog,
    Navigation,
    Combat,
    Shop,
}

/// Category of action (for UI styling/filtering)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "action_category", rename_all = "snake_case")]
pub enum ActionCategory {
    Combat,
    Magic,
    Melee,
    Ranged,
    Heal,
    Rest,
    Shop,
    Craft,
    Social,
    Mission,
    Travel,
    Knowledge,
}

/// A town or region in the game world
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Town {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub region: String,
    pub required_level: i32,
    pub map_image: Option<String>,
    pub is_safe_zone: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// A specific location within a town (Tavern, Guild Hall, etc.)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Location {
    pub id: Uuid,
    pub town_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub icon: String,
    pub location_type: LocationType,
    pub map_position_x: f32,
    pub map_position_y: f32,
    pub required_level: i32,
    pub required_quest_id: Option<Uuid>,
    pub is_active: bool,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// An action that can be performed at a location
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LocationAction {
    pub id: Uuid,
    pub location_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub icon: String,
    pub action_type: ActionType,
    pub category: ActionCategory,
    // Requirements
    pub required_level: i32,
    pub required_gold: i64,
    pub required_item_id: Option<Uuid>,
    pub required_item_quantity: i32,
    pub action_points_cost: i32,
    // Timing
    pub cooldown_seconds: i32,
    pub duration_seconds: i32,
    // Outcomes
    pub rewards: Option<sqlx::types::Json<ActionRewards>>,
    pub is_repeatable: bool,
    pub is_active: bool,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ============================================================================
// ACTION REWARDS (JSON structure)
// ============================================================================

/// Flexible reward structure stored as JSON in the database
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ActionRewards {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gold: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experience: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ItemReward>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stat_changes: Option<StatChanges>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlocks: Option<Vec<UnlockReward>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teleport_to: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemReward {
    pub item_id: Uuid,
    pub quantity: i32,
    #[serde(default = "default_chance")]
    pub chance: f32,  // 0.0 - 1.0, for random drops
}

fn default_chance() -> f32 {
    1.0
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StatChanges {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mana: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dexterity: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intelligence: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constitution: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wisdom: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charisma: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlockReward {
    pub unlock_type: String,  // "location", "action", "quest", "skill"
    pub target_id: Uuid,
}

// ============================================================================
// PLAYER STATE TRACKING
// ============================================================================

/// Tracks where a player/character currently is
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PlayerLocation {
    pub id: Uuid,
    pub character_id: Uuid,
    pub town_id: Uuid,
    pub location_id: Option<Uuid>,
    pub entered_at: DateTime<Utc>,
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
    pub character_class: Option<String>,
}

impl Default for CreateCharacter {
    fn default() -> Self {
        Self {
            player_id: Uuid::nil(),
            name: String::new(),
            character_class: None,
        }
    }
}

/// Data for creating a new town
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTown {
    pub name: String,
    pub description: Option<String>,
    pub region: String,
    pub required_level: i32,
    pub map_image: Option<String>,
    pub is_safe_zone: bool,
}

/// Data for creating a new location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLocation {
    pub town_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub icon: String,
    pub location_type: LocationType,
    pub map_position_x: f32,
    pub map_position_y: f32,
    pub required_level: i32,
}

/// Data for creating a new action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLocationAction {
    pub location_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub icon: String,
    pub action_type: ActionType,
    pub category: ActionCategory,
    pub required_level: i32,
    pub required_gold: i64,
    pub action_points_cost: i32,
    pub cooldown_seconds: i32,
    pub duration_seconds: i32,
    pub rewards: Option<ActionRewards>,
    pub is_repeatable: bool,
}
