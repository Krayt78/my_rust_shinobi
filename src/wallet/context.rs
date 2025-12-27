use leptos::prelude::*;
use super::WalletAccount;
use crate::api::PlayerInfo;

#[derive(Clone, Debug, Default)]
pub struct WalletState {
    pub connected: bool,
    pub accounts: Vec<WalletAccount>,
    pub selected_account: Option<WalletAccount>,
    pub error: Option<String>,
    pub loading: bool,
    /// Player info from the database (set after authentication)
    pub player: Option<PlayerInfo>,
    /// Whether this is a new player (just registered)
    pub is_new_player: bool,
}

/// Provide wallet context to child components
#[component]
pub fn WalletProvider(children: Children) -> impl IntoView {
    let wallet_state = RwSignal::new(WalletState::default());
    
    provide_context(wallet_state);
    
    children()
}

/// Hook to access wallet state - returns Option to handle cases where context isn't available
pub fn use_wallet() -> RwSignal<WalletState> {
    use_context::<RwSignal<WalletState>>()
        .unwrap_or_else(|| {
            // Create a default signal if context not found (during SSR)
            RwSignal::new(WalletState::default())
        })
}

/// Hook to try to access wallet state - returns None if not in WalletProvider
pub fn try_use_wallet() -> Option<RwSignal<WalletState>> {
    use_context::<RwSignal<WalletState>>()
}