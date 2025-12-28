//! Layout components - Navigation, headers, footers

use leptos::prelude::*;
use leptos_router::hooks::use_location;

use crate::wallet::ConnectWalletButton;

//TODO: move elsewhere
#[derive(Debug, PartialEq)]
enum Page {
    Home,
    Character,
    Quests,
}

impl Page {
    fn path(&self) -> String {
        match self {
            Page::Home => "/".to_string(),
            Page::Character => "/character".to_string(),
            Page::Quests => "/quests".to_string(),
        }
    }
}

/// Top Navigation Bar with game menus
#[component]
pub fn TopNavBar() -> impl IntoView {
    let location = use_location();
    let is_home = move || location.pathname.get() == "/";
    let is_quests = move || location.pathname.get() == "/quests";
    let is_character = move || location.pathname.get() == "/character";

    let current_page = move || {
        if is_home() {
            Page::Home
        } else if is_quests() {
            Page::Quests
        } else if is_character() {
            Page::Character
        } else {
            Page::Home
        }
    };

    println!("{}", location.pathname.get_untracked());

    view! {
        <header class="top-nav">
            <div class="nav-logo">
                <span class="logo-icon">"⚔️"</span>
                <span class="logo-text">"Realm of Legends"</span>
            </div>

            <nav class="nav-menu">
                <a href="/" class=move || format!("nav-item{}", if is_home() { " active" } else { "" })>"🏠 Home"</a>
                <a href="/character" class=move || format!("nav-item{}", if is_character() { " active" } else { "" })>"🧙 Character"</a>
                <a href="/quests" class=move || format!("nav-item{}", if is_quests() { " active" } else { "" })>"📜 Quests"</a>
            </nav>

            <div class="nav-wallet">
                <ConnectWalletButton />
            </div>
        </header>
    }
}
