use leptos::prelude::*;
use super::WalletAccount;

#[derive(Clone, Debug, Default)]
pub struct WalletState {
    pub connected: bool,
    pub accounts: Vec<WalletAccount>,
    pub selected_account: Option<WalletAccount>,
    pub error: Option<String>,
    pub loading: bool,
}

/// Provide wallet context to child components
#[component]
pub fn WalletProvider(children: Children) -> impl IntoView {
    let wallet_state = RwSignal::new(WalletState::default());
    
    provide_context(wallet_state);
    
    children()
}

/// Hook to access wallet state
pub fn use_wallet() -> RwSignal<WalletState> {
    expect_context::<RwSignal<WalletState>>()
}