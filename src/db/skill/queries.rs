//! Skill-related database queries

use super::models::{CharacterSkill, Skill};
use crate::db::DbPool;
use uuid::Uuid;

/// Get a skill by ID
pub async fn get_skill_by_id(pool: &DbPool, skill_id: Uuid) -> Result<Option<Skill>, sqlx::Error> {
    sqlx::query_as::<_, Skill>(
        r#"
        SELECT id, name, description, skill_type, element, mana_cost,
               cooldown_seconds, base_damage, required_level
        FROM skills
        WHERE id = $1
        "#,
    )
    .bind(skill_id)
    .fetch_optional(pool)
    .await
}

/// Get all skills of a certain type
pub async fn get_skills_by_type(
    pool: &DbPool,
    skill_type: &str,
) -> Result<Vec<Skill>, sqlx::Error> {
    sqlx::query_as::<_, Skill>(
        r#"
        SELECT id, name, description, skill_type, element, mana_cost,
               cooldown_seconds, base_damage, required_level
        FROM skills
        WHERE skill_type = $1
        ORDER BY required_level, name
        "#,
    )
    .bind(skill_type)
    .fetch_all(pool)
    .await
}

/// Get all skills available at a certain level
pub async fn get_skills_for_level(pool: &DbPool, level: i32) -> Result<Vec<Skill>, sqlx::Error> {
    sqlx::query_as::<_, Skill>(
        r#"
        SELECT id, name, description, skill_type, element, mana_cost,
               cooldown_seconds, base_damage, required_level
        FROM skills
        WHERE required_level <= $1
        ORDER BY required_level, skill_type, name
        "#,
    )
    .bind(level)
    .fetch_all(pool)
    .await
}

/// Get character's learned skills
pub async fn get_character_skills(
    pool: &DbPool,
    character_id: Uuid,
) -> Result<Vec<CharacterSkill>, sqlx::Error> {
    sqlx::query_as::<_, CharacterSkill>(
        r#"
        SELECT id, character_id, skill_id, skill_level, experience
        FROM character_skills
        WHERE character_id = $1
        ORDER BY skill_level DESC
        "#,
    )
    .bind(character_id)
    .fetch_all(pool)
    .await
}

/// Learn a new skill
pub async fn learn_skill(
    pool: &DbPool,
    character_id: Uuid,
    skill_id: Uuid,
) -> Result<CharacterSkill, sqlx::Error> {
    sqlx::query_as::<_, CharacterSkill>(
        r#"
        INSERT INTO character_skills (id, character_id, skill_id, skill_level, experience)
        VALUES (gen_random_uuid(), $1, $2, 1, 0)
        ON CONFLICT (character_id, skill_id) DO NOTHING
        RETURNING id, character_id, skill_id, skill_level, experience
        "#,
    )
    .bind(character_id)
    .bind(skill_id)
    .fetch_one(pool)
    .await
}

/// Check if character knows a skill
pub async fn has_skill(
    pool: &DbPool,
    character_id: Uuid,
    skill_id: Uuid,
) -> Result<bool, sqlx::Error> {
    let result: Option<(Uuid,)> = sqlx::query_as(
        r#"
        SELECT id FROM character_skills
        WHERE character_id = $1 AND skill_id = $2
        "#,
    )
    .bind(character_id)
    .bind(skill_id)
    .fetch_optional(pool)
    .await?;

    Ok(result.is_some())
}
