//! Character page - Character stats, equipment, and inventory management

use leptos::prelude::*;
use crate::wallet::context::use_wallet;
use crate::components::StatBar;

/// Character page with 3-panel layout: Stats | Equipment | Inventory
#[component]
pub fn CharacterPage() -> impl IntoView {
    let wallet = use_wallet();
    let is_connected = move || wallet.get().connected;
    
    view! {
        <div class="character-page">
            {move || {
                if is_connected() {
                    view! {
                        <div class="character-layout">
                            // Left Panel - Character Stats
                            <aside class="character-stats-panel">
                                <StatsPanel />
                            </aside>
                            
                            // Center Panel - Character Model & Equipment
                            <section class="character-equipment-panel">
                                <EquipmentPanel />
                            </section>
                            
                            // Right Panel - Inventory
                            <aside class="character-inventory-panel">
                                <InventoryPanel />
                            </aside>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="not-connected-full">
                            <div class="lock-icon">"🔒"</div>
                            <h2>"Character Locked"</h2>
                            <p>"Connect your wallet to view your character"</p>
                        </div>
                    }.into_any()
                }
            }}
        </div>
    }
}

/// Left Panel - All character stats
#[component]
fn StatsPanel() -> impl IntoView {
    let wallet = use_wallet();
    let player_info = move || wallet.get().player.clone();
    
    view! {
        <div class="stats-panel">
            // Character Header
            <div class="character-header">
                <div class="character-name-level">
                    <span class="char-name">
                        {move || player_info()
                            .and_then(|p| p.username)
                            .unwrap_or_else(|| "Wandering Hero".to_string())}
                    </span>
                    <span class="char-level">"Lv. 1"</span>
                </div>
            </div>
            
            // Resource Bars
            <div class="resource-bars">
                <StatBar label="EXP" current=0 max=100 color="#f1c40f" />
                <StatBar label="HP" current=100 max=100 color="#e74c3c" />
                <StatBar label="Mana" current=50 max=50 color="#3498db" />
            </div>
            
            // Character Details
            <div class="details-section">
                <h3 class="section-title">"Details"</h3>
                <div class="detail-row">
                    <span class="detail-icon">"🏰"</span>
                    <span class="detail-label">"Kingdom"</span>
                    <span class="detail-value">"Eldoria"</span>
                </div>
                <div class="detail-row">
                    <span class="detail-icon">"⚔️"</span>
                    <span class="detail-label">"Class"</span>
                    <span class="detail-value">"Adventurer"</span>
                </div>
                <div class="detail-row">
                    <span class="detail-icon">"🎭"</span>
                    <span class="detail-label">"Specialty"</span>
                    <span class="detail-value">"Warrior"</span>
                </div>
            </div>
            
            // Primary Stats (D&D style)
            <div class="primary-stats-section">
                <h3 class="section-title">"Attributes"</h3>
                <div class="stat-row">
                    <span class="stat-icon">"💪"</span>
                    <span class="stat-name">"Strength"</span>
                    <span class="stat-value">"10"</span>
                </div>
                <div class="stat-row">
                    <span class="stat-icon">"🏃"</span>
                    <span class="stat-name">"Dexterity"</span>
                    <span class="stat-value">"12"</span>
                </div>
                <div class="stat-row">
                    <span class="stat-icon">"❤️"</span>
                    <span class="stat-name">"Constitution"</span>
                    <span class="stat-value">"8"</span>
                </div>
                <div class="stat-row">
                    <span class="stat-icon">"🧠"</span>
                    <span class="stat-name">"Intelligence"</span>
                    <span class="stat-value">"15"</span>
                </div>
                <div class="stat-row">
                    <span class="stat-icon">"👁️"</span>
                    <span class="stat-name">"Wisdom"</span>
                    <span class="stat-value">"11"</span>
                </div>
                <div class="stat-row">
                    <span class="stat-icon">"✨"</span>
                    <span class="stat-name">"Charisma"</span>
                    <span class="stat-value">"7"</span>
                </div>
            </div>
            
            // Combat Stats
            <div class="combat-stats-section">
                <h3 class="section-title">"Combat"</h3>
                <div class="stat-row">
                    <span class="stat-name">"Attack"</span>
                    <span class="stat-value combat">"45"</span>
                </div>
                <div class="stat-row">
                    <span class="stat-name">"Defense"</span>
                    <span class="stat-value combat">"32"</span>
                </div>
                <div class="stat-row">
                    <span class="stat-name">"Speed"</span>
                    <span class="stat-value combat">"28"</span>
                </div>
                <div class="stat-row">
                    <span class="stat-name">"Crit Rate"</span>
                    <span class="stat-value combat">"5.2%"</span>
                </div>
            </div>
        </div>
    }
}

