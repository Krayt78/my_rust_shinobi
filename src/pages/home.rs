//! Home page - The main dashboard for players

use leptos::prelude::*;
use crate::wallet::context::use_wallet;
use crate::components::StatBar;

/// Location data structure
#[derive(Clone, Debug, PartialEq)]
pub struct Location {
    pub id: &'static str,
    pub name: &'static str,
    pub icon: &'static str,
    pub description: &'static str,
}

/// Available locations in the kingdom
const LOCATIONS: &[Location] = &[
    Location { id: "town_square", name: "Town Square", icon: "🏰", description: "The heart of the town" },
    Location { id: "guild_hall", name: "Adventurer's Guild", icon: "⚔️", description: "Accept quests" },
    Location { id: "castle", name: "Castle", icon: "👑", description: "The royal court" },
    Location { id: "temple", name: "Temple", icon: "⛪", description: "Heal and pray" },
    Location { id: "tavern", name: "Tavern", icon: "🍺", description: "Rest and rumors" },
    Location { id: "training", name: "Training Grounds", icon: "🎯", description: "Hone your skills" },
    Location { id: "market", name: "Marketplace", icon: "🛒", description: "Buy & sell goods" },
    Location { id: "arena", name: "Colosseum", icon: "🏟️", description: "PvP battles" },
    Location { id: "gates", name: "Town Gates", icon: "🚪", description: "Leave town" },
    Location { id: "wizard_tower", name: "Wizard Tower", icon: "🗼", description: "Learn magic" },
    Location { id: "blacksmith", name: "Blacksmith", icon: "🔨", description: "Forge equipment" },
];

fn get_location(id: &str) -> Option<&'static Location> {
    LOCATIONS.iter().find(|l| l.id == id)
}

/// Home page with 3-column layout
#[component]
pub fn HomePage() -> impl IntoView {
    // Shared state for current location
    let current_location = RwSignal::new("town_square".to_string());
    provide_context(current_location);
    
    view! {
        <div class="three-column-layout">
            // Left Panel - Character Stats (25%)
            <aside class="panel left-panel">
                <CharacterPanel />
            </aside>
            
            // Center Panel - Main Content (50%)
            <section class="panel center-panel">
                <CenterContent />
            </section>
            
            // Right Panel - Location Actions (25%)
            <aside class="panel right-panel">
                <LocationActionsPanel />
            </aside>
        </div>
    }
}

/// Left Panel - Character Stats and Avatar
#[component]
fn CharacterPanel() -> impl IntoView {
    let wallet = use_wallet();
    let is_connected = move || wallet.get().connected;
    let player_info = move || wallet.get().player.clone();
    
    view! {
        <div class="character-panel">
            <h2 class="panel-title">"Character"</h2>
            
            {move || {
                if is_connected() {
                    view! {
                        <div class="character-content">
                            // Character Avatar
                            <div class="character-avatar">
                                <div class="avatar-frame">
                                    <div class="avatar-placeholder">"🧙‍♂️"</div>
                                </div>
                                <div class="character-name">
                                    {move || player_info()
                                        .and_then(|p| p.username)
                                        .unwrap_or_else(|| "Wandering Hero".to_string())}
                                </div>
                                <div class="character-rank">"Adventurer"</div>
                            </div>
                            
                            // Character Stats
                            <div class="stats-container">
                                <h3 class="stats-title">"Stats"</h3>
                                
                                // HP Bar
                                <StatBar 
                                    label="HP" 
                                    current=100 
                                    max=100 
                                    color="#e74c3c"
                                />
                                
                                // Mana Bar
                                <StatBar 
                                    label="Mana" 
                                    current=50 
                                    max=50 
                                    color="#3498db"
                                />
                                
                                // Experience Bar
                                <StatBar 
                                    label="EXP" 
                                    current=0 
                                    max=100 
                                    color="#f1c40f"
                                />
                                
                                // Attributes
                                <div class="attributes">
                                    <div class="attribute">
                                        <span class="attr-icon">"💪"</span>
                                        <span class="attr-name">"Strength"</span>
                                        <span class="attr-value">"10"</span>
                                    </div>
                                    <div class="attribute">
                                        <span class="attr-icon">"🏃"</span>
                                        <span class="attr-name">"Dexterity"</span>
                                        <span class="attr-value">"10"</span>
                                    </div>
                                    <div class="attribute">
                                        <span class="attr-icon">"🧠"</span>
                                        <span class="attr-name">"Intelligence"</span>
                                        <span class="attr-value">"10"</span>
                                    </div>
                                </div>
                                
                                // Level
                                <div class="level-display">
                                    <span class="level-label">"Level"</span>
                                    <span class="level-value">"1"</span>
                                </div>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="not-connected">
                            <p>"Connect your wallet to view your character"</p>
                        </div>
                    }.into_any()
                }
            }}
        </div>
    }
}

