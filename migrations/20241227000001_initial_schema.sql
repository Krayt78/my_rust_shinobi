-- Initial database schema for My Rust Shinobi game
-- This migration creates the core tables for players, characters, items, skills, and guilds

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- ============================================================================
-- Players table - linked to wallet addresses
-- ============================================================================
CREATE TABLE players (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    wallet_address VARCHAR(64) NOT NULL UNIQUE,
    username VARCHAR(32) UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_login TIMESTAMPTZ
);

CREATE INDEX idx_players_wallet ON players(wallet_address);
CREATE INDEX idx_players_username ON players(username) WHERE username IS NOT NULL;

-- ============================================================================
-- Characters table - the ninja characters players control
-- ============================================================================
CREATE TABLE characters (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    player_id UUID NOT NULL REFERENCES players(id) ON DELETE CASCADE,
    name VARCHAR(32) NOT NULL UNIQUE,
    level INTEGER NOT NULL DEFAULT 1,
    experience BIGINT NOT NULL DEFAULT 0,
    health INTEGER NOT NULL DEFAULT 100,
    max_health INTEGER NOT NULL DEFAULT 100,
    chakra INTEGER NOT NULL DEFAULT 50,
    max_chakra INTEGER NOT NULL DEFAULT 50,
    strength INTEGER NOT NULL DEFAULT 10,
    agility INTEGER NOT NULL DEFAULT 10,
    intelligence INTEGER NOT NULL DEFAULT 10,
    village VARCHAR(32),
    rank VARCHAR(32) NOT NULL DEFAULT 'Genin',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_characters_player ON characters(player_id);
CREATE INDEX idx_characters_name ON characters(name);
CREATE INDEX idx_characters_level ON characters(level DESC);

-- ============================================================================
-- Items table - all items in the game
-- ============================================================================
CREATE TABLE items (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(64) NOT NULL UNIQUE,
    description TEXT,
    item_type VARCHAR(32) NOT NULL, -- weapon, armor, consumable, scroll, material
    rarity VARCHAR(16) NOT NULL DEFAULT 'common', -- common, uncommon, rare, epic, legendary
    base_price BIGINT NOT NULL DEFAULT 0,
    stats JSONB -- flexible stats storage (damage, defense, etc.)
);

CREATE INDEX idx_items_type ON items(item_type);
CREATE INDEX idx_items_rarity ON items(rarity);

-- ============================================================================
-- Inventory table - items owned by characters
-- ============================================================================
CREATE TABLE inventory (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    character_id UUID NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    item_id UUID NOT NULL REFERENCES items(id) ON DELETE CASCADE,
    quantity INTEGER NOT NULL DEFAULT 1,
    equipped BOOLEAN NOT NULL DEFAULT FALSE,
    slot VARCHAR(32), -- head, body, weapon, accessory, etc.
    UNIQUE(character_id, item_id, slot)
);

CREATE INDEX idx_inventory_character ON inventory(character_id);
CREATE INDEX idx_inventory_equipped ON inventory(character_id) WHERE equipped = TRUE;

-- ============================================================================
-- Skills table - jutsu and abilities
-- ============================================================================
CREATE TABLE skills (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(64) NOT NULL UNIQUE,
    description TEXT,
    skill_type VARCHAR(32) NOT NULL, -- ninjutsu, taijutsu, genjutsu, medical
    element VARCHAR(16), -- fire, water, earth, wind, lightning, null
    chakra_cost INTEGER NOT NULL DEFAULT 0,
    cooldown_seconds INTEGER NOT NULL DEFAULT 0,
    base_damage INTEGER,
    required_level INTEGER NOT NULL DEFAULT 1
);

CREATE INDEX idx_skills_type ON skills(skill_type);
CREATE INDEX idx_skills_element ON skills(element) WHERE element IS NOT NULL;
CREATE INDEX idx_skills_level ON skills(required_level);

-- ============================================================================
-- Character skills - skills learned by characters
-- ============================================================================
CREATE TABLE character_skills (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    character_id UUID NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    skill_id UUID NOT NULL REFERENCES skills(id) ON DELETE CASCADE,
    skill_level INTEGER NOT NULL DEFAULT 1,
    experience BIGINT NOT NULL DEFAULT 0,
    UNIQUE(character_id, skill_id)
);

CREATE INDEX idx_character_skills_character ON character_skills(character_id);

-- ============================================================================
-- Guilds table - clans and organizations
-- ============================================================================
CREATE TABLE guilds (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(32) NOT NULL UNIQUE,
    description TEXT,
    leader_id UUID NOT NULL REFERENCES characters(id),
    village VARCHAR(32),
    level INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_guilds_name ON guilds(name);
CREATE INDEX idx_guilds_village ON guilds(village) WHERE village IS NOT NULL;

-- ============================================================================
-- Guild members table
-- ============================================================================
CREATE TABLE guild_members (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    guild_id UUID NOT NULL REFERENCES guilds(id) ON DELETE CASCADE,
    character_id UUID NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    rank VARCHAR(32) NOT NULL DEFAULT 'member', -- leader, officer, member
    joined_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(character_id) -- A character can only be in one guild
);

CREATE INDEX idx_guild_members_guild ON guild_members(guild_id);
CREATE INDEX idx_guild_members_character ON guild_members(character_id);

-- ============================================================================
-- Insert some starter items and skills
-- ============================================================================

-- Basic starter items
INSERT INTO items (name, description, item_type, rarity, base_price, stats) VALUES
('Kunai', 'A basic throwing knife used by all ninjas', 'weapon', 'common', 100, '{"damage": 5}'),
('Shuriken', 'Small throwing stars', 'weapon', 'common', 50, '{"damage": 3}'),
('Basic Headband', 'A simple ninja headband', 'armor', 'common', 200, '{"defense": 2}'),
('Health Potion', 'Restores 50 HP', 'consumable', 'common', 100, '{"heal": 50}'),
('Chakra Pill', 'Restores 25 chakra', 'consumable', 'common', 150, '{"chakra_restore": 25}');

-- Basic starter skills
INSERT INTO skills (name, description, skill_type, element, chakra_cost, cooldown_seconds, base_damage, required_level) VALUES
('Basic Punch', 'A simple but effective punch', 'taijutsu', NULL, 0, 0, 10, 1),
('Basic Kick', 'A powerful kick', 'taijutsu', NULL, 0, 1, 12, 1),
('Clone Jutsu', 'Creates illusory clones to confuse enemies', 'ninjutsu', NULL, 10, 5, 0, 1),
('Substitution Jutsu', 'Replace yourself with an object to avoid damage', 'ninjutsu', NULL, 15, 10, 0, 3),
('Fireball Jutsu', 'A powerful fire technique', 'ninjutsu', 'fire', 30, 8, 35, 5),
('Water Dragon Jutsu', 'Summons a water dragon to attack', 'ninjutsu', 'water', 40, 12, 45, 10),
('Healing Palm', 'Basic medical ninjutsu to heal wounds', 'medical', NULL, 20, 5, 0, 5);