/// Center Panel - Character model and equipment slots
#[component]
fn EquipmentPanel() -> impl IntoView {
    view! {
        <div class="equipment-panel">
            <h2 class="panel-title">"Equipment"</h2>
            
            <div class="equipment-grid">
                // Top row - Head
                <div class="equipment-row top-row">
                    <EquipmentSlot slot_type="head" icon="🪖" label="Helm" />
                </div>
                
                // Second row - Shoulders
                <div class="equipment-row shoulder-row">
                    <EquipmentSlot slot_type="shoulder-l" icon="🛡️" label="L.Pauldron" />
                    <div class="spacer"></div>
                    <EquipmentSlot slot_type="shoulder-r" icon="🛡️" label="R.Pauldron" />
                </div>
                
                // Main row - Weapon, Character, Off-hand
                <div class="equipment-row main-row">
                    <EquipmentSlot slot_type="weapon" icon="⚔️" label="Weapon" />
                    
                    <div class="character-model">
                        <div class="model-frame">
                            <div class="ninja-avatar">"🧙‍♂️"</div>
                        </div>
                    </div>
                    
                    <EquipmentSlot slot_type="offhand" icon="🛡️" label="Shield" />
                </div>
                
                // Armor row - Chest, Hands
                <div class="equipment-row armor-row">
                    <EquipmentSlot slot_type="chest" icon="🎽" label="Chestplate" />
                    <EquipmentSlot slot_type="hands" icon="🧤" label="Gauntlets" />
                </div>
                
                // Bottom row - Legs, Feet
                <div class="equipment-row bottom-row">
                    <EquipmentSlot slot_type="legs" icon="👖" label="Greaves" />
                    <EquipmentSlot slot_type="feet" icon="🥾" label="Boots" />
                </div>
                
                // Accessory row
                <div class="equipment-row accessory-row">
                    <EquipmentSlot slot_type="ring1" icon="💍" label="Ring" />
                    <EquipmentSlot slot_type="amulet" icon="📿" label="Amulet" />
                    <EquipmentSlot slot_type="ring2" icon="💍" label="Ring" />
                </div>
            </div>
            
            // Equipment Stats Summary
            <div class="equipment-summary">
                <div class="summary-item">
                    <span class="summary-icon">"⚖️"</span>
                    <span class="summary-label">"Weight"</span>
                    <span class="summary-value">"12/50 kg"</span>
                </div>
                <div class="summary-item">
                    <span class="summary-icon">"🛡️"</span>
                    <span class="summary-label">"Armor Class"</span>
                    <span class="summary-value">"15"</span>
                </div>
            </div>
        </div>
    }
}

/// Individual equipment slot component
#[component]
fn EquipmentSlot(
    slot_type: &'static str,
    icon: &'static str,
    label: &'static str,
) -> impl IntoView {
    view! {
        <div class=format!("equipment-slot {}", slot_type)>
            <div class="slot-frame">
                <span class="slot-icon">{icon}</span>
            </div>
            <span class="slot-label">{label}</span>
        </div>
    }
}

