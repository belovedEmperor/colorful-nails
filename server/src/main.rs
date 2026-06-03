use app::{App, shell};
use axum::{
    Json,
    Router,
    extract::State,
    http::StatusCode,
    // routing::{get, post},
};
use leptos::{config::get_configuration, logging::log};
use leptos_axum::{LeptosRoutes, generate_route_list};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, query, query_as};

#[derive(Clone)]
struct AppState {
    db: PgPool,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&db_url)
        .await
        .expect("Failed to connect to database");

    let state = AppState { db: pool };

    let configuration = get_configuration(None).expect("Leptos configuration should exist");
    let address = configuration.leptos_options.site_addr;
    let leptos_options = configuration.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        // .route("/appointments", get(get_appointments))
        // .route("/appointments", post(create_appointment))
        .with_state(leptos_options);

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

#[derive(Serialize, sqlx::FromRow)]
struct Appointment {
    id: uuid::Uuid,
    user_id: uuid::Uuid,
    scheduled_at: chrono::DateTime<chrono::Utc>,
    services: Option<Vec<String>>,
    accepted: bool,
    created_at: chrono::DateTime<chrono::Utc>,
}

async fn get_appointments(
    State(state): State<AppState>,
) -> Result<Json<Vec<Appointment>>, (StatusCode, String)> {
    let appointments = query_as!(
        Appointment,
        "
        SELECT * FROM appointments;
        "
    )
    .fetch_all(&state.db)
    .await
    .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()))?;

    Ok(Json(appointments))
}

#[derive(Deserialize)]
struct CreateAppointment {
    scheduled_at: chrono::DateTime<chrono::Utc>,
    services: Option<Vec<String>>,
    first_name: String,
    last_name: String,
    phone: String,
    email: String,
}

async fn create_appointment(
    State(state): State<AppState>,
    Json(body): Json<CreateAppointment>,
) -> Result<Json<Appointment>, (StatusCode, String)> {
    let user = query!(
        "
            INSERT INTO users (first_name,last_name,phone,email)
            VALUES ($1,$2,$3,$4)
            RETURNING id
        ",
        body.first_name,
        body.last_name,
        body.phone,
        body.email
    )
    .fetch_one(&state.db)
    .await
    .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()))?;

    let user_id: uuid::Uuid = user.id;

    let appointment = query_as!(
        Appointment,
        "
        INSERT INTO appointments (user_id,scheduled_at,services)
        VALUES ($1,$2,$3)
        RETURNING *
    ",
        user_id,
        body.scheduled_at,
        body.services.as_deref()
    )
    .fetch_one(&state.db)
    .await
    .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()))?;

    Ok(Json(appointment))
}
