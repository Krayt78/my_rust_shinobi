//! Database queries for the game
//! 
//! All database operations are defined here using sqlx.

use super::{DbPool, Player, Character, CreatePlayer, CreateCharacter};
use chrono::Utc;
use uuid::Uuid;

// ============================================================================
// Player Queries
// ============================================================================

/// Get a player by their wallet address
pub async fn get_player_by_wallet(
    pool: &DbPool,
    wallet_address: &str,
) -> Result<Option<Player>, sqlx::Error> {
    sqlx::query_as::<_, Player>(
        r#"
        SELECT id, wallet_address, username, created_at, updated_at, last_login
        FROM players
        WHERE wallet_address = $1
        "#
    )
    .bind(wallet_address)
    .fetch_optional(pool)
    .await
}

/// Get a player by their ID
pub async fn get_player_by_id(
    pool: &DbPool,
    player_id: Uuid,
) -> Result<Option<Player>, sqlx::Error> {
    sqlx::query_as::<_, Player>(
        r#"
        SELECT id, wallet_address, username, created_at, updated_at, last_login
        FROM players
        WHERE id = $1
        "#
    )
    .bind(player_id)
    .fetch_optional(pool)
    .await
}

/// Create a new player (or get existing one by wallet)
pub async fn create_player(
    pool: &DbPool,
    data: &CreatePlayer,
) -> Result<Player, sqlx::Error> {
    let now = Utc::now();
    let id = Uuid::new_v4();
    
    sqlx::query_as::<_, Player>(
        r#"
        INSERT INTO players (id, wallet_address, username, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $4)
        ON CONFLICT (wallet_address) DO UPDATE SET last_login = $4
        RETURNING id, wallet_address, username, created_at, updated_at, last_login
        "#
    )
    .bind(id)
    .bind(&data.wallet_address)
    .bind(&data.username)
    .bind(now)
    .fetch_one(pool)
    .await
}

/// Update player's last login time
pub async fn update_player_login(
    pool: &DbPool,
    player_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE players SET last_login = $1, updated_at = $1
        WHERE id = $2
        "#
    )
    .bind(Utc::now())
    .bind(player_id)
    .execute(pool)
    .await?;
    
    Ok(())
}

// ============================================================================
// Character Queries
// ============================================================================

/// Get all characters for a player
pub async fn get_characters_by_player(
    pool: &DbPool,
    player_id: Uuid,
) -> Result<Vec<Character>, sqlx::Error> {
    sqlx::query_as::<_, Character>(
        r#"
        SELECT id, player_id, name, level, experience, health, max_health,
               chakra, max_chakra, strength, agility, intelligence,
               village, rank, created_at, updated_at
        FROM characters
        WHERE player_id = $1
        ORDER BY created_at DESC
        "#
    )
    .bind(player_id)
    .fetch_all(pool)
    .await
}

/// Get a character by ID
pub async fn get_character_by_id(
    pool: &DbPool,
    character_id: Uuid,
) -> Result<Option<Character>, sqlx::Error> {
    sqlx::query_as::<_, Character>(
        r#"
        SELECT id, player_id, name, level, experience, health, max_health,
               chakra, max_chakra, strength, agility, intelligence,
               village, rank, created_at, updated_at
        FROM characters
        WHERE id = $1
        "#
    )
    .bind(character_id)
    .fetch_optional(pool)
    .await
}

/// Create a new character with default starting stats
pub async fn create_character(
    pool: &DbPool,
    data: &CreateCharacter,
) -> Result<Character, sqlx::Error> {
    let now = Utc::now();
    let id = Uuid::new_v4();
    
    // Default starting stats for a new ninja
    let starting_health = 100;
    let starting_chakra = 50;
    let starting_stat = 10;
    
    sqlx::query_as::<_, Character>(
        r#"
        INSERT INTO characters (
            id, player_id, name, level, experience,
            health, max_health, chakra, max_chakra,
            strength, agility, intelligence,
            village, rank, created_at, updated_at
        )
        VALUES ($1, $2, $3, 1, 0, $4, $4, $5, $5, $6, $6, $6, $7, 'Genin', $8, $8)
        RETURNING id, player_id, name, level, experience, health, max_health,
                  chakra, max_chakra, strength, agility, intelligence,
                  village, rank, created_at, updated_at
        "#
    )
    .bind(id)
    .bind(data.player_id)
    .bind(&data.name)
    .bind(starting_health)
    .bind(starting_chakra)
    .bind(starting_stat)
    .bind(&data.village)
    .bind(now)
    .fetch_one(pool)
    .await
}

/// Update character stats after leveling up or training
pub async fn update_character_stats(
    pool: &DbPool,
    character_id: Uuid,
    level: i32,
    experience: i64,
    max_health: i32,
    max_chakra: i32,
    strength: i32,
    agility: i32,
    intelligence: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE characters
        SET level = $1, experience = $2, max_health = $3, health = $3,
            max_chakra = $4, chakra = $4, strength = $5, agility = $6,
            intelligence = $7, updated_at = $8
        WHERE id = $9
        "#
    )
    .bind(level)
    .bind(experience)
    .bind(max_health)
    .bind(max_chakra)
    .bind(strength)
    .bind(agility)
    .bind(intelligence)
    .bind(Utc::now())
    .bind(character_id)
    .execute(pool)
    .await?;
    
    Ok(())
}

/// Check if a character name is already taken
pub async fn is_character_name_taken(
    pool: &DbPool,
    name: &str,
) -> Result<bool, sqlx::Error> {
    let count: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*) FROM characters WHERE LOWER(name) = LOWER($1)
        "#
    )
    .bind(name)
    .fetch_one(pool)
    .await?;
    
    Ok(count.0 > 0)
}

