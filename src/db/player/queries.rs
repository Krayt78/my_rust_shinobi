//! Player-related database queries

use super::models::{CreatePlayer, Player};
use crate::db::DbPool;
use chrono::Utc;
use uuid::Uuid;

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
        "#,
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
        "#,
    )
    .bind(player_id)
    .fetch_optional(pool)
    .await
}

/// Create a new player (or get existing one by wallet)
pub async fn create_player(pool: &DbPool, data: &CreatePlayer) -> Result<Player, sqlx::Error> {
    let now = Utc::now();
    let id = Uuid::new_v4();

    sqlx::query_as::<_, Player>(
        r#"
        INSERT INTO players (id, wallet_address, username, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $4)
        ON CONFLICT (wallet_address) DO UPDATE SET last_login = $4
        RETURNING id, wallet_address, username, created_at, updated_at, last_login
        "#,
    )
    .bind(id)
    .bind(&data.wallet_address)
    .bind(&data.username)
    .bind(now)
    .fetch_one(pool)
    .await
}

/// Update player's last login time
pub async fn update_player_login(pool: &DbPool, player_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE players SET last_login = $1, updated_at = $1
        WHERE id = $2
        "#,
    )
    .bind(Utc::now())
    .bind(player_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Update player's username
pub async fn update_player_username(
    pool: &DbPool,
    player_id: Uuid,
    username: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE players SET username = $1, updated_at = $2
        WHERE id = $3
        "#,
    )
    .bind(username)
    .bind(Utc::now())
    .bind(player_id)
    .execute(pool)
    .await?;

    Ok(())
}
