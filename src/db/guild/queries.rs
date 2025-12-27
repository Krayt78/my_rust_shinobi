//! Guild-related database queries

use super::models::{Guild, GuildMember, CreateGuild};
use crate::db::DbPool;
use chrono::Utc;
use uuid::Uuid;

/// Get a guild by ID
pub async fn get_guild_by_id(
    pool: &DbPool,
    guild_id: Uuid,
) -> Result<Option<Guild>, sqlx::Error> {
    sqlx::query_as::<_, Guild>(
        r#"
        SELECT id, name, description, leader_id, level, created_at
        FROM guilds
        WHERE id = $1
        "#
    )
    .bind(guild_id)
    .fetch_optional(pool)
    .await
}

/// Get a guild by name
pub async fn get_guild_by_name(
    pool: &DbPool,
    name: &str,
) -> Result<Option<Guild>, sqlx::Error> {
    sqlx::query_as::<_, Guild>(
        r#"
        SELECT id, name, description, leader_id, level, created_at
        FROM guilds
        WHERE LOWER(name) = LOWER($1)
        "#
    )
    .bind(name)
    .fetch_optional(pool)
    .await
}

/// Get all guilds
pub async fn get_all_guilds(pool: &DbPool) -> Result<Vec<Guild>, sqlx::Error> {
    sqlx::query_as::<_, Guild>(
        r#"
        SELECT id, name, description, leader_id, level, created_at
        FROM guilds
        ORDER BY level DESC, name
        "#
    )
    .fetch_all(pool)
    .await
}

/// Create a new guild
pub async fn create_guild(
    pool: &DbPool,
    data: &CreateGuild,
) -> Result<Guild, sqlx::Error> {
    let now = Utc::now();
    let id = Uuid::new_v4();
    
    sqlx::query_as::<_, Guild>(
        r#"
        INSERT INTO guilds (id, name, description, leader_id, level, created_at)
        VALUES ($1, $2, $3, $4, 1, $5)
        RETURNING id, name, description, leader_id, level, created_at
        "#
    )
    .bind(id)
    .bind(&data.name)
    .bind(&data.description)
    .bind(data.leader_id)
    .bind(now)
    .fetch_one(pool)
    .await
}

/// Get all members of a guild
pub async fn get_guild_members(
    pool: &DbPool,
    guild_id: Uuid,
) -> Result<Vec<GuildMember>, sqlx::Error> {
    sqlx::query_as::<_, GuildMember>(
        r#"
        SELECT id, guild_id, character_id, rank, joined_at
        FROM guild_members
        WHERE guild_id = $1
        ORDER BY 
            CASE rank 
                WHEN 'leader' THEN 0 
                WHEN 'officer' THEN 1 
                ELSE 2 
            END,
            joined_at
        "#
    )
    .bind(guild_id)
    .fetch_all(pool)
    .await
}

/// Get character's guild membership
pub async fn get_character_guild(
    pool: &DbPool,
    character_id: Uuid,
) -> Result<Option<GuildMember>, sqlx::Error> {
    sqlx::query_as::<_, GuildMember>(
        r#"
        SELECT id, guild_id, character_id, rank, joined_at
        FROM guild_members
        WHERE character_id = $1
        "#
    )
    .bind(character_id)
    .fetch_optional(pool)
    .await
}

/// Join a guild
pub async fn join_guild(
    pool: &DbPool,
    guild_id: Uuid,
    character_id: Uuid,
    rank: &str,
) -> Result<GuildMember, sqlx::Error> {
    let now = Utc::now();
    
    sqlx::query_as::<_, GuildMember>(
        r#"
        INSERT INTO guild_members (id, guild_id, character_id, rank, joined_at)
        VALUES (gen_random_uuid(), $1, $2, $3, $4)
        RETURNING id, guild_id, character_id, rank, joined_at
        "#
    )
    .bind(guild_id)
    .bind(character_id)
    .bind(rank)
    .bind(now)
    .fetch_one(pool)
    .await
}

/// Leave a guild
pub async fn leave_guild(
    pool: &DbPool,
    character_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"DELETE FROM guild_members WHERE character_id = $1"#
    )
    .bind(character_id)
    .execute(pool)
    .await?;
    
    Ok(())
}

/// Check if guild name is taken
pub async fn is_guild_name_taken(
    pool: &DbPool,
    name: &str,
) -> Result<bool, sqlx::Error> {
    let count: (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM guilds WHERE LOWER(name) = LOWER($1)"#
    )
    .bind(name)
    .fetch_one(pool)
    .await?;
    
    Ok(count.0 > 0)
}

