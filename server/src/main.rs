#![recursion_limit = "512"]
use app::{App, Telegram, shell};
use axum::{Router, extract::FromRef};
use leptos::{
    config::{LeptosOptions, get_configuration},
    logging::log,
    prelude::provide_context,
};
use leptos_axum::{LeptosRoutes as _, generate_route_list};
use sqlx::PgPool;
use tower_http::{compression::CompressionLayer, services::ServeDir};

use crate::booking::telegram_webhook;

mod booking;

#[derive(Clone)]
struct AppState {
    leptos_options: LeptosOptions,
    db: PgPool,
    client: reqwest::Client,
    telegram: Telegram,
    resend_api_key: String,
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
impl FromRef<AppState> for String {
    fn from_ref(state: &AppState) -> Self {
        state.resend_api_key.clone()
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&db_url)
        .await
        .expect("Failed to connect to database");

    let mut configuration = get_configuration(None).expect("Leptos configuration should exist");
    if let Ok(port) = std::env::var("PORT") {
        let port: u16 = port.parse().expect("PORT must be a number");
        configuration.leptos_options.site_addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    }
    let address = configuration.leptos_options.site_addr;
    let leptos_options = configuration.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let telegram = Telegram {
        token: std::env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN must be set"),
        chat_id: std::env::var("TELEGRAM_CHAT_ID").expect("TELEGRAM_CHAT_ID must be set"),
    };

    let resend_api_key = std::env::var("RESEND_API_KEY").expect("RESEND_API_KEY");

    let state = AppState {
        leptos_options,
        db: pool,
        client: reqwest::Client::new(),
        telegram,
        resend_api_key,
    };

    let app = Router::new()
        .nest_service("/pkg", ServeDir::new("pkg"))
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
        .with_state(state)
        .layer(CompressionLayer::new());

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
