//! Item-related database models

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

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

/// Data for creating a new item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateItem {
    pub name: String,
    pub description: Option<String>,
    pub item_type: String,
    pub rarity: String,
    pub base_price: i64,
    pub stats: Option<serde_json::Value>,
}
