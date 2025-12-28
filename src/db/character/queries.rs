//! Character-related database queries

use super::models::{ActionCooldown, Character, CreateCharacter, PlayerLocation};
use crate::db::DbPool;
use chrono::Utc;
use uuid::Uuid;

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
               mana, max_mana, strength, dexterity, intelligence,
               constitution, wisdom, charisma, gold, action_points, max_action_points,
               character_class, created_at, updated_at
        FROM characters
        WHERE player_id = $1
        ORDER BY created_at DESC
        "#,
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
               mana, max_mana, strength, dexterity, intelligence,
               constitution, wisdom, charisma, gold, action_points, max_action_points,
               character_class, created_at, updated_at
        FROM characters
        WHERE id = $1
        "#,
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

    // Default starting stats for a new adventurer
    let starting_health = 100;
    let starting_mana = 50;
    let starting_stat = 10;
    let starting_gold: i64 = 100;
    let starting_ap = 10;

    sqlx::query_as::<_, Character>(
        r#"
        INSERT INTO characters (
            id, player_id, name, level, experience,
            health, max_health, mana, max_mana,
            strength, dexterity, intelligence, constitution, wisdom, charisma,
            gold, action_points, max_action_points,
            character_class, created_at, updated_at
        )
        VALUES ($1, $2, $3, 1, 0, $4, $4, $5, $5, $6, $6, $6, $6, $6, $6, $7, $8, $8, $9, $10, $10)
        RETURNING id, player_id, name, level, experience, health, max_health,
                  mana, max_mana, strength, dexterity, intelligence,
                  constitution, wisdom, charisma, gold, action_points, max_action_points,
                  character_class, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(data.player_id)
    .bind(&data.name)
    .bind(starting_health)
    .bind(starting_mana)
    .bind(starting_stat)
    .bind(starting_gold)
    .bind(starting_ap)
    .bind(&data.character_class)
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
    max_mana: i32,
    strength: i32,
    dexterity: i32,
    intelligence: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE characters
        SET level = $1, experience = $2, max_health = $3, health = $3,
            max_mana = $4, mana = $4, strength = $5, dexterity = $6,
            intelligence = $7, updated_at = $8
        WHERE id = $9
        "#,
    )
    .bind(level)
    .bind(experience)
    .bind(max_health)
    .bind(max_mana)
    .bind(strength)
    .bind(dexterity)
    .bind(intelligence)
    .bind(Utc::now())
    .bind(character_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Check if a character name is already taken
pub async fn is_character_name_taken(pool: &DbPool, name: &str) -> Result<bool, sqlx::Error> {
    let count: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*) FROM characters WHERE LOWER(name) = LOWER($1)
        "#,
    )
    .bind(name)
    .fetch_one(pool)
    .await?;

    Ok(count.0 > 0)
}

// ============================================================================
// Player Location Tracking
// ============================================================================

/// Get character's current location
pub async fn get_player_location(
    pool: &DbPool,
    character_id: Uuid,
) -> Result<Option<PlayerLocation>, sqlx::Error> {
    sqlx::query_as::<_, PlayerLocation>(
        r#"
        SELECT id, character_id, town_id, location_id, entered_at
        FROM player_locations
        WHERE character_id = $1
        "#,
    )
    .bind(character_id)
    .fetch_optional(pool)
    .await
}

/// Set character's location (upsert)
pub async fn set_player_location(
    pool: &DbPool,
    character_id: Uuid,
    town_id: Uuid,
    location_id: Option<Uuid>,
) -> Result<PlayerLocation, sqlx::Error> {
    let now = Utc::now();

    sqlx::query_as::<_, PlayerLocation>(
        r#"
        INSERT INTO player_locations (id, character_id, town_id, location_id, entered_at)
        VALUES (gen_random_uuid(), $1, $2, $3, $4)
        ON CONFLICT (character_id) DO UPDATE 
        SET town_id = $2, location_id = $3, entered_at = $4
        RETURNING id, character_id, town_id, location_id, entered_at
        "#,
    )
    .bind(character_id)
    .bind(town_id)
    .bind(location_id)
    .bind(now)
    .fetch_one(pool)
    .await
}

// ============================================================================
// Action Cooldowns
// ============================================================================

/// Set a cooldown for an action
pub async fn set_action_cooldown(
    pool: &DbPool,
    character_id: Uuid,
    action_id: Uuid,
    cooldown_seconds: i32,
) -> Result<ActionCooldown, sqlx::Error> {
    let available_at = Utc::now() + chrono::Duration::seconds(cooldown_seconds as i64);

    sqlx::query_as::<_, ActionCooldown>(
        r#"
        INSERT INTO action_cooldowns (id, character_id, action_id, available_at)
        VALUES (gen_random_uuid(), $1, $2, $3)
        ON CONFLICT (character_id, action_id) DO UPDATE 
        SET available_at = $3
        RETURNING id, character_id, action_id, available_at
        "#,
    )
    .bind(character_id)
    .bind(action_id)
    .bind(available_at)
    .fetch_one(pool)
    .await
}

/// Check if an action is on cooldown
pub async fn is_action_on_cooldown(
    pool: &DbPool,
    character_id: Uuid,
    action_id: Uuid,
) -> Result<bool, sqlx::Error> {
    let result: Option<(chrono::DateTime<Utc>,)> = sqlx::query_as(
        r#"
        SELECT available_at FROM action_cooldowns
        WHERE character_id = $1 AND action_id = $2 AND available_at > NOW()
        "#,
    )
    .bind(character_id)
    .bind(action_id)
    .fetch_optional(pool)
    .await?;

    Ok(result.is_some())
}

/// Clean up expired cooldowns
pub async fn cleanup_expired_cooldowns(pool: &DbPool) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(r#"DELETE FROM action_cooldowns WHERE available_at <= NOW()"#)
        .execute(pool)
        .await?;

    Ok(result.rows_affected())
}
