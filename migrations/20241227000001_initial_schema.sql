-- Initial database schema for Realm of Legends game
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
-- Characters table - the adventurer characters players control
-- ============================================================================
CREATE TABLE characters (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    player_id UUID NOT NULL REFERENCES players(id) ON DELETE CASCADE,
    name VARCHAR(32) NOT NULL UNIQUE,
    level INTEGER NOT NULL DEFAULT 1,
    experience BIGINT NOT NULL DEFAULT 0,
    health INTEGER NOT NULL DEFAULT 100,
    max_health INTEGER NOT NULL DEFAULT 100,
    mana INTEGER NOT NULL DEFAULT 50,
    max_mana INTEGER NOT NULL DEFAULT 50,
    -- D&D style attributes
    strength INTEGER NOT NULL DEFAULT 10,
    dexterity INTEGER NOT NULL DEFAULT 10,
    intelligence INTEGER NOT NULL DEFAULT 10,
    constitution INTEGER NOT NULL DEFAULT 10,
    wisdom INTEGER NOT NULL DEFAULT 10,
    charisma INTEGER NOT NULL DEFAULT 10,
    -- Resources
    gold BIGINT NOT NULL DEFAULT 100,
    action_points INTEGER NOT NULL DEFAULT 10,
    max_action_points INTEGER NOT NULL DEFAULT 10,
    -- Class/Type
    character_class VARCHAR(32),
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
    item_type VARCHAR(32) NOT NULL, -- weapon, armor, consumable, material, accessory
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
-- Skills table - spells and abilities
-- ============================================================================
CREATE TABLE skills (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(64) NOT NULL UNIQUE,
    description TEXT,
    skill_type VARCHAR(32) NOT NULL, -- melee, ranged, magic, support, passive
    element VARCHAR(16), -- fire, water, earth, wind, lightning, holy, dark, null
    mana_cost INTEGER NOT NULL DEFAULT 0,
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
-- Guilds table - player organizations
-- ============================================================================
CREATE TABLE guilds (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(32) NOT NULL UNIQUE,
    description TEXT,
    leader_id UUID NOT NULL REFERENCES characters(id),
    level INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_guilds_name ON guilds(name);

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
-- Insert some starter items and skills (Fantasy themed)
-- ============================================================================

-- Basic starter items
INSERT INTO items (name, description, item_type, rarity, base_price, stats) VALUES
('Iron Sword', 'A basic iron sword for beginners', 'weapon', 'common', 100, '{"damage": 8}'),
('Wooden Shield', 'A simple wooden shield', 'armor', 'common', 75, '{"defense": 3}'),
('Leather Cap', 'Basic head protection', 'armor', 'common', 50, '{"defense": 1}'),
('Bread Loaf', 'Restores 20 HP', 'consumable', 'common', 10, '{"heal": 20}'),
('Health Potion', 'Restores 50 HP', 'consumable', 'common', 100, '{"heal": 50}'),
('Mana Potion', 'Restores 25 mana', 'consumable', 'common', 120, '{"mana_restore": 25}'),
('Torch', 'Lights the way in dark places', 'material', 'common', 5, '{}'),
('Rope', 'Useful for climbing and tying things', 'material', 'common', 15, '{}');

-- Basic starter skills
INSERT INTO skills (name, description, skill_type, element, mana_cost, cooldown_seconds, base_damage, required_level) VALUES
('Basic Attack', 'A simple weapon strike', 'melee', NULL, 0, 0, 10, 1),
('Power Strike', 'A powerful overhead attack', 'melee', NULL, 5, 3, 18, 1),
('Shield Bash', 'Strike with your shield to stun enemies', 'melee', NULL, 8, 5, 8, 3),
('Fireball', 'Hurl a ball of fire at your enemies', 'magic', 'fire', 15, 4, 25, 3),
('Frost Bolt', 'A bolt of ice that slows enemies', 'magic', 'water', 12, 3, 18, 3),
('Lightning Bolt', 'Call down lightning from the sky', 'magic', 'lightning', 25, 6, 40, 8),
('Heal', 'Restore health to yourself or an ally', 'support', 'holy', 20, 5, 0, 1),
('Bless', 'Increase an ally''s strength temporarily', 'support', 'holy', 15, 10, 0, 5);