/// Center Panel - Kingdom Map and Chat
#[component]
fn CenterContent() -> impl IntoView {
    let wallet = use_wallet();
    let is_connected = move || wallet.get().connected;
    
    view! {
        <div class="center-content-split">
            {move || {
                if !is_connected() {
                    view! {
                        <div class="welcome-screen">
                            <div class="welcome-icon">"🏰"</div>
                            <h1>"Welcome to the Realm"</h1>
                            <p>"Connect your wallet to begin your adventure"</p>
                            <div class="features">
                                <div class="feature">
                                    <span class="feature-icon">"⚔️"</span>
                                    <span>"Battle fierce monsters"</span>
                                </div>
                                <div class="feature">
                                    <span class="feature-icon">"📜"</span>
                                    <span>"Complete epic quests"</span>
                                </div>
                                <div class="feature">
                                    <span class="feature-icon">"🏴"</span>
                                    <span>"Join a guild"</span>
                                </div>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <>
                            // Kingdom Map (Top)
                            <KingdomMap />
                            
                            // Chat (Bottom)
                            <TavernChat />
                        </>
                    }.into_any()
                }
            }}
        </div>
    }
}

/// Kingdom Map with clickable locations overlaid on background image
#[component]
fn KingdomMap() -> impl IntoView {
    let current_location = expect_context::<RwSignal<String>>();
    let location_name = move || {
        get_location(&current_location.get())
            .map(|l| l.name)
            .unwrap_or("Unknown")
    };
    
    view! {
        <div class="village-map-container">
            <div class="map-header">
                <h2 class="map-title">"🏰 Eldoria Kingdom"</h2>
                <span class="map-location">"📍 " {location_name}</span>
            </div>
            
            <div class="village-map-wrapper">
                // Background image
                <img 
                    src="/images/starting village.png" 
                    alt="Kingdom Map" 
                    class="village-map-bg"
                />
                
                // Clickable location overlays
                <div class="map-locations-overlay">
                    <MapLocationOverlay 
                        id="castle"
                        name="Castle" 
                        icon="👑" 
                        top="15%"
                        left="50%"
                    />
                    <MapLocationOverlay 
                        id="wizard_tower"
                        name="Wizard Tower" 
                        icon="🗼" 
                        top="22%"
                        left="25%"
                    />
                    <MapLocationOverlay 
                        id="guild_hall"
                        name="Guild Hall" 
                        icon="⚔️" 
                        top="25%"
                        left="75%"
                    />
                    <MapLocationOverlay 
                        id="temple"
                        name="Temple" 
                        icon="⛪" 
                        top="40%"
                        left="15%"
                    />
                    <MapLocationOverlay 
                        id="tavern"
                        name="Tavern" 
                        icon="🍺" 
                        top="50%"
                        left="35%"
                    />
                    <MapLocationOverlay 
                        id="blacksmith"
                        name="Blacksmith" 
                        icon="🔨" 
                        top="55%"
                        left="65%"
                    />
                    <MapLocationOverlay 
                        id="training"
                        name="Training" 
                        icon="🎯" 
                        top="45%"
                        left="85%"
                    />
                    <MapLocationOverlay 
                        id="market"
                        name="Market" 
                        icon="🛒" 
                        top="70%"
                        left="30%"
                    />
                    <MapLocationOverlay 
                        id="arena"
                        name="Colosseum" 
                        icon="🏟️" 
                        top="70%"
                        left="60%"
                    />
                    <MapLocationOverlay 
                        id="gates"
                        name="Gates" 
                        icon="🚪" 
                        top="85%"
                        left="50%"
                    />
                </div>
            </div>
        </div>
    }
}

