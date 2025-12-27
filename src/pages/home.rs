//! Home page - The main dashboard for players

use leptos::prelude::*;
use crate::wallet::context::use_wallet;
use crate::components::StatBar;

/// Home page with 3-column layout
#[component]
pub fn HomePage() -> impl IntoView {
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
            
            // Right Panel - Actions/Info (25%)
            <aside class="panel right-panel">
                <QuickActionsPanel />
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
    view! {
        <div class="village-map-container">
            <div class="map-header">
                <h2 class="map-title">"🏯 Konoha Village"</h2>
                <span class="map-location">"Current: Village Square"</span>
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
                    // Hokage Tower (top center)
                    <MapLocationOverlay 
                        name="Hokage Tower" 
                        icon="🏛️" 
                        top="15%"
                        left="50%"
                        description="Village headquarters"
                    />
                    
                    // Academy (top left area)
                    <MapLocationOverlay 
                        name="Academy" 
                        icon="📚" 
                        top="25%"
                        left="20%"
                        description="Learn new jutsu"
                    />
                    
                    // Hospital (top right)
                    <MapLocationOverlay 
                        name="Hospital" 
                        icon="🏥" 
                        top="20%"
                        left="80%"
                        description="Heal your wounds"
                    />
                    
                    // Hot Springs (left side)
                    <MapLocationOverlay 
                        name="Hot Springs" 
                        icon="♨️" 
                        top="50%"
                        left="15%"
                        description="Rest and recover"
                    />
                    
                    // Training Ground (right side)
                    <MapLocationOverlay 
                        name="Training" 
                        icon="🎯" 
                        top="45%"
                        left="85%"
                        description="Train your skills"
                    />
                    
                    // Market (bottom left)
                    <MapLocationOverlay 
                        name="Market" 
                        icon="🛒" 
                        top="70%"
                        left="25%"
                        description="Buy & sell items"
                    />
                    
                    // Arena (bottom center)
                    <MapLocationOverlay 
                        name="Arena" 
                        icon="⚔️" 
                        top="75%"
                        left="50%"
                        description="PvP battles"
                    />
                    
                    // Village Gates (bottom)
                    <MapLocationOverlay 
                        name="Gates" 
                        icon="🚪" 
                        top="85%"
                        left="50%"
                        description="Leave village"
                    />
                </div>
            </div>
        </div>
    }
}

/// Individual map location overlay (positioned absolutely on the map)
#[component]
fn MapLocationOverlay(
    name: &'static str,
    icon: &'static str,
    top: &'static str,
    left: &'static str,
    description: &'static str,
) -> impl IntoView {
    view! {
        <a 
            href="#" 
            class="map-location-overlay"
            style=format!("top: {}; left: {};", top, left)
            title=description
        >
            <div class="location-marker-icon">{icon}</div>
            <span class="location-label">{name}</span>
        </a>
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

/// Right Panel - Quick Actions and Info
#[component]
fn QuickActionsPanel() -> impl IntoView {
    let wallet = use_wallet();
    let is_connected = move || wallet.get().connected;
    
    view! {
        <div class="right-panel">
            <h2 class="panel-title">"Quick Actions"</h2>
            
            {move || {
                if is_connected() {
                    view! {
                        <div class="quick-actions">
                            <button class="action-btn">"🎯 Train"</button>
                            <button class="action-btn">"📜 Missions"</button>
                            <button class="action-btn">"⚔️ Battle"</button>
                            <button class="action-btn">"🛒 Shop"</button>
                        </div>
                        
                        <div class="info-section">
                            <h3>"Village Info"</h3>
                            <div class="info-item">
                                <span class="info-label">"Players Online:"</span>
                                <span class="info-value">"42"</span>
                            </div>
                            <div class="info-item">
                                <span class="info-label">"Village:"</span>
                                <span class="info-value">"Konoha"</span>
                            </div>
                            <div class="info-item">
                                <span class="info-label">"Rank:"</span>
                                <span class="info-value">"Genin"</span>
                            </div>
                        </div>
                        
                        <div class="online-players">
                            <h3>"Online Ninjas"</h3>
                            <div class="player-list">
                                <div class="player-item">"🥷 ShadowBlade"</div>
                                <div class="player-item">"🥷 FireFist"</div>
                                <div class="player-item">"🥷 WindRunner"</div>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="not-connected">
                            <p>"Connect wallet to see quick actions"</p>
                        </div>
                    }.into_any()
                }
            }}
        </div>
    }
}

