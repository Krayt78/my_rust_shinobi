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

/// Available locations in the village
const LOCATIONS: &[Location] = &[
    Location { id: "village_square", name: "Village Square", icon: "🏯", description: "The center of the village" },
    Location { id: "academy", name: "Academy", icon: "📚", description: "Learn new jutsu" },
    Location { id: "hokage_tower", name: "Hokage Tower", icon: "🏛️", description: "Village headquarters" },
    Location { id: "hospital", name: "Hospital", icon: "🏥", description: "Heal your wounds" },
    Location { id: "hot_springs", name: "Hot Springs", icon: "♨️", description: "Rest and recover" },
    Location { id: "training", name: "Training Ground", icon: "🎯", description: "Train your skills" },
    Location { id: "market", name: "Market", icon: "🛒", description: "Buy & sell items" },
    Location { id: "arena", name: "Arena", icon: "⚔️", description: "PvP battles" },
    Location { id: "gates", name: "Village Gates", icon: "🚪", description: "Leave village" },
];

fn get_location(id: &str) -> Option<&'static Location> {
    LOCATIONS.iter().find(|l| l.id == id)
}

/// Home page with 3-column layout
#[component]
pub fn HomePage() -> impl IntoView {
    // Shared state for current location
    let current_location = RwSignal::new("village_square".to_string());
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
                                    <div class="avatar-placeholder">"🥷"</div>
                                </div>
                                <div class="character-name">
                                    {move || player_info()
                                        .and_then(|p| p.username)
                                        .unwrap_or_else(|| "Anonymous Ninja".to_string())}
                                </div>
                                <div class="character-rank">"Genin"</div>
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
                                
                                // Chakra Bar
                                <StatBar 
                                    label="Chakra" 
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
                                        <span class="attr-icon">"⚡"</span>
                                        <span class="attr-name">"Agility"</span>
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

/// Center Panel - Village Map and Chat
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
                            <div class="welcome-icon">"🏯"</div>
                            <h1>"Welcome to Rust Shinobi"</h1>
                            <p>"Connect your wallet to begin your ninja journey"</p>
                            <div class="features">
                                <div class="feature">
                                    <span class="feature-icon">"⚔️"</span>
                                    <span>"Battle other ninjas"</span>
                                </div>
                                <div class="feature">
                                    <span class="feature-icon">"📜"</span>
                                    <span>"Complete missions"</span>
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
                            // Village Map (Top)
                            <VillageMap />
                            
                            // Chat (Bottom)
                            <VillageChat />
                        </>
                    }.into_any()
                }
            }}
        </div>
    }
}