/// Right Panel - Inventory grid
#[component]
fn InventoryPanel() -> impl IntoView {
    // Inventory tabs
    let active_tab = RwSignal::new("all");
    
    view! {
        <div class="inventory-panel">
            <h2 class="panel-title">"Inventory"</h2>
            
            // Inventory Tabs
            <div class="inventory-tabs">
                <button 
                    class=move || if active_tab.get() == "all" { "tab-btn active" } else { "tab-btn" }
                    on:click=move |_| active_tab.set("all")
                >"All"</button>
                <button 
                    class=move || if active_tab.get() == "weapons" { "tab-btn active" } else { "tab-btn" }
                    on:click=move |_| active_tab.set("weapons")
                >"⚔️"</button>
                <button 
                    class=move || if active_tab.get() == "armor" { "tab-btn active" } else { "tab-btn" }
                    on:click=move |_| active_tab.set("armor")
                >"🛡️"</button>
                <button 
                    class=move || if active_tab.get() == "consumables" { "tab-btn active" } else { "tab-btn" }
                    on:click=move |_| active_tab.set("consumables")
                >"🧪"</button>
                <button 
                    class=move || if active_tab.get() == "materials" { "tab-btn active" } else { "tab-btn" }
                    on:click=move |_| active_tab.set("materials")
                >"📦"</button>
            </div>
            
            // Search
            <div class="inventory-search">
                <input type="text" placeholder="Search items..." class="search-input" />
            </div>
            
            // Inventory Grid (8 columns)
            <div class="inventory-grid">
                // Sample items - Fantasy themed items - Row 1
                <InventorySlot icon="🍞" quantity=5 rarity="common" />
                <InventorySlot icon="🧪" quantity=3 rarity="common" />
                <InventorySlot icon="📜" quantity=1 rarity="uncommon" />
                <InventorySlot icon="💣" quantity=10 rarity="common" />
                <InventorySlot icon="🗡️" quantity=1 rarity="rare" />
                <InventorySlot icon="🪄" quantity=1 rarity="epic" />
                <InventorySlot icon="👑" quantity=1 rarity="legendary" />
                <InventorySlot icon="🧪" quantity=8 rarity="common" />
                
                // Row 2
                <InventorySlot icon="🛡️" quantity=1 rarity="uncommon" />
                <InventorySlot icon="🎽" quantity=1 rarity="rare" />
                <InventorySlot icon="🩹" quantity=15 rarity="common" />
                <InventorySlot icon="📿" quantity=1 rarity="epic" />
                <EmptySlot />
                <EmptySlot />
                <EmptySlot />
                <EmptySlot />
                
                // Rows 3-6 (Empty slots)
                <EmptySlot /><EmptySlot /><EmptySlot /><EmptySlot /><EmptySlot /><EmptySlot /><EmptySlot /><EmptySlot />
                <EmptySlot /><EmptySlot /><EmptySlot /><EmptySlot /><EmptySlot /><EmptySlot /><EmptySlot /><EmptySlot />
                <EmptySlot /><EmptySlot /><EmptySlot /><EmptySlot /><EmptySlot /><EmptySlot /><EmptySlot /><EmptySlot />
                <EmptySlot /><EmptySlot /><EmptySlot /><EmptySlot /><EmptySlot /><EmptySlot /><EmptySlot /><EmptySlot />
            </div>
            
            // Inventory Footer
            <div class="inventory-footer">
                <div class="inventory-weight">
                    <span class="weight-icon">"⚖️"</span>
                    <span class="weight-text">"24/100"</span>
                </div>
                <div class="inventory-currency">
                    <span class="currency gold">"🪙 1,250 Gold"</span>
                </div>
            </div>
        </div>
    }
}

/// Inventory slot with an item
#[component]
fn InventorySlot(
    icon: &'static str,
    quantity: i32,
    rarity: &'static str,
) -> impl IntoView {
    view! {
        <div class=format!("inventory-slot item {}", rarity)>
            <span class="item-icon">{icon}</span>
            {if quantity > 1 {
                Some(view! { <span class="item-quantity">{quantity}</span> })
            } else {
                None
            }}
        </div>
    }
}

/// Empty inventory slot
#[component]
fn EmptySlot() -> impl IntoView {
    view! {
        <div class="inventory-slot empty"></div>
    }
}