/// Individual map location overlay (positioned absolutely on the map)
#[component]
fn MapLocationOverlay(
    id: &'static str,
    name: &'static str,
    icon: &'static str,
    top: &'static str,
    left: &'static str,
) -> impl IntoView {
    let current_location = expect_context::<RwSignal<String>>();
    let is_current = move || current_location.get() == id;
    
    view! {
        <button 
            class=move || if is_current() { "map-location-overlay active" } else { "map-location-overlay" }
            style=format!("top: {}; left: {};", top, left)
            on:click=move |_| current_location.set(id.to_string())
        >
            <div class="location-marker-icon">{icon}</div>
            <span class="location-label">{name}</span>
        </button>
    }
}

/// Tavern Chat component
#[component]
fn TavernChat() -> impl IntoView {
    let chat_input = RwSignal::new(String::new());
    
    view! {
        <div class="village-chat">
            <div class="chat-header">
                <span class="chat-icon">"💬"</span>
                <span class="chat-title">"Tavern Chat"</span>
                <div class="chat-tabs">
                    <button class="chat-tab active">"Global"</button>
                    <button class="chat-tab">"Guild"</button>
                    <button class="chat-tab">"Whisper"</button>
                </div>
            </div>
            
            <div class="chat-messages">
                <ChatMessage 
                    username="Thorin" 
                    message="Anyone want to raid the dragon's lair?" 
                    time="2m ago"
                    rank="Knight"
                />
                <ChatMessage 
                    username="Elara" 
                    message="I just found a legendary sword! ⚔️" 
                    time="5m ago"
                    rank="Archmage"
                />
                <ChatMessage 
                    username="Finn" 
                    message="The tournament starts in 30 minutes" 
                    time="8m ago"
                    rank="Squire"
                />
                <ChatMessage 
                    username="Herald" 
                    message="Welcome to Eldoria! New adventurers can visit the Guild Hall for quests." 
                    time="10m ago"
                    rank="System"
                />
            </div>
            
            <div class="chat-input-container">
                <input 
                    type="text" 
                    class="chat-input"
                    placeholder="Type a message..."
                    prop:value=move || chat_input.get()
                    on:input=move |ev| chat_input.set(event_target_value(&ev))
                />
                <button class="chat-send-btn">"Send"</button>
            </div>
        </div>
    }
}

/// Individual chat message
#[component]
fn ChatMessage(
    username: &'static str,
    message: &'static str,
    time: &'static str,
    rank: &'static str,
) -> impl IntoView {
    let rank_class = match rank {
        "System" => "rank-system",
        "Archmage" => "rank-jonin",
        "Knight" => "rank-chunin",
        _ => "rank-genin",
    };
    
    view! {
        <div class=format!("chat-message {}", rank_class)>
            <div class="message-header">
                <span class=format!("message-username {}", rank_class)>{username}</span>
                <span class="message-rank">"["{rank}"]"</span>
                <span class="message-time">{time}</span>
            </div>
            <div class="message-content">{message}</div>
        </div>
    }
}

/// Right Panel - Location-based actions
#[component]
fn LocationActionsPanel() -> impl IntoView {
    let wallet = use_wallet();
    let is_connected = move || wallet.get().connected;
    let current_location = expect_context::<RwSignal<String>>();
    
    let location_info = move || {
        get_location(&current_location.get())
    };
    
    view! {
        <div class="location-actions-panel">
            {move || {
                if !is_connected() {
                    view! {
                        <div class="not-connected">
                            <p>"Connect wallet to see location actions"</p>
                        </div>
                    }.into_any()
                } else {
                    let loc = location_info();
                    view! {
                        <>
                            // Location header with image placeholder
                            <div class="location-header">
                                <div class="location-image">
                                    <span class="location-icon-large">{loc.map(|l| l.icon).unwrap_or("📍")}</span>
                                </div>
                                <div class="location-title">
                                    <span class="location-marker-text">"📍 at "</span>
                                    <span class="location-name-text">{loc.map(|l| l.name).unwrap_or("Unknown")}</span>
                                </div>
                            </div>
                            
                            // Actions based on location
                            <div class="location-actions">
                                <LocationActions location_id=current_location.get() />
                            </div>
                        </>
                    }.into_any()
                }
            }}
        </div>
    }
}

