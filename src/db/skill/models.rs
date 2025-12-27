//! Skill-related database models

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

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

/// Data for creating a new skill
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSkill {
    pub name: String,
    pub description: Option<String>,
    pub skill_type: String,
    pub element: Option<String>,
    pub mana_cost: i32,
    pub cooldown_seconds: i32,
    pub base_damage: Option<i32>,
    pub required_level: i32,
}

