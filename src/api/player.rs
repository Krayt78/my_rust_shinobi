//! Player-related server functions

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// Player information returned to the client
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PlayerInfo {
    pub id: String,
    pub wallet_address: String,
    pub username: Option<String>,
    pub is_new: bool,
}

/// Server function to authenticate/register a player by wallet address
/// 
/// This function:
/// 1. Checks if a player with this wallet address exists
/// 2. If not, creates a new player
/// 3. Updates the last login time
/// 4. Returns player information
#[server(AuthenticatePlayer, "/api")]
pub async fn authenticate_player(wallet_address: String) -> Result<PlayerInfo, ServerFnError> {
    use crate::db::{get_player_by_wallet, create_player, CreatePlayer, DbPool};
    use axum::Extension;
    use leptos_axum::extract;
    
    // Get the database pool from Axum extensions
    let Extension(pool): Extension<DbPool> = extract().await?;
    
    // Check if player already exists
    let existing_player = get_player_by_wallet(&pool, &wallet_address)
        .await
        .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;
    
    match existing_player {
        Some(player) => {
            // Player exists - update last login and return info
            leptos::logging::log!("Player {} logged in with wallet {}", player.id, wallet_address);
            
            Ok(PlayerInfo {
                id: player.id.to_string(),
                wallet_address: player.wallet_address,
                username: player.username,
                is_new: false,
            })
        }
        None => {
            // New player - create account
            leptos::logging::log!("Creating new player for wallet {}", wallet_address);
            
            let new_player = create_player(&pool, &CreatePlayer {
                wallet_address: wallet_address.clone(),
                username: None,
            })
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to create player: {}", e)))?;
            
            Ok(PlayerInfo {
                id: new_player.id.to_string(),
                wallet_address: new_player.wallet_address,
                username: new_player.username,
                is_new: true,
            })
        }
    }
}

/// Server function to update player's username
#[server(UpdateUsername, "/api")]
pub async fn update_username(player_id: String, username: String) -> Result<(), ServerFnError> {
    use crate::db::DbPool;
    use axum::Extension;
    use leptos_axum::extract;
    use sqlx::query;
    
    // Validate username
    if username.len() < 3 || username.len() > 32 {
        return Err(ServerFnError::new("Username must be between 3 and 32 characters"));
    }
    
    let Extension(pool): Extension<DbPool> = extract().await?;
    
    let player_uuid = uuid::Uuid::parse_str(&player_id)
        .map_err(|_| ServerFnError::new("Invalid player ID"))?;
    
    // Check if username is already taken
    let existing: Option<(i64,)> = sqlx::query_as(
        "SELECT COUNT(*) FROM players WHERE LOWER(username) = LOWER($1) AND id != $2"
    )
    .bind(&username)
    .bind(player_uuid)
    .fetch_optional(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;
    
    if let Some((count,)) = existing {
        if count > 0 {
            return Err(ServerFnError::new("Username is already taken"));
        }
    }
    
    // Update username
    query("UPDATE players SET username = $1, updated_at = NOW() WHERE id = $2")
        .bind(&username)
        .bind(player_uuid)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to update username: {}", e)))?;
    
    Ok(())
}

