//! Location-related database queries (Towns, Locations, Actions)

use super::models::{Town, Location, LocationAction};
use crate::db::DbPool;
use uuid::Uuid;

// ============================================================================
// Town Queries
// ============================================================================

/// Get all towns
pub async fn get_all_towns(pool: &DbPool) -> Result<Vec<Town>, sqlx::Error> {
    sqlx::query_as::<_, Town>(
        r#"
        SELECT id, name, description, region, required_level, map_image, 
               is_safe_zone, created_at, updated_at
        FROM towns
        ORDER BY required_level, name
        "#
    )
    .fetch_all(pool)
    .await
}

/// Get a town by ID
pub async fn get_town_by_id(
    pool: &DbPool,
    town_id: Uuid,
) -> Result<Option<Town>, sqlx::Error> {
    sqlx::query_as::<_, Town>(
        r#"
        SELECT id, name, description, region, required_level, map_image, 
               is_safe_zone, created_at, updated_at
        FROM towns
        WHERE id = $1
        "#
    )
    .bind(town_id)
    .fetch_optional(pool)
    .await
}

/// Get starting town (first town in starting_zone)
pub async fn get_starting_town(pool: &DbPool) -> Result<Option<Town>, sqlx::Error> {
    sqlx::query_as::<_, Town>(
        r#"
        SELECT id, name, description, region, required_level, map_image, 
               is_safe_zone, created_at, updated_at
        FROM towns
        WHERE region = 'starting_zone'
        ORDER BY required_level
        LIMIT 1
        "#
    )
    .fetch_optional(pool)
    .await
}

/// Get towns by region
pub async fn get_towns_by_region(
    pool: &DbPool,
    region: &str,
) -> Result<Vec<Town>, sqlx::Error> {
    sqlx::query_as::<_, Town>(
        r#"
        SELECT id, name, description, region, required_level, map_image, 
               is_safe_zone, created_at, updated_at
        FROM towns
        WHERE region = $1
        ORDER BY required_level, name
        "#
    )
    .bind(region)
    .fetch_all(pool)
    .await
}

// ============================================================================
// Location Queries
// ============================================================================

/// Get all locations for a town
pub async fn get_locations_by_town(
    pool: &DbPool,
    town_id: Uuid,
) -> Result<Vec<Location>, sqlx::Error> {
    sqlx::query_as::<_, Location>(
        r#"
        SELECT id, town_id, name, description, icon, location_type,
               map_position_x, map_position_y, required_level, required_quest_id,
               is_active, sort_order, created_at, updated_at
        FROM locations
        WHERE town_id = $1 AND is_active = true
        ORDER BY sort_order, name
        "#
    )
    .bind(town_id)
    .fetch_all(pool)
    .await
}

/// Get a location by ID
pub async fn get_location_by_id(
    pool: &DbPool,
    location_id: Uuid,
) -> Result<Option<Location>, sqlx::Error> {
    sqlx::query_as::<_, Location>(
        r#"
        SELECT id, town_id, name, description, icon, location_type,
               map_position_x, map_position_y, required_level, required_quest_id,
               is_active, sort_order, created_at, updated_at
        FROM locations
        WHERE id = $1
        "#
    )
    .bind(location_id)
    .fetch_optional(pool)
    .await
}

/// Get all active locations (across all towns)
pub async fn get_all_active_locations(pool: &DbPool) -> Result<Vec<Location>, sqlx::Error> {
    sqlx::query_as::<_, Location>(
        r#"
        SELECT id, town_id, name, description, icon, location_type,
               map_position_x, map_position_y, required_level, required_quest_id,
               is_active, sort_order, created_at, updated_at
        FROM locations
        WHERE is_active = true
        ORDER BY town_id, sort_order, name
        "#
    )
    .fetch_all(pool)
    .await
}

// ============================================================================
// Location Action Queries
// ============================================================================

/// Get all actions for a location
pub async fn get_actions_by_location(
    pool: &DbPool,
    location_id: Uuid,
) -> Result<Vec<LocationAction>, sqlx::Error> {
    sqlx::query_as::<_, LocationAction>(
        r#"
        SELECT id, location_id, name, description, icon, action_type, category,
               required_level, required_gold, required_item_id, required_item_quantity,
               action_points_cost, cooldown_seconds, duration_seconds, rewards,
               is_repeatable, is_active, sort_order, created_at, updated_at
        FROM location_actions
        WHERE location_id = $1 AND is_active = true
        ORDER BY sort_order, name
        "#
    )
    .bind(location_id)
    .fetch_all(pool)
    .await
}

/// Get available actions for a character at a location (respecting level and cooldowns)
pub async fn get_available_actions(
    pool: &DbPool,
    location_id: Uuid,
    character_id: Uuid,
    character_level: i32,
) -> Result<Vec<LocationAction>, sqlx::Error> {
    sqlx::query_as::<_, LocationAction>(
        r#"
        SELECT a.id, a.location_id, a.name, a.description, a.icon, 
               a.action_type, a.category, a.required_level, a.required_gold,
               a.required_item_id, a.required_item_quantity, a.action_points_cost,
               a.cooldown_seconds, a.duration_seconds, a.rewards,
               a.is_repeatable, a.is_active, a.sort_order, a.created_at, a.updated_at
        FROM location_actions a
        LEFT JOIN action_cooldowns c 
            ON c.action_id = a.id AND c.character_id = $2
        WHERE a.location_id = $1 
          AND a.is_active = true
          AND a.required_level <= $3
          AND (c.available_at IS NULL OR c.available_at <= NOW())
        ORDER BY a.sort_order, a.name
        "#
    )
    .bind(location_id)
    .bind(character_id)
    .bind(character_level)
    .fetch_all(pool)
    .await
}

/// Get a single action by ID
pub async fn get_action_by_id(
    pool: &DbPool,
    action_id: Uuid,
) -> Result<Option<LocationAction>, sqlx::Error> {
    sqlx::query_as::<_, LocationAction>(
        r#"
        SELECT id, location_id, name, description, icon, action_type, category,
               required_level, required_gold, required_item_id, required_item_quantity,
               action_points_cost, cooldown_seconds, duration_seconds, rewards,
               is_repeatable, is_active, sort_order, created_at, updated_at
        FROM location_actions
        WHERE id = $1
        "#
    )
    .bind(action_id)
    .fetch_optional(pool)
    .await
}

