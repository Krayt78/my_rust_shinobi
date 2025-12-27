
#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{Extension, Router};
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use my_rust_shinobi::app::*;
    use my_rust_shinobi::db::{init_db_pool, get_database_url, run_migrations, DbPool};

    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Initialize database connection pool
    let database_url = get_database_url();
    log!("Connecting to database...");
    
    let db_pool: DbPool = init_db_pool(&database_url)
        .await
        .expect("Failed to connect to database. Make sure PostgreSQL is running and DATABASE_URL is correct.");
    
    log!("Database connected successfully!");

    // Run migrations if enabled
    if std::env::var("RUN_MIGRATIONS").unwrap_or_default() == "true" {
        log!("Running database migrations...");
        run_migrations(&db_pool)
            .await
            .expect("Failed to run database migrations");
        log!("Migrations completed!");
    }

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .layer(Extension(db_pool))
        .with_state(leptos_options);

    // run our app with hyper
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
