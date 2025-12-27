-- Town and Location System Migration
-- Creates tables for towns, locations, and actions

-- ============================================================================
-- ENUM TYPES
-- ============================================================================

CREATE TYPE location_type AS ENUM (
    'shop',
    'training',
    'service',
    'social',
    'quest',
    'crafting',
    'combat',
    'travel',
    'special'
);

CREATE TYPE action_type AS ENUM (
    'instant',
    'timed',
    'dialog',
    'navigation',
    'combat',
    'shop'
);

CREATE TYPE action_category AS ENUM (
    'combat',
    'magic',
    'melee',
    'ranged',
    'heal',
    'rest',
    'shop',
    'craft',
    'social',
    'mission',
    'travel',
    'knowledge'
);

-- ============================================================================
-- TOWNS TABLE
-- ============================================================================

CREATE TABLE towns (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    region VARCHAR(100) NOT NULL DEFAULT 'starting_zone',
    required_level INT NOT NULL DEFAULT 1,
    map_image VARCHAR(500),
    is_safe_zone BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_towns_region ON towns(region);
CREATE INDEX idx_towns_required_level ON towns(required_level);

-- ============================================================================
-- LOCATIONS TABLE
-- ============================================================================

CREATE TABLE locations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    town_id UUID NOT NULL REFERENCES towns(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    icon VARCHAR(50) NOT NULL DEFAULT '📍',
    location_type location_type NOT NULL DEFAULT 'special',
    map_position_x REAL NOT NULL DEFAULT 50.0,
    map_position_y REAL NOT NULL DEFAULT 50.0,
    required_level INT NOT NULL DEFAULT 1,
    required_quest_id UUID,
    is_active BOOLEAN NOT NULL DEFAULT true,
    sort_order INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_locations_town_id ON locations(town_id);
CREATE INDEX idx_locations_type ON locations(location_type);
CREATE INDEX idx_locations_active ON locations(is_active) WHERE is_active = true;

-- ============================================================================
-- LOCATION ACTIONS TABLE
-- ============================================================================

CREATE TABLE location_actions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    location_id UUID NOT NULL REFERENCES locations(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    icon VARCHAR(50) NOT NULL DEFAULT '⚡',
    action_type action_type NOT NULL DEFAULT 'instant',
    category action_category NOT NULL DEFAULT 'social',
    
    -- Requirements
    required_level INT NOT NULL DEFAULT 1,
    required_gold BIGINT NOT NULL DEFAULT 0,
    required_item_id UUID REFERENCES items(id) ON DELETE SET NULL,
    required_item_quantity INT NOT NULL DEFAULT 0,
    action_points_cost INT NOT NULL DEFAULT 0,
    
    -- Timing
    cooldown_seconds INT NOT NULL DEFAULT 0,
    duration_seconds INT NOT NULL DEFAULT 0,
    
    -- Outcomes (flexible JSON structure)
    rewards JSONB,
    
    is_repeatable BOOLEAN NOT NULL DEFAULT true,
    is_active BOOLEAN NOT NULL DEFAULT true,
    sort_order INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_location_actions_location_id ON location_actions(location_id);
CREATE INDEX idx_location_actions_type ON location_actions(action_type);
CREATE INDEX idx_location_actions_category ON location_actions(category);
CREATE INDEX idx_location_actions_active ON location_actions(is_active) WHERE is_active = true;

-- ============================================================================
-- PLAYER LOCATION TRACKING
-- ============================================================================

CREATE TABLE player_locations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    character_id UUID NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    town_id UUID NOT NULL REFERENCES towns(id) ON DELETE CASCADE,
    location_id UUID REFERENCES locations(id) ON DELETE SET NULL,
    entered_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Each character can only be in one place
    UNIQUE(character_id)
);

CREATE INDEX idx_player_locations_character ON player_locations(character_id);
CREATE INDEX idx_player_locations_town ON player_locations(town_id);

-- ============================================================================
-- ACTION COOLDOWNS
-- ============================================================================

CREATE TABLE action_cooldowns (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    character_id UUID NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    action_id UUID NOT NULL REFERENCES location_actions(id) ON DELETE CASCADE,
    available_at TIMESTAMPTZ NOT NULL,
    
    -- One cooldown entry per character per action
    UNIQUE(character_id, action_id)
);

CREATE INDEX idx_action_cooldowns_character ON action_cooldowns(character_id);
CREATE INDEX idx_action_cooldowns_available ON action_cooldowns(available_at);

-- ============================================================================
-- COMPLETED ACTIONS (for tracking progress)
-- ============================================================================

CREATE TABLE completed_actions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    character_id UUID NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    action_id UUID NOT NULL REFERENCES location_actions(id) ON DELETE CASCADE,
    completed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    times_completed INT NOT NULL DEFAULT 1,
    
    UNIQUE(character_id, action_id)
);

CREATE INDEX idx_completed_actions_character ON completed_actions(character_id);

-- ============================================================================
-- SEED DATA: Eldoria Town
-- ============================================================================

-- Insert the starting town
INSERT INTO towns (id, name, description, region, required_level, map_image, is_safe_zone)
VALUES (
    'a0000000-0000-0000-0000-000000000001'::uuid,
    'Eldoria',
    'A prosperous kingdom nestled in the valley, where brave adventurers begin their journey.',
    'starting_zone',
    1,
    '/images/starting village.png',
    true
);

-- Insert locations for Eldoria
INSERT INTO locations (id, town_id, name, description, icon, location_type, map_position_x, map_position_y, sort_order) VALUES
    ('b0000000-0000-0000-0000-000000000001'::uuid, 'a0000000-0000-0000-0000-000000000001'::uuid, 'Castle', 'The royal court where the king holds audience', '👑', 'quest', 50.0, 15.0, 1),
    ('b0000000-0000-0000-0000-000000000002'::uuid, 'a0000000-0000-0000-0000-000000000001'::uuid, 'Wizard Tower', 'A tall spire where arcane knowledge is studied', '🗼', 'training', 25.0, 22.0, 2),
    ('b0000000-0000-0000-0000-000000000003'::uuid, 'a0000000-0000-0000-0000-000000000001'::uuid, 'Guild Hall', 'The Adventurer''s Guild headquarters', '⚔️', 'quest', 75.0, 25.0, 3),
    ('b0000000-0000-0000-0000-000000000004'::uuid, 'a0000000-0000-0000-0000-000000000001'::uuid, 'Temple', 'A holy sanctuary offering healing and blessings', '⛪', 'service', 15.0, 40.0, 4),
    ('b0000000-0000-0000-0000-000000000005'::uuid, 'a0000000-0000-0000-0000-000000000001'::uuid, 'Tavern', 'The Golden Flagon - where adventurers gather', '🍺', 'social', 35.0, 50.0, 5),
    ('b0000000-0000-0000-0000-000000000006'::uuid, 'a0000000-0000-0000-0000-000000000001'::uuid, 'Blacksmith', 'Master Brondar''s forge', '🔨', 'crafting', 65.0, 55.0, 6),
    ('b0000000-0000-0000-0000-000000000007'::uuid, 'a0000000-0000-0000-0000-000000000001'::uuid, 'Training Grounds', 'Practice your combat skills', '🎯', 'training', 85.0, 45.0, 7),
    ('b0000000-0000-0000-0000-000000000008'::uuid, 'a0000000-0000-0000-0000-000000000001'::uuid, 'Marketplace', 'Buy and sell goods', '🛒', 'shop', 30.0, 70.0, 8),
    ('b0000000-0000-0000-0000-000000000009'::uuid, 'a0000000-0000-0000-0000-000000000001'::uuid, 'Colosseum', 'The arena for gladiatorial combat', '🏟️', 'combat', 60.0, 70.0, 9),
    ('b0000000-0000-0000-0000-000000000010'::uuid, 'a0000000-0000-0000-0000-000000000001'::uuid, 'Town Gates', 'The entrance to the outside world', '🚪', 'travel', 50.0, 85.0, 10);

-- Insert actions for Tavern
INSERT INTO location_actions (location_id, name, description, icon, action_type, category, required_gold, cooldown_seconds, duration_seconds, rewards, sort_order) VALUES
    ('b0000000-0000-0000-0000-000000000005'::uuid, 'Rest', 'Rent a room and recover', '🛏️', 'timed', 'rest', 5, 0, 300, '{"stat_changes": {"health": 100, "mana": 50}}', 1),
    ('b0000000-0000-0000-0000-000000000005'::uuid, 'Buy a Drink', 'Hear the latest rumors', '🍺', 'instant', 'social', 2, 60, 0, '{"experience": 5}', 2),
    ('b0000000-0000-0000-0000-000000000005'::uuid, 'Gamble', 'Try your luck at dice', '🎲', 'instant', 'social', 10, 30, 0, '{"gold": 20}', 3),
    ('b0000000-0000-0000-0000-000000000005'::uuid, 'Recruit Mercenary', 'Hire a companion', '💂', 'dialog', 'social', 50, 0, 0, NULL, 4),
    ('b0000000-0000-0000-0000-000000000005'::uuid, 'Bard''s Tale', 'Listen to inspiring stories', '🎵', 'instant', 'rest', 0, 300, 0, '{"experience": 10}', 5);

-- Insert actions for Guild Hall
INSERT INTO location_actions (location_id, name, description, icon, action_type, category, required_gold, action_points_cost, rewards, sort_order) VALUES
    ('b0000000-0000-0000-0000-000000000003'::uuid, 'Accept Quest', 'Browse available quests', '📜', 'dialog', 'mission', 0, 0, NULL, 1),
    ('b0000000-0000-0000-0000-000000000003'::uuid, 'Turn In Quest', 'Complete a quest', '✅', 'dialog', 'mission', 0, 0, NULL, 2),
    ('b0000000-0000-0000-0000-000000000003'::uuid, 'Guild Board', 'View guild rankings', '📋', 'dialog', 'social', 0, 0, NULL, 3),
    ('b0000000-0000-0000-0000-000000000003'::uuid, 'Party Finder', 'Find adventuring party', '👥', 'dialog', 'social', 0, 0, NULL, 4);

-- Insert actions for Temple
INSERT INTO location_actions (location_id, name, description, icon, action_type, category, required_gold, cooldown_seconds, rewards, sort_order) VALUES
    ('b0000000-0000-0000-0000-000000000004'::uuid, 'Heal Wounds', 'Restore HP', '💚', 'instant', 'heal', 25, 0, '{"stat_changes": {"health": 50}}', 1),
    ('b0000000-0000-0000-0000-000000000004'::uuid, 'Full Restoration', 'Restore HP & Mana', '✝️', 'instant', 'heal', 50, 0, '{"stat_changes": {"health": 100, "mana": 100}}', 2),
    ('b0000000-0000-0000-0000-000000000004'::uuid, 'Remove Curse', 'Cure afflictions', '🙏', 'instant', 'heal', 100, 0, NULL, 3),
    ('b0000000-0000-0000-0000-000000000004'::uuid, 'Learn Holy Magic', 'Study divine spells', '☀️', 'timed', 'magic', 0, 3600, '{"experience": 25}', 4),
    ('b0000000-0000-0000-0000-000000000004'::uuid, 'Donate', 'Gain blessings', '🪙', 'instant', 'social', 10, 86400, '{"experience": 15}', 5);

-- Insert actions for Wizard Tower
INSERT INTO location_actions (location_id, name, description, icon, action_type, category, action_points_cost, cooldown_seconds, rewards, sort_order) VALUES
    ('b0000000-0000-0000-0000-000000000002'::uuid, 'Study Fire Magic', 'Learn fire spells', '🔥', 'timed', 'magic', 1, 3600, '{"experience": 20}', 1),
    ('b0000000-0000-0000-0000-000000000002'::uuid, 'Study Ice Magic', 'Learn frost spells', '❄️', 'timed', 'magic', 1, 3600, '{"experience": 20}', 2),
    ('b0000000-0000-0000-0000-000000000002'::uuid, 'Study Lightning', 'Learn storm spells', '⚡', 'timed', 'magic', 1, 3600, '{"experience": 20}', 3),
    ('b0000000-0000-0000-0000-000000000002'::uuid, 'Enchant Item', 'Add magic to equipment', '✨', 'dialog', 'craft', 0, 0, NULL, 4),
    ('b0000000-0000-0000-0000-000000000002'::uuid, 'Identify Item', 'Reveal item properties', '🔍', 'instant', 'knowledge', 0, 0, NULL, 5);

-- Insert actions for Blacksmith
INSERT INTO location_actions (location_id, name, description, icon, action_type, category, required_gold, rewards, sort_order) VALUES
    ('b0000000-0000-0000-0000-000000000006'::uuid, 'Forge Weapon', 'Create weapons', '⚔️', 'dialog', 'craft', 0, NULL, 1),
    ('b0000000-0000-0000-0000-000000000006'::uuid, 'Forge Armor', 'Create armor', '🛡️', 'dialog', 'craft', 0, NULL, 2),
    ('b0000000-0000-0000-0000-000000000006'::uuid, 'Repair Equipment', 'Fix damaged gear', '🔧', 'instant', 'craft', 0, NULL, 3),
    ('b0000000-0000-0000-0000-000000000006'::uuid, 'Upgrade Item', 'Enhance equipment', '⬆️', 'dialog', 'craft', 0, NULL, 4),
    ('b0000000-0000-0000-0000-000000000006'::uuid, 'Salvage', 'Break down items', '♻️', 'instant', 'craft', 0, NULL, 5);

-- Insert actions for Training Grounds
INSERT INTO location_actions (location_id, name, description, icon, action_type, category, action_points_cost, cooldown_seconds, rewards, sort_order) VALUES
    ('b0000000-0000-0000-0000-000000000007'::uuid, 'Strength Training', 'Increase STR', '💪', 'timed', 'melee', 1, 1800, '{"stat_changes": {"strength": 1}, "experience": 15}', 1),
    ('b0000000-0000-0000-0000-000000000007'::uuid, 'Agility Training', 'Increase DEX', '🏃', 'timed', 'melee', 1, 1800, '{"stat_changes": {"dexterity": 1}, "experience": 15}', 2),
    ('b0000000-0000-0000-0000-000000000007'::uuid, 'Combat Practice', 'Improve skills', '⚔️', 'timed', 'combat', 1, 1800, '{"experience": 20}', 3),
    ('b0000000-0000-0000-0000-000000000007'::uuid, 'Archery Range', 'Ranged training', '🏹', 'timed', 'ranged', 1, 1800, '{"experience": 15}', 4),
    ('b0000000-0000-0000-0000-000000000007'::uuid, 'Sparring Match', 'Practice combat', '🤺', 'combat', 'combat', 0, 300, '{"experience": 10}', 5);

-- Insert actions for Marketplace
INSERT INTO location_actions (location_id, name, description, icon, action_type, category, sort_order) VALUES
    ('b0000000-0000-0000-0000-000000000008'::uuid, 'Buy Items', 'Purchase goods', '🛒', 'shop', 'shop', 1),
    ('b0000000-0000-0000-0000-000000000008'::uuid, 'Sell Items', 'Sell your loot', '💰', 'shop', 'shop', 2),
    ('b0000000-0000-0000-0000-000000000008'::uuid, 'Auction House', 'Player market', '🏛️', 'dialog', 'shop', 3),
    ('b0000000-0000-0000-0000-000000000008'::uuid, 'Black Market', 'Rare goods...', '🕶️', 'dialog', 'shop', 4);

-- Insert actions for Colosseum
INSERT INTO location_actions (location_id, name, description, icon, action_type, category, required_gold, action_points_cost, rewards, sort_order) VALUES
    ('b0000000-0000-0000-0000-000000000009'::uuid, 'Quick Duel', 'Fight random opponent', '⚔️', 'combat', 'combat', 0, 0, '{"experience": 25, "gold": 10}', 1),
    ('b0000000-0000-0000-0000-000000000009'::uuid, 'Ranked Battle', 'Competitive match', '🏆', 'combat', 'combat', 0, 1, '{"experience": 50, "gold": 25}', 2),
    ('b0000000-0000-0000-0000-000000000009'::uuid, 'Tournament', 'Join tournament', '👑', 'dialog', 'combat', 100, 0, NULL, 3),
    ('b0000000-0000-0000-0000-000000000009'::uuid, 'Monster Arena', 'Fight beasts', '🐉', 'combat', 'combat', 10, 0, '{"experience": 30, "gold": 15}', 4),
    ('b0000000-0000-0000-0000-000000000009'::uuid, 'Spectate', 'Watch battles', '👀', 'instant', 'social', 0, 0, NULL, 5);

-- Insert actions for Town Gates
INSERT INTO location_actions (location_id, name, description, icon, action_type, category, required_gold, action_points_cost, rewards, sort_order) VALUES
    ('b0000000-0000-0000-0000-000000000010'::uuid, 'Guard Duty', 'Earn gold', '🛡️', 'timed', 'mission', 0, 1, '{"gold": 20, "experience": 10}', 1),
    ('b0000000-0000-0000-0000-000000000010'::uuid, 'Leave Town', 'Go adventuring', '🚪', 'navigation', 'travel', 0, 0, NULL, 2),
    ('b0000000-0000-0000-0000-000000000010'::uuid, 'World Map', 'View regions', '🗺️', 'dialog', 'travel', 0, 0, NULL, 3),
    ('b0000000-0000-0000-0000-000000000010'::uuid, 'Caravan', 'Fast travel', '🐴', 'navigation', 'travel', 20, 0, NULL, 4);

-- Insert actions for Castle
INSERT INTO location_actions (location_id, name, description, icon, action_type, category, required_level, rewards, sort_order) VALUES
    ('b0000000-0000-0000-0000-000000000001'::uuid, 'Audience with King', 'Request royal quest', '👑', 'dialog', 'mission', 5, NULL, 1),
    ('b0000000-0000-0000-0000-000000000001'::uuid, 'Royal Treasury', 'Exchange rare items', '💎', 'shop', 'shop', 10, NULL, 2),
    ('b0000000-0000-0000-0000-000000000001'::uuid, 'War Council', 'View kingdom affairs', '🗺️', 'dialog', 'knowledge', 1, NULL, 3),
    ('b0000000-0000-0000-0000-000000000001'::uuid, 'Knight''s Order', 'Join the knights', '🛡️', 'dialog', 'social', 15, NULL, 4);

