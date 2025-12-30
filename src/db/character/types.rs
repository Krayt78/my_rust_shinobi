use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
pub enum CharacterClass {
    Adventurer,
    Warrior,
    Mage,
    Rogue,
    Cleric,
}