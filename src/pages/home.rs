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

/// Center Panel - Main Content Area
#[component]
fn CenterContent() -> impl IntoView {
    let wallet = use_wallet();
    let is_connected = move || wallet.get().connected;
    let is_new_player = move || wallet.get().is_new_player;
    
    view! {
        <div class="center-content">
            <h2 class="panel-title">"Village Square"</h2>
            
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
                } else if is_new_player() {
                    view! {
                        <div class="new-player-screen">
                            <div class="welcome-icon">"🎉"</div>
                            <h1>"Welcome, New Ninja!"</h1>
                            <p>"Your journey begins here. Start by exploring the village."</p>
                            <div class="quick-start">
                                <h3>"Getting Started:"</h3>
                                <ul>
                                    <li>"Visit the Training Grounds to learn basic jutsu"</li>
                                    <li>"Check the Mission Board for your first task"</li>
                                    <li>"Explore the Shop for equipment"</li>
                                </ul>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="dashboard">
                            <div class="activity-feed">
                                <h3>"Recent Activity"</h3>
                                <div class="activity-item">
                                    <span class="activity-icon">"📢"</span>
                                    <span>"Welcome back to the village!"</span>
                                </div>
                                <div class="activity-item">
                                    <span class="activity-icon">"⚔️"</span>
                                    <span>"The Arena is open for battles"</span>
                                </div>
                                <div class="activity-item">
                                    <span class="activity-icon">"📜"</span>
                                    <span>"New missions available"</span>
                                </div>
                            </div>
                        </div>
                    }.into_any()
                }
            }}
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

