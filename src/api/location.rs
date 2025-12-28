//! Location-related server functions

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// Location information returned to the client
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct LocationInfo {
    pub id: String,
    pub town_id: String,
    pub name: String,
    pub description: Option<String>,
    pub icon: String,
    pub location_type: String,
    pub map_position_x: f32,
    pub map_position_y: f32,
}

/// Town information returned to the client
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TownInfo {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub region: String,
    pub required_level: i32,
    pub map_image: Option<String>,
    pub is_safe_zone: bool,
}

/// Server function to get a location by ID
#[server(GetLocationById, "/api")]
pub async fn get_location_by_id(location_id: String) -> Result<Option<LocationInfo>, ServerFnError> {
    use crate::db::location::get_location_by_id as db_get_location_by_id;
    use crate::db::DbPool;
    use axum::Extension;
    use leptos_axum::extract;
    use uuid::Uuid;

    // Get the database pool from Axum extensions
    let Extension(pool): Extension<DbPool> = extract().await?;

    // Parse the UUID string
    let location_uuid = Uuid::parse_str(&location_id)
        .map_err(|_| ServerFnError::new("Invalid location ID format"))?;

    // Query the database
    let location = db_get_location_by_id(&pool, location_uuid)
        .await
        .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;

    // Convert to client-friendly format
    Ok(location.map(|loc| LocationInfo {
        id: loc.id.to_string(),
        town_id: loc.town_id.to_string(),
        name: loc.name,
        description: loc.description,
        icon: loc.icon,
        location_type: format!("{:?}", loc.location_type),
        map_position_x: loc.map_position_x,
        map_position_y: loc.map_position_y,
    }))
}

/// Server function to get a town by ID
#[server(GetTownById, "/api")]
pub async fn get_town_by_id(town_id: String) -> Result<Option<TownInfo>, ServerFnError> {
    use crate::db::location::get_town_by_id as db_get_town_by_id;
    use crate::db::DbPool;
    use axum::Extension;
    use leptos_axum::extract;
    use uuid::Uuid;

    // Get the database pool from Axum extensions
    let Extension(pool): Extension<DbPool> = extract().await?;

    // Parse the UUID string
    let town_uuid = Uuid::parse_str(&town_id)
        .map_err(|_| ServerFnError::new("Invalid town ID format"))?;

    // Query the database
    let town = db_get_town_by_id(&pool, town_uuid)
        .await
        .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;

    // Convert to client-friendly format
    Ok(town.map(|t| TownInfo {
        id: t.id.to_string(),
        name: t.name,
        description: t.description,
        region: t.region,
        required_level: t.required_level,
        map_image: t.map_image,
        is_safe_zone: t.is_safe_zone,
    }))
}

/// Server function to get all locations for a town
#[server(GetLocationsByTown, "/api")]
pub async fn get_locations_by_town(town_id: String) -> Result<Vec<LocationInfo>, ServerFnError> {
    use crate::db::location::get_locations_by_town as db_get_locations_by_town;
    use crate::db::DbPool;
    use axum::Extension;
    use leptos_axum::extract;
    use uuid::Uuid;

    // Get the database pool from Axum extensions
    let Extension(pool): Extension<DbPool> = extract().await?;

    // Parse the UUID string
    let town_uuid = Uuid::parse_str(&town_id)
        .map_err(|_| ServerFnError::new("Invalid town ID format"))?;

    // Query the database
    let locations = db_get_locations_by_town(&pool, town_uuid)
        .await
        .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;

    // Convert to client-friendly format
    Ok(locations.into_iter().map(|loc| LocationInfo {
        id: loc.id.to_string(),
        town_id: loc.town_id.to_string(),
        name: loc.name,
        description: loc.description,
        icon: loc.icon,
        location_type: format!("{:?}", loc.location_type),
        map_position_x: loc.map_position_x,
        map_position_y: loc.map_position_y,
    }).collect())
}

/// Action information returned to the client
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ActionInfo {
    pub id: String,
    pub location_id: String,
    pub name: String,
    pub description: Option<String>,
    pub icon: String,
    pub action_type: String,
    pub category: String,
    pub required_level: i32,
    pub required_gold: i64,
    pub action_points_cost: i32,
    pub cooldown_seconds: i32,
    pub duration_seconds: i32,
    pub is_repeatable: bool,
}

/// Server function to get all actions for a location
#[server(GetActionsByLocation, "/api")]
pub async fn get_actions_by_location(location_id: String) -> Result<Vec<ActionInfo>, ServerFnError> {
    use crate::db::location::get_actions_by_location as db_get_actions_by_location;
    use crate::db::DbPool;
    use axum::Extension;
    use leptos_axum::extract;
    use uuid::Uuid;

    // Get the database pool from Axum extensions
    let Extension(pool): Extension<DbPool> = extract().await?;

    // Parse the UUID string
    let location_uuid = Uuid::parse_str(&location_id)
        .map_err(|_| ServerFnError::new("Invalid location ID format"))?;

    // Query the database
    let actions = db_get_actions_by_location(&pool, location_uuid)
        .await
        .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;

    // Convert to client-friendly format
    Ok(actions.into_iter().map(|action| ActionInfo {
        id: action.id.to_string(),
        location_id: action.location_id.to_string(),
        name: action.name,
        description: action.description,
        icon: action.icon,
        action_type: format!("{:?}", action.action_type),
        category: format!("{:?}", action.category),
        required_level: action.required_level,
        required_gold: action.required_gold,
        action_points_cost: action.action_points_cost,
        cooldown_seconds: action.cooldown_seconds,
        duration_seconds: action.duration_seconds,
        is_repeatable: action.is_repeatable,
    }).collect())
}
