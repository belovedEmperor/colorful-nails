use app::{App, Telegram, shell};
use axum::{Router, extract::FromRef};
use leptos::{
    config::{LeptosOptions, get_configuration},
    logging::log,
    prelude::provide_context,
};
use leptos_axum::{LeptosRoutes as _, generate_route_list};
use sqlx::PgPool;

use crate::telegram::telegram_webhook;

mod telegram;

#[derive(Clone)]
struct AppState {
    leptos_options: LeptosOptions,
    db: PgPool,
    client: reqwest::Client,
    telegram: Telegram,
}

impl FromRef<AppState> for LeptosOptions {
    fn from_ref(state: &AppState) -> Self {
        state.leptos_options.clone()
    }
}
impl FromRef<AppState> for PgPool {
    fn from_ref(state: &AppState) -> Self {
        state.db.clone()
    }
}
impl FromRef<AppState> for reqwest::Client {
    fn from_ref(state: &AppState) -> Self {
        state.client.clone()
    }
}
impl FromRef<AppState> for Telegram {
    fn from_ref(state: &AppState) -> Self {
        state.telegram.clone()
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&db_url)
        .await
        .expect("Failed to connect to database");

    let configuration = get_configuration(None).expect("Leptos configuration should exist");
    let address = configuration.leptos_options.site_addr;
    let leptos_options = configuration.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let token = std::env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN must be set");
    let chat_id = std::env::var("TELEGRAM_CHAT_ID").expect("TELEGRAM_CHAT_ID must be set");
    let telegram = Telegram { token, chat_id };

    let state = AppState {
        leptos_options,
        db: pool,
        client: reqwest::Client::new(),
        telegram,
    };

    let app = Router::new()
        .route("/telegram/webhook", axum::routing::post(telegram_webhook))
        .leptos_routes_with_context(
            &state,
            routes,
            {
                let pool = state.db.clone();
                let client = state.client.clone();
                let telegram = state.telegram.clone();
                move || {
                    provide_context(pool.clone());
                    provide_context(client.clone());
                    provide_context(telegram.clone());
                }
            },
            {
                let leptos_options = state.leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
        .with_state(state);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &address);
    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .expect("Listener should bind");
    axum::serve(listener, app.into_make_service())
        .await
        .expect("Server should serve");
}
