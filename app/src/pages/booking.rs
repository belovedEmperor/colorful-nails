use crate::{components::errors::ErrorView, pages::services::SERVICE_CATEGORIES};
use leptos::prelude::*;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Appointment {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub scheduled_at: chrono::DateTime<chrono::Utc>,
    pub services: Option<String>,
    pub notes: Option<String>,
    pub accepted: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[server]
pub async fn create_appointment(
    scheduled_at: String,
    services: Option<String>,
    notes: Option<String>,
    first_name: String,
    last_name: String,
    phone: String,
    email: String,
) -> Result<Appointment, ServerFnError> {
    use chrono::NaiveDateTime;
    use sqlx::{PgPool, query, query_as};

    let scheduled_at = NaiveDateTime::parse_from_str(&scheduled_at, "%Y-%m-%dT%H:%M")
        .map_err(|error| ServerFnError::new(format!("Invalid date: {error}")))?
        .and_utc();

    let db =
        use_context::<PgPool>().ok_or_else(|| ServerFnError::new("DB should be in context"))?;

    let user_uid = query!(
        "
            INSERT INTO users (first_name,last_name,phone,email)
            VALUES ($1,$2,$3,$4)
            RETURNING id
        ",
        first_name,
        last_name,
        phone,
        email
    )
    .fetch_one(&db)
    .await
    .map_err(ServerFnError::new)?;

    let user_id: uuid::Uuid = user_uid.id;

    let appointment = query_as!(
        Appointment,
        "
        INSERT INTO appointments (user_id,scheduled_at,services,notes)
        VALUES ($1,$2,$3,$4)
        RETURNING *
    ",
        user_id,
        scheduled_at,
        services,
        notes
    )
    .fetch_one(&db)
    .await
    .map_err(ServerFnError::new)?;

    Ok(appointment)
}

fn format_phone_number(input: &str) -> String {
    let digits: String = input.chars().filter(|char| char.is_ascii_digit()).collect();

    match digits.len() {
        0 => String::new(),
        1..=3 => format!("({digits}"),
        4..=6 => {
            let (area_code, rest) = digits.split_at(3);
            format!("({area_code}) {rest}")
        }
        7..=10 => {
            let (area_code, rest) = digits.split_at(3);
            let (middle, rest) = rest.split_at(3);
            format!("({area_code}) {middle}-{rest}")
        }
        _ => {
            let (area_code, rest) = digits.split_at(3);
            let (middle, rest) = rest.split_at(3);
            let (rest, _) = rest.split_at(4);
            format!("({area_code}) {middle}-{rest}")
        }
    }
}

#[component]
pub fn Booking() -> impl IntoView {
    let action = ServerAction::<CreateAppointment>::new();
    let (phone_number, set_phone_number) = signal(String::new());

    view! {
        <ErrorBoundary fallback=|errors| {
            view! { <ErrorView errors=errors /> }
        }>
            <div class="flex flex-col page-container section-padding section-container">
                <h1 class="page-header">"Booking"</h1>

                <ActionForm action=action>
                    <div class="grid grid-cols-2 gap-8 justify-between">
                        <div class="input-container">
                            <label for="first_name">"First Name:*"</label>
                            <input
                                class="input"
                                type="text"
                                placeholder="Jane"
                                id="first_name"
                                name="first_name"
                                required
                            />
                        </div>
                        <div class="input-container">
                            <label for="last_name">"Last Name:"</label>
                            <input
                                class="input"
                                type="text"
                                placeholder="Doe"
                                id="last_name"
                                name="last_name"
                            />
                        </div>

                        <div class="input-container">
                            <label for="phone">"Phone Number:*"</label>
                            <input
                                class="input"
                                type="tel"
                                placeholder="(570) 455-2799"
                                id="phone"
                                name="phone"
                                pattern="\\([0-9]{3}\\) [0-9]{3}-[0-9]{4}"
                                required
                                on:input:target=move |event| {
                                    set_phone_number
                                        .set(format_phone_number(&event.target().value()));
                                }
                                prop:value=phone_number
                            />
                        </div>

                        <div class="input-container">
                            <label for="email">"Email:*"</label>
                            <input
                                class="input"
                                type="email"
                                placeholder="example@example.com"
                                id="email"
                                name="email"
                                required
                            />
                        </div>

                        <div class="input-container">
                            <label for="services">"Services:"</label>
                            <select class="input" name="services" id="services">
                                <option value="">Choose a service</option>
                                {SERVICE_CATEGORIES
                                    .iter()
                                    .map(|service_category| {
                                        view! {
                                            <hr />
                                            <optgroup label=service_category
                                                .name>
                                                {service_category
                                                    .services
                                                    .iter()
                                                    .map(|service| {
                                                        view! {
                                                            <option value=service
                                                                .to_lowercase()
                                                                .retain(|character| {
                                                                    !character.is_whitespace()
                                                                })>{*service}</option>
                                                        }
                                                    })
                                                    .collect_view()}
                                            </optgroup>
                                        }
                                    })
                                    .collect_view()}
                            </select>
                        </div>

                        <div class="input-container">
                            <label for="scheduled_at">"Preferred Date:*"</label>
                            <input
                                class="input"
                                type="datetime-local"
                                id="scheduled_at"
                                name="scheduled_at"
                                required
                            />
                        </div>

                        <div class="input-container col-span-2">
                            <label for="notes">"Notes:"</label>
                            <textarea class="input h-16" id="notes" name="notes" rows="5" />
                        </div>

                        {move || {
                            if action.pending().get() {
                                view! { <p>"Submitting..."</p> }.into_any()
                            } else {
                                match action.value().get() {
                                    None => view! { <p></p> }.into_any(),
                                    Some(Ok(appointment)) => {
                                        view! { <p>"Booked! ID: " {appointment.id.to_string()}</p> }
                                            .into_any()
                                    }
                                    Some(Err(error)) => {
                                        view! { <p>"Error: " {error.to_string()}</p> }.into_any()
                                    }
                                }
                            }
                        }}

                        <input
                            class="button bg-primary col-start-1 col-span-2 w-fit mx-auto"
                            type="submit"
                            value="Submit"
                            disabled=move || action.pending().get()
                        />
                    </div>
                </ActionForm>
            </div>
        </ErrorBoundary>
    }
}
