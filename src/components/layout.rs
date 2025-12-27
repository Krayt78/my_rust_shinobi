//! Layout components - Navigation, headers, footers

use leptos::prelude::*;
use crate::wallet::ConnectWalletButton;

/// Top Navigation Bar with game menus
#[component]
pub fn TopNavBar() -> impl IntoView {
    view! {
        <header class="top-nav">
            <div class="nav-logo">
                <span class="logo-icon">"⚔️"</span>
                <span class="logo-text">"Realm of Legends"</span>
            </div>
            
            <nav class="nav-menu">
                <a href="/" class="nav-item active">"🏠 Home"</a>
                <a href="/character" class="nav-item">"🧙 Character"</a>
                <a href="/quests" class="nav-item">"📜 Quests"</a>
            </nav>
            
            <div class="nav-wallet">
                <ConnectWalletButton />
            </div>
        </header>
    }
}
