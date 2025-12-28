use super::{connect_polkadot_wallet, context::use_wallet, WalletAccount};
use crate::api::authenticate_player;
use leptos::prelude::*;

#[component]
pub fn ConnectWalletButton() -> impl IntoView {
    let wallet = use_wallet();

    // Use Action::new_local for non-Send futures (JavaScript futures)
    let connect_action = Action::new_local(move |_: &()| async move {
        // Set loading state
        wallet.update(|w| {
            w.loading = true;
            w.error = None;
        });

        // Step 1: Connect to wallet extension
        match connect_polkadot_wallet("Realm of Legends").await {
            Ok(accounts) => {
                let selected_account = accounts.first().cloned();

                wallet.update(|w| {
                    w.accounts = accounts.clone();
                    w.selected_account = selected_account.clone();
                });

                // Step 2: Authenticate with the server
                if let Some(account) = selected_account {
                    match authenticate_player(account.address.clone()).await {
                        Ok(player_info) => {
                            let is_new = player_info.is_new;
                            leptos::logging::log!(
                                "Player authenticated: {} (new: {})",
                                player_info.id,
                                is_new
                            );

                            wallet.update(|w| {
                                w.loading = false;
                                w.connected = true;
                                w.is_new_player = is_new;
                                w.player = Some(player_info);
                            });
                        }
                        Err(e) => {
                            leptos::logging::log!("Authentication error: {:?}", e);
                            wallet.update(|w| {
                                w.loading = false;
                                w.connected = true; // Still connected to wallet
                                w.error = Some(format!("Server error: {}", e));
                            });
                        }
                    }
                } else {
                    wallet.update(|w| {
                        w.loading = false;
                        w.error = Some("No accounts found in wallet".to_string());
                    });
                }
            }
            Err(e) => {
                wallet.update(|w| {
                    w.loading = false;
                    w.error = Some(e);
                });
            }
        }
    });

    let is_loading = move || wallet.get().loading;
    let is_connected = move || wallet.get().connected;
    let selected_address = move || {
        wallet
            .get()
            .selected_account
            .map(|a| truncate_address(&a.address))
    };
    let error_message = move || wallet.get().error.clone();
    let player_info = move || wallet.get().player.clone();
    let is_new_player = move || wallet.get().is_new_player;

    view! {
        <div class="wallet-container">
            {move || {
                if is_connected() {
                    view! {
                        <div class="wallet-connected">
                            <div class="wallet-info">
                                <span class="wallet-address">
                                    {selected_address()}
                                </span>
                                {move || player_info().map(|p| {
                                    view! {
                                        <span class="player-name">
                                            {p.username.unwrap_or_else(|| "Wandering Hero".to_string())}
                                        </span>
                                    }
                                })}
                            </div>
                            <AccountSelector />
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <button
                            class="connect-wallet-btn"
                            on:click=move |_| {connect_action.dispatch(());}
                            disabled=is_loading
                        >
                            {move || if is_loading() { "Connecting..." } else { "Connect Wallet" }}
                        </button>
                    }.into_any()
                }
            }}

            // Show welcome message for new players
            {move || {
                if is_new_player() {
                    Some(view! {
                        <div class="new-player-welcome">
                            <p>"🎉 Welcome, brave adventurer! Your quest begins now."</p>
                        </div>
                    })
                } else {
                    None
                }
            }}

            {move || error_message().map(|e| view! {
                <p class="wallet-error">{e}</p>
            })}
        </div>
    }
}

#[component]
fn AccountSelector() -> impl IntoView {
    let wallet = use_wallet();
    let accounts = move || wallet.get().accounts.clone();
    let selected = move || wallet.get().selected_account.clone();

    let on_select = move |account: WalletAccount| {
        wallet.update(|w| w.selected_account = Some(account));
    };

    view! {
        <select
            class="account-selector"
            on:change=move |ev| {
                let value = event_target_value(&ev);
                if let Some(account) = accounts().into_iter().find(|a| a.address == value) {
                    on_select(account);
                }
            }
        >
            <For
                each=accounts
                key=|account| account.address.clone()
                children=move |account| {
                    let addr = account.address.clone();
                    let name = account.name.clone().unwrap_or_else(|| truncate_address(&addr));
                    let is_selected = selected().map(|s| s.address == addr).unwrap_or(false);
                    view! {
                        <option value=addr selected=is_selected>
                            {name}
                        </option>
                    }
                }
            />
        </select>
    }
}

fn truncate_address(address: &str) -> String {
    if address.len() > 12 {
        format!("{}...{}", &address[..6], &address[address.len() - 4..])
    } else {
        address.to_string()
    }
}