/// Actions available at each location
#[component]
fn LocationActions(location_id: String) -> impl IntoView {
    match location_id.as_str() {
        "guild_hall" => view! {
            <ActionButton 
                name="Accept Quest" 
                description="Browse available quests" 
                cost="Free" 
                icon="📜"
                category="mission"
            />
            <ActionButton 
                name="Turn In Quest" 
                description="Complete a quest" 
                cost="Free" 
                icon="✅"
                category="mission"
            />
            <ActionButton 
                name="Guild Board" 
                description="View guild rankings" 
                cost="Free" 
                icon="📋"
                category="social"
            />
            <ActionButton 
                name="Party Finder" 
                description="Find adventuring party" 
                cost="Free" 
                icon="👥"
                category="social"
            />
        }.into_any(),
        
        "wizard_tower" => view! {
            <ActionButton 
                name="Study Fire Magic" 
                description="Learn fire spells" 
                cost="1 AP" 
                icon="🔥"
                category="magic"
            />
            <ActionButton 
                name="Study Ice Magic" 
                description="Learn frost spells" 
                cost="1 AP" 
                icon="❄️"
                category="magic"
            />
            <ActionButton 
                name="Study Lightning" 
                description="Learn storm spells" 
                cost="1 AP" 
                icon="⚡"
                category="magic"
            />
            <ActionButton 
                name="Enchant Item" 
                description="Add magic to equipment" 
                cost="50 Gold" 
                icon="✨"
                category="craft"
            />
            <ActionButton 
                name="Identify Item" 
                description="Reveal item properties" 
                cost="10 Gold" 
                icon="🔍"
                category="knowledge"
            />
        }.into_any(),
        
        "castle" => view! {
            <ActionButton 
                name="Audience with King" 
                description="Request royal quest" 
                cost="Free" 
                icon="👑"
                category="mission"
            />
            <ActionButton 
                name="Royal Treasury" 
                description="Exchange rare items" 
                cost="Free" 
                icon="💎"
                category="shop"
            />
            <ActionButton 
                name="War Council" 
                description="View kingdom affairs" 
                cost="Free" 
                icon="🗺️"
                category="knowledge"
            />
            <ActionButton 
                name="Knight's Order" 
                description="Join the knights" 
                cost="Free" 
                icon="🛡️"
                category="social"
            />
        }.into_any(),
        
        "temple" => view! {
            <ActionButton 
                name="Heal Wounds" 
                description="Restore HP" 
                cost="25 Gold" 
                icon="💚"
                category="heal"
            />
            <ActionButton 
                name="Full Restoration" 
                description="Restore HP & Mana" 
                cost="50 Gold" 
                icon="✝️"
                category="heal"
            />
            <ActionButton 
                name="Remove Curse" 
                description="Cure afflictions" 
                cost="100 Gold" 
                icon="🙏"
                category="heal"
            />
            <ActionButton 
                name="Learn Holy Magic" 
                description="Divine spells" 
                cost="1 AP" 
                icon="☀️"
                category="magic"
            />
            <ActionButton 
                name="Donate" 
                description="Gain blessings" 
                cost="Variable" 
                icon="🪙"
                category="social"
            />
        }.into_any(),
        
        "tavern" => view! {
            <ActionButton 
                name="Rest" 
                description="Recover over time" 
                cost="5 Gold" 
                icon="🛏️"
                category="rest"
            />
            <ActionButton 
                name="Buy a Drink" 
                description="Hear rumors" 
                cost="2 Gold" 
                icon="🍺"
                category="social"
            />
            <ActionButton 
                name="Gamble" 
                description="Try your luck" 
                cost="Variable" 
                icon="🎲"
                category="social"
            />
            <ActionButton 
                name="Recruit Mercenary" 
                description="Hire help" 
                cost="50 Gold" 
                icon="💂"
                category="social"
            />
            <ActionButton 
                name="Bard's Tale" 
                description="Gain inspiration" 
                cost="Free" 
                icon="🎵"
                category="rest"
            />
        }.into_any(),
        
        "training" => view! {
            <ActionButton 
                name="Strength Training" 
                description="Increase STR" 
                cost="1 AP" 
                icon="💪"
                category="melee"
            />
            <ActionButton 
                name="Agility Training" 
                description="Increase DEX" 
                cost="1 AP" 
                icon="🏃"
                category="melee"
            />
            <ActionButton 
                name="Combat Practice" 
                description="Improve skills" 
                cost="1 AP" 
                icon="⚔️"
                category="combat"
            />
            <ActionButton 
                name="Archery Range" 
                description="Ranged training" 
                cost="1 AP" 
                icon="🏹"
                category="ranged"
            />
            <ActionButton 
                name="Sparring Match" 
                description="Practice combat" 
                cost="Free" 
                icon="🤺"
                category="combat"
            />
        }.into_any(),
        
        "blacksmith" => view! {
            <ActionButton 
                name="Forge Weapon" 
                description="Create weapons" 
                cost="Materials" 
                icon="⚔️"
                category="craft"
            />
            <ActionButton 
                name="Forge Armor" 
                description="Create armor" 
                cost="Materials" 
                icon="🛡️"
                category="craft"
            />
            <ActionButton 
                name="Repair Equipment" 
                description="Fix damaged gear" 
                cost="Variable" 
                icon="🔧"
                category="craft"
            />
            <ActionButton 
                name="Upgrade Item" 
                description="Enhance equipment" 
                cost="Gold + Mats" 
                icon="⬆️"
                category="craft"
            />
            <ActionButton 
                name="Salvage" 
                description="Break down items" 
                cost="Free" 
                icon="♻️"
                category="craft"
            />
        }.into_any(),
        
        "market" => view! {
            <ActionButton 
                name="Buy Items" 
                description="Purchase goods" 
                cost="Free" 
                icon="🛒"
                category="shop"
            />
            <ActionButton 
                name="Sell Items" 
                description="Sell your loot" 
                cost="Free" 
                icon="💰"
                category="shop"
            />
            <ActionButton 
                name="Auction House" 
                description="Player market" 
                cost="Free" 
                icon="🏛️"
                category="shop"
            />
            <ActionButton 
                name="Black Market" 
                description="Rare goods..." 
                cost="Free" 
                icon="🕶️"
                category="shop"
            />
        }.into_any(),
        
        "arena" => view! {
            <ActionButton 
                name="Quick Duel" 
                description="Fight random opponent" 
                cost="Free" 
                icon="⚔️"
                category="combat"
            />
            <ActionButton 
                name="Ranked Battle" 
                description="Competitive match" 
                cost="1 AP" 
                icon="🏆"
                category="combat"
            />
            <ActionButton 
                name="Tournament" 
                description="Join tournament" 
                cost="Entry Fee" 
                icon="👑"
                category="combat"
            />
            <ActionButton 
                name="Monster Arena" 
                description="Fight beasts" 
                cost="10 Gold" 
                icon="🐉"
                category="combat"
            />
            <ActionButton 
                name="Spectate" 
                description="Watch battles" 
                cost="Free" 
                icon="👀"
                category="social"
            />
        }.into_any(),
        
        "gates" => view! {
            <ActionButton 
                name="Guard Duty" 
                description="Earn gold" 
                cost="1 AP" 
                icon="🛡️"
                category="mission"
            />
            <ActionButton 
                name="Leave Town" 
                description="Go adventuring" 
                cost="Free" 
                icon="🚪"
                category="travel"
            />
            <ActionButton 
                name="World Map" 
                description="View regions" 
                cost="Free" 
                icon="🗺️"
                category="travel"
            />
            <ActionButton 
                name="Caravan" 
                description="Fast travel" 
                cost="20 Gold" 
                icon="🐴"
                category="travel"
            />
        }.into_any(),
        
        // Town Square (default)
        _ => view! {
            <ActionButton 
                name="Rest" 
                description="Recover stamina" 
                cost="Free" 
                icon="😴"
                category="rest"
            />
            <ActionButton 
                name="Look Around" 
                description="See what's happening" 
                cost="Free" 
                icon="👀"
                category="social"
            />
            <ActionButton 
                name="Town Crier" 
                description="Hear announcements" 
                cost="Free" 
                icon="📢"
                category="knowledge"
            />
            <ActionButton 
                name="Bounty Board" 
                description="View wanted posters" 
                cost="Free" 
                icon="📋"
                category="mission"
            />
        }.into_any(),
    }
}

/// Action button component
#[component]
fn ActionButton(
    name: &'static str,
    description: &'static str,
    cost: &'static str,
    icon: &'static str,
    category: &'static str,
) -> impl IntoView {
    let category_class = format!("action-item category-{}", category);
    
    view! {
        <button class=category_class>
            <div class="action-main">
                <span class="action-icon">{icon}</span>
                <div class="action-text">
                    <span class="action-name">{name}</span>
                    <span class="action-desc">{description}</span>
                </div>
            </div>
            <span class="action-cost">{cost}</span>
        </button>
    }
}
