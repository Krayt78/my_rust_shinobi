use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use crate::wallet::{WalletProvider, ConnectWalletButton, context::use_wallet};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
                // Google Fonts for ninja-style typography
                <link rel="preconnect" href="https://fonts.googleapis.com"/>
                <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous"/>
                <link href="https://fonts.googleapis.com/css2?family=Rajdhani:wght@400;500;600;700&family=Orbitron:wght@400;500;600;700&display=swap" rel="stylesheet"/>
                // Bundled Polkadot wallet JavaScript
                <script src="/wallet.js"></script>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/my_rust_shinobi.css"/>
        <Title text="My Rust Shinobi"/>
        
        <WalletProvider>
            <Router>
                <div class="game-container">
                    // Top Navigation Bar
                    <TopNavBar />
                    
                    // Main Content Area
                    <main class="main-content">
                        <Routes fallback=|| "Page not found.".into_view()>
                            <Route path=StaticSegment("") view=HomePage/>
                        </Routes>
                    </main>
                </div>
            </Router>
        </WalletProvider>
    }
}

/// Top Navigation Bar with game menus
#[component]
fn TopNavBar() -> impl IntoView {
    view! {
        <header class="top-nav">
            <div class="nav-logo">
                <span class="logo-icon">"忍"</span>
                <span class="logo-text">"Rust Shinobi"</span>
            </div>
            
            <nav class="nav-menu">
                <a href="/" class="nav-item active">"🏠 Home"</a>
                <a href="/character" class="nav-item">"👤 Character"</a>
                <a href="/village" class="nav-item">"🏯 Village"</a>
                <a href="/missions" class="nav-item">"📜 Missions"</a>
                <a href="/arena" class="nav-item">"⚔️ Arena"</a>
                <a href="/shop" class="nav-item">"🛒 Shop"</a>
                <a href="/guild" class="nav-item">"🏴 Guild"</a>
            </nav>
            
            <div class="nav-wallet">
                <ConnectWalletButton />
            </div>
        </header>
    }
}

/// Home page with 3-column layout
#[component]
fn HomePage() -> impl IntoView {
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
                <RightPanel />
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

/// Stat bar component
#[component]
fn StatBar(
    label: &'static str,
    current: i32,
    max: i32,
    color: &'static str,
) -> impl IntoView {
    let percentage = if max > 0 { (current as f64 / max as f64) * 100.0 } else { 0.0 };
    
    view! {
        <div class="stat-bar">
            <div class="stat-label">
                <span>{label}</span>
                <span class="stat-values">{current}"/" {max}</span>
            </div>
            <div class="stat-bar-bg">
                <div 
                    class="stat-bar-fill"
                    style:width=format!("{}%", percentage)
                    style:background-color=color
                />
            </div>
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
fn RightPanel() -> impl IntoView {
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