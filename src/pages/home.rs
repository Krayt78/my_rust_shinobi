//! Home page - The main dashboard for players

use crate::api::{get_location_by_id, get_town_by_id, get_locations_by_town, get_actions_by_location, LocationInfo, TownInfo, ActionInfo};
use crate::components::StatBar;
use crate::wallet::context::use_wallet;
use leptos::prelude::*;

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
    Location {
        id: "town_square",
        name: "Town Square",
        icon: "🏰",
        description: "The heart of the town",
    },
    Location {
        id: "guild_hall",
        name: "Adventurer's Guild",
        icon: "⚔️",
        description: "Accept quests",
    },
    Location {
        id: "castle",
        name: "Castle",
        icon: "👑",
        description: "The royal court",
    },
    Location {
        id: "temple",
        name: "Temple",
        icon: "⛪",
        description: "Heal and pray",
    },
    Location {
        id: "tavern",
        name: "Tavern",
        icon: "🍺",
        description: "Rest and rumors",
    },
    Location {
        id: "training",
        name: "Training Grounds",
        icon: "🎯",
        description: "Hone your skills",
    },
    Location {
        id: "market",
        name: "Marketplace",
        icon: "🛒",
        description: "Buy & sell goods",
    },
    Location {
        id: "arena",
        name: "Colosseum",
        icon: "🏟️",
        description: "PvP battles",
    },
    Location {
        id: "gates",
        name: "Town Gates",
        icon: "🚪",
        description: "Leave town",
    },
    Location {
        id: "wizard_tower",
        name: "Wizard Tower",
        icon: "🗼",
        description: "Learn magic",
    },
    Location {
        id: "blacksmith",
        name: "Blacksmith",
        icon: "🔨",
        description: "Forge equipment",
    },
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

    // Fetch location from database using server function
    // The location ID: b0000000-0000-0000-0000-000000000001
    let location_id = "b0000000-0000-0000-0000-000000000001".to_string();
    let location_id_for_resource = location_id.clone();
    
    // Use Resource to fetch location data asynchronously
    let location_resource = Resource::new(
        move || location_id_for_resource.clone(),
        move |id: String| {
            let id = id.clone();
            async move {
                get_location_by_id(id).await
            }
        },
    );

    // No Effect needed! The Resource will automatically trigger updates
    // when the data loads. Components that use it will reactively update.

    view! {
        <div class="three-column-layout">
            // Left Panel - Character Stats (25%)
            <aside class="panel left-panel">
                <CharacterPanel />
            </aside>

            // Center Panel - Main Content (50%)
            <section class="panel center-panel">
                <CenterContent location_id=location_id />
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
fn CenterContent(location_id: String) -> impl IntoView {
    let wallet = use_wallet();
    let is_connected = move || wallet.get().connected;
    let location_id_clone = location_id.clone();

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
                            <KingdomMap location_id=location_id_clone.clone() />

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
fn KingdomMap(location_id: String) -> impl IntoView {
    // Create resource to fetch location
    let location_resource = Resource::new(
        move || location_id.clone(),
        move |id: String| {
            let id = id.clone();
            async move {
                get_location_by_id(id).await
            }
        },
    );
    
    // Create a signal to track town_id, updated when location loads
    let town_id_signal = RwSignal::new(None::<String>);
    
    // Create a resource that depends on the town_id signal
    let town_resource = Resource::new(
        move || town_id_signal.get(),
        move |town_id: Option<String>| {
            async move {
                if let Some(town_id) = town_id {
                    get_town_by_id(town_id).await
                } else {
                    Ok(None)
                }
            }
        },
    );
    
    // Create a resource to fetch locations for the town
    let locations_resource = Resource::new(
        move || town_id_signal.get(),
        move |town_id: Option<String>| {
            async move {
                if let Some(town_id) = town_id {
                    get_locations_by_town(town_id).await
                } else {
                    Ok(Vec::new())
                }
            }
        },
    );
    
    view! {
        <div class="village-map-container">
            <div class="map-header">
                <h2 class="map-title">
                    {move || {
                        match town_resource.get() {
                            Some(Ok(Some(town))) => format!("🏰 {}", town.name),
                            _ => "🏰 Default Kingdom".to_string(),
                        }
                    }}
                </h2>
                <span class="map-location">
                    "📍 " 
                    {move || {
                        // Access location_resource and extract name, also update town_id
                        if let Some(result) = location_resource.get() {
                            if let Ok(Some(location)) = result {
                                town_id_signal.set(Some(location.town_id.clone()));
                                location.name
                            } else {
                                "Unknown".to_string()
                            }
                        } else {
                            "Loading...".to_string()
                        }
                    }}
                </span>
            </div>

            <div class="village-map-wrapper">
                // Background image - use town's map_image if available
                <img
                    src=move || {
                        match town_resource.get() {
                            Some(Ok(Some(town))) => town.map_image.unwrap_or_else(|| "/images/starting village.png".to_string()),
                            _ => "/images/starting village.png".to_string(),
                        }
                    }
                    alt="Kingdom Map"
                    class="village-map-bg"
                />

                // Clickable location overlays - dynamically generated from database
                <div class="map-locations-overlay">
                    {move || {
                        match locations_resource.get() {
                            Some(Ok(locations)) => {
                                if locations.is_empty() {
                                    view! { <></> }.into_any()
                                } else {
                                    locations.into_iter().map(|loc| {
                                        let loc_id = loc.id.clone();
                                        let loc_name = loc.name.clone();
                                        let loc_icon = loc.icon.clone();
                                        let top_pos = format!("{}%", loc.map_position_y);
                                        let left_pos = format!("{}%", loc.map_position_x);
                                        view! {
                                            <MapLocationOverlay
                                                id=loc_id
                                                name=loc_name
                                                icon=loc_icon
                                                top=top_pos
                                                left=left_pos
                                            />
                                        }
                                    }).collect::<Vec<_>>().into_any()
                                }
                            }
                            _ => view! { <></> }.into_any()
                        }
                    }}
                </div>
            </div>
        </div>
    }
}

/// Individual map location overlay (positioned absolutely on the map)
#[component]
fn MapLocationOverlay(
    id: String,
    name: String,
    icon: String,
    top: String,
    left: String,
) -> impl IntoView {
    let current_location = expect_context::<RwSignal<String>>();
    let id_clone = id.clone();
    let is_current = move || current_location.get() == id_clone;

    view! {
        <button
            class=move || if is_current() { "map-location-overlay active" } else { "map-location-overlay" }
            style=format!("top: {}; left: {};", top, left)
            on:click=move |_| current_location.set(id.clone())
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

    // Fetch location from database using the current_location signal
    let location_resource = Resource::new(
        move || current_location.get(),
        move |location_id: String| {
            async move {
                // Try to fetch from database - if location_id is a UUID, it will work
                // If it's an old static string ID, the server will return an error which we handle
                get_location_by_id(location_id).await
            }
        },
    );

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
                    view! {
                        <>
                            // Location header
                            <div class="location-header">
                                <div class="location-image">
                                    <span class="location-icon-large">
                                        {move || {
                                            match location_resource.get() {
                                                Some(Ok(Some(location))) => location.icon,
                                                _ => "📍".to_string(),
                                            }
                                        }}
                                    </span>
                                </div>
                                <div class="location-title">
                                    <span class="location-marker-text">"📍 at "</span>
                                    <span class="location-name-text">
                                        {move || {
                                            match location_resource.get() {
                                                Some(Ok(Some(location))) => location.name,
                                                _ => "Unknown".to_string(),
                                            }
                                        }}
                                    </span>
                                </div>
                            </div>

                            // Actions based on location
                            <div class="location-actions">
                                {move || {
                                    match location_resource.get() {
                                        Some(Ok(Some(location))) => {
                                            view! {
                                                <LocationActions location_id=location.id />
                                            }.into_any()
                                        }
                                        _ => view! {
                                            <div class="no-actions">
                                                <p>"No location selected"</p>
                                            </div>
                                        }.into_any()
                                    }
                                }}
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
    // Fetch actions from database
    let actions_resource = Resource::new(
        move || location_id.clone(),
        move |id: String| {
            async move {
                get_actions_by_location(id).await
            }
        },
    );

    view! {
        {move || {
            match actions_resource.get() {
                Some(Ok(actions)) => {
                    if actions.is_empty() {
                        view! {
                            <div class="no-actions">
                                <p>"No actions available at this location"</p>
                            </div>
                        }.into_any()
                    } else {
                        actions.into_iter().map(|action| {
                            // Format cost string based on action requirements
                            let cost = if action.required_gold > 0 && action.action_points_cost > 0 {
                                format!("{} Gold + {} AP", action.required_gold, action.action_points_cost)
                            } else if action.required_gold > 0 {
                                format!("{} Gold", action.required_gold)
                            } else if action.action_points_cost > 0 {
                                format!("{} AP", action.action_points_cost)
                            } else {
                                "Free".to_string()
                            };

                            let action_name = action.name.clone();
                            let action_desc = action.description.unwrap_or_else(|| "".to_string());
                            let action_icon = action.icon.clone();
                            let action_category = action.category.clone().to_lowercase();

                            view! {
                                <ActionButton
                                    name=action_name
                                    description=action_desc
                                    cost=cost
                                    icon=action_icon
                                    category=action_category
                                />
                            }
                        }).collect::<Vec<_>>().into_any()
                    }
                }
                Some(Err(_)) => view! {
                    <div class="error">
                        <p>"Failed to load actions"</p>
                    </div>
                }.into_any(),
                _ => view! {
                    <div class="loading">
                        <p>"Loading actions..."</p>
                    </div>
                }.into_any()
            }
        }}
    }
}

/// Action button component
#[component]
fn ActionButton(
    name: String,
    description: String,
    cost: String,
    icon: String,
    category: String,
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