/// Village Map with clickable locations overlaid on background image
#[component]
fn VillageMap() -> impl IntoView {
    let current_location = expect_context::<RwSignal<String>>();
    let location_name = move || {
        get_location(&current_location.get())
            .map(|l| l.name)
            .unwrap_or("Unknown")
    };
    
    view! {
        <div class="village-map-container">
            <div class="map-header">
                <h2 class="map-title">"🏯 Konoha Village"</h2>
                <span class="map-location">"📍 " {location_name}</span>
            </div>
            
            <div class="village-map-wrapper">
                // Background image
                <img 
                    src="/images/starting village.png" 
                    alt="Village Map" 
                    class="village-map-bg"
                />
                
                // Clickable location overlays
                <div class="map-locations-overlay">
                    <MapLocationOverlay 
                        id="hokage_tower"
                        name="Hokage Tower" 
                        icon="🏛️" 
                        top="15%"
                        left="50%"
                    />
                    <MapLocationOverlay 
                        id="academy"
                        name="Academy" 
                        icon="📚" 
                        top="25%"
                        left="20%"
                    />
                    <MapLocationOverlay 
                        id="hospital"
                        name="Hospital" 
                        icon="🏥" 
                        top="20%"
                        left="80%"
                    />
                    <MapLocationOverlay 
                        id="hot_springs"
                        name="Hot Springs" 
                        icon="♨️" 
                        top="50%"
                        left="15%"
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
                        left="25%"
                    />
                    <MapLocationOverlay 
                        id="arena"
                        name="Arena" 
                        icon="⚔️" 
                        top="75%"
                        left="50%"
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

/// Village Chat component
#[component]
fn VillageChat() -> impl IntoView {
    let chat_input = RwSignal::new(String::new());
    
    view! {
        <div class="village-chat">
            <div class="chat-header">
                <span class="chat-icon">"💬"</span>
                <span class="chat-title">"Village Chat"</span>
                <div class="chat-tabs">
                    <button class="chat-tab active">"Global"</button>
                    <button class="chat-tab">"Guild"</button>
                    <button class="chat-tab">"Private"</button>
                </div>
            </div>
            
            <div class="chat-messages">
                <ChatMessage 
                    username="ShadowBlade" 
                    message="Anyone want to team up for the forest mission?" 
                    time="2m ago"
                    rank="Chunin"
                />
                <ChatMessage 
                    username="FireFist" 
                    message="I just got a legendary kunai! 🎉" 
                    time="5m ago"
                    rank="Jonin"
                />
                <ChatMessage 
                    username="WindRunner" 
                    message="The arena event starts in 30 minutes" 
                    time="8m ago"
                    rank="Genin"
                />
                <ChatMessage 
                    username="System" 
                    message="Welcome to Konoha! New players can visit the Academy for tutorials." 
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
        "Jonin" => "rank-jonin",
        "Chunin" => "rank-chunin",
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
        "academy" => view! {
            <ActionButton 
                name="Physical Training" 
                description="Train Taijutsu" 
                cost="1 PA" 
                icon="💪"
                category="taijutsu"
            />
            <ActionButton 
                name="Ninjutsu Class" 
                description="Learn ninjutsu techniques" 
                cost="1 PA" 
                icon="🔥"
                category="ninjutsu"
            />
            <ActionButton 
                name="Genjutsu Class" 
                description="Study illusion arts" 
                cost="1 PA" 
                icon="👁️"
                category="genjutsu"
            />
            <ActionButton 
                name="Spar" 
                description="Practice combat" 
                cost="Free" 
                icon="⚔️"
                category="combat"
            />
            <ActionButton 
                name="Study Tactics" 
                description="Learn combat strategies" 
                cost="Free" 
                icon="📖"
                category="knowledge"
            />
        }.into_any(),
        
        "hokage_tower" => view! {
            <ActionButton 
                name="Request Mission" 
                description="Get a new mission" 
                cost="Free" 
                icon="📜"
                category="mission"
            />
            <ActionButton 
                name="Report Mission" 
                description="Complete mission" 
                cost="Free" 
                icon="✅"
                category="mission"
            />
            <ActionButton 
                name="Village Council" 
                description="View village affairs" 
                cost="Free" 
                icon="🏛️"
                category="social"
            />
        }.into_any(),
        
        "hospital" => view! {
            <ActionButton 
                name="Heal Wounds" 
                description="Restore HP" 
                cost="50 Ryo" 
                icon="💊"
                category="heal"
            />
            <ActionButton 
                name="Full Recovery" 
                description="Restore HP & Chakra" 
                cost="100 Ryo" 
                icon="🏥"
                category="heal"
            />
            <ActionButton 
                name="Medical Training" 
                description="Learn medical jutsu" 
                cost="1 PA" 
                icon="🩹"
                category="ninjutsu"
            />
        }.into_any(),
        
        "hot_springs" => view! {
            <ActionButton 
                name="Relax" 
                description="Recover chakra slowly" 
                cost="Free" 
                icon="♨️"
                category="rest"
            />
            <ActionButton 
                name="Meditate" 
                description="Increase chakra regen" 
                cost="1 PA" 
                icon="🧘"
                category="training"
            />
            <ActionButton 
                name="Socialize" 
                description="Meet other ninjas" 
                cost="Free" 
                icon="💬"
                category="social"
            />
        }.into_any(),
        
        "training" => view! {
            <ActionButton 
                name="Physical Training" 
                description="Increase Strength" 
                cost="1 PA" 
                icon="💪"
                category="taijutsu"
            />
            <ActionButton 
                name="Speed Training" 
                description="Increase Agility" 
                cost="1 PA" 
                icon="⚡"
                category="taijutsu"
            />
            <ActionButton 
                name="Chakra Control" 
                description="Increase max chakra" 
                cost="1 PA" 
                icon="🔵"
                category="ninjutsu"
            />
            <ActionButton 
                name="Target Practice" 
                description="Improve accuracy" 
                cost="1 PA" 
                icon="🎯"
                category="combat"
            />
        }.into_any(),
        
        "market" => view! {
            <ActionButton 
                name="Buy Items" 
                description="Purchase equipment" 
                cost="Free" 
                icon="🛒"
                category="shop"
            />
            <ActionButton 
                name="Sell Items" 
                description="Sell your items" 
                cost="Free" 
                icon="💰"
                category="shop"
            />
            <ActionButton 
                name="Black Market" 
                description="Rare items..." 
                cost="Free" 
                icon="🕶️"
                category="shop"
            />
        }.into_any(),
        
        "arena" => view! {
            <ActionButton 
                name="Quick Match" 
                description="Fight random opponent" 
                cost="Free" 
                icon="⚔️"
                category="combat"
            />
            <ActionButton 
                name="Ranked Battle" 
                description="Competitive match" 
                cost="1 PA" 
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
                name="Spectate" 
                description="Watch battles" 
                cost="Free" 
                icon="👀"
                category="social"
            />
        }.into_any(),
        
        "gates" => view! {
            <ActionButton 
                name="Patrol Mission" 
                description="Guard the gates" 
                cost="1 PA" 
                icon="🛡️"
                category="mission"
            />
            <ActionButton 
                name="Leave Village" 
                description="Go on expedition" 
                cost="Free" 
                icon="🚪"
                category="travel"
            />
            <ActionButton 
                name="World Map" 
                description="View other locations" 
                cost="Free" 
                icon="🗺️"
                category="travel"
            />
        }.into_any(),
        
        // Village Square (default)
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
                name="Check Missions" 
                description="View active missions" 
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
