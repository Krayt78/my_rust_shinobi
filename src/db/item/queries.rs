//! Item-related database queries

use super::models::{Item, InventoryItem};
use crate::db::DbPool;
use uuid::Uuid;

/// Get an item by ID
pub async fn get_item_by_id(
    pool: &DbPool,
    item_id: Uuid,
) -> Result<Option<Item>, sqlx::Error> {
    sqlx::query_as::<_, Item>(
        r#"
        SELECT id, name, description, item_type, rarity, base_price, stats
        FROM items
        WHERE id = $1
        "#
    )
    .bind(item_id)
    .fetch_optional(pool)
    .await
}

/// Get all items of a certain type
pub async fn get_items_by_type(
    pool: &DbPool,
    item_type: &str,
) -> Result<Vec<Item>, sqlx::Error> {
    sqlx::query_as::<_, Item>(
        r#"
        SELECT id, name, description, item_type, rarity, base_price, stats
        FROM items
        WHERE item_type = $1
        ORDER BY base_price, name
        "#
    )
    .bind(item_type)
    .fetch_all(pool)
    .await
}

/// Get character's inventory
pub async fn get_character_inventory(
    pool: &DbPool,
    character_id: Uuid,
) -> Result<Vec<InventoryItem>, sqlx::Error> {
    sqlx::query_as::<_, InventoryItem>(
        r#"
        SELECT id, character_id, item_id, quantity, equipped, slot
        FROM inventory
        WHERE character_id = $1
        ORDER BY equipped DESC, slot, item_id
        "#
    )
    .bind(character_id)
    .fetch_all(pool)
    .await
}

/// Get equipped items for a character
pub async fn get_equipped_items(
    pool: &DbPool,
    character_id: Uuid,
) -> Result<Vec<InventoryItem>, sqlx::Error> {
    sqlx::query_as::<_, InventoryItem>(
        r#"
        SELECT id, character_id, item_id, quantity, equipped, slot
        FROM inventory
        WHERE character_id = $1 AND equipped = true
        ORDER BY slot
        "#
    )
    .bind(character_id)
    .fetch_all(pool)
    .await
}

/// Add item to character's inventory
pub async fn add_item_to_inventory(
    pool: &DbPool,
    character_id: Uuid,
    item_id: Uuid,
    quantity: i32,
) -> Result<InventoryItem, sqlx::Error> {
    sqlx::query_as::<_, InventoryItem>(
        r#"
        INSERT INTO inventory (id, character_id, item_id, quantity, equipped)
        VALUES (gen_random_uuid(), $1, $2, $3, false)
        ON CONFLICT (character_id, item_id, slot) 
        WHERE slot IS NULL
        DO UPDATE SET quantity = inventory.quantity + $3
        RETURNING id, character_id, item_id, quantity, equipped, slot
        "#
    )
    .bind(character_id)
    .bind(item_id)
    .bind(quantity)
    .fetch_one(pool)
    .await
}

/// Remove item from character's inventory
pub async fn remove_item_from_inventory(
    pool: &DbPool,
    character_id: Uuid,
    item_id: Uuid,
    quantity: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE inventory 
        SET quantity = quantity - $3
        WHERE character_id = $1 AND item_id = $2 AND equipped = false
        "#
    )
    .bind(character_id)
    .bind(item_id)
    .bind(quantity)
    .execute(pool)
    .await?;
    
    // Clean up zero quantity items
    sqlx::query(
        r#"DELETE FROM inventory WHERE character_id = $1 AND quantity <= 0"#
    )
    .bind(character_id)
    .execute(pool)
    .await?;
    
    Ok(())
}

