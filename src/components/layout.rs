//! Layout components - Navigation, headers, footers

use leptos::prelude::*;
use crate::wallet::ConnectWalletButton;

/// Top Navigation Bar with game menus
#[component]
pub fn TopNavBar() -> impl IntoView {
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

