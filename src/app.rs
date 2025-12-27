//! Main application entry point and routing

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::wallet::WalletProvider;
use crate::components::TopNavBar;
use crate::pages::{HomePage, CharacterPage};

/// Server-side rendered HTML shell
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
                // Google Fonts for fantasy-style typography
                <link rel="preconnect" href="https://fonts.googleapis.com"/>
                <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous"/>
                <link href="https://fonts.googleapis.com/css2?family=Cinzel:wght@400;500;600;700&family=Almendra:wght@400;700&family=MedievalSharp&display=swap" rel="stylesheet"/>
                // Bundled Polkadot wallet JavaScript
                <script src="/wallet.js"></script>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

/// Main application component with routing
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/my_rust_shinobi.css"/>
        <Title text="Realm of Legends"/>
        
        <WalletProvider>
            <Router>
                <div class="game-container">
                    // Top Navigation Bar
                    <TopNavBar />
                    
                    // Main Content Area
                    <main class="main-content">
                        <Routes fallback=|| "Page not found.".into_view()>
                            <Route path=StaticSegment("") view=HomePage/>
                            <Route path=StaticSegment("character") view=CharacterPage/>
                        </Routes>
                    </main>
                </div>
            </Router>
        </WalletProvider>
    }
}
