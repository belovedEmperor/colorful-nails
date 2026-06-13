use crate::{components::errors::ErrorView, pages::services::SERVICE_CATEGORIES};
use leptos::prelude::*;

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
                                class="input shadow"
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
                                class="input shadow"
                                type="text"
                                placeholder="Doe"
                                id="last_name"
                                name="last_name"
                            />
                        </div>

                        <div class="input-container">
                            <label for="phone">"Phone Number:*"</label>
                            <input
                                class="input shadow"
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
                                class="input shadow"
                                type="email"
                                placeholder="example@example.com"
                                id="email"
                                name="email"
                                required
                            />
                        </div>

                        <div class="input-container">
                            <label for="services">"Services:"</label>
                            <select class="input shadow" name="services" id="services">
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
                                                        let value = service.to_lowercase().replace(' ', "");
                                                        view! { <option value=value>{*service}</option> }
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
                                class="input shadow"
                                type="datetime-local"
                                id="scheduled_at"
                                name="scheduled_at"
                                required
                            />
                        </div>

                        <div class="input-container col-span-2">
                            <label for="notes">"Notes:"</label>
                            <textarea class="input h-16 shadow" id="notes" name="notes" rows="5" />
                        </div>

                        {move || {
                            if action.pending().get() {
                                view! { <p>"Submitting..."</p> }.into_any()
                            } else {
                                match action.value().get() {
                                    None => view! { <p></p> }.into_any(),
                                    Some(Ok(appointment)) => {
                                        view! {
                                            <p>
                                                "Booked! ID: " {appointment.id.to_string()} <br />
                                                "We will email you about the status of the appointment soon."
                                            </p>
                                        }
                                            .into_any()
                                    }
                                    Some(Err(error)) => {

                                        view! { <p>"Error: " {error.to_string()}</p> }
                                            .into_any()
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

#[derive(serde::Serialize, serde::Deserialize, Clone)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[cfg_attr(not(feature = "ssr"), allow(dead_code))]
pub struct User {
    pub id: uuid::Uuid,
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

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
    use sqlx::{PgPool, query_as};

    let scheduled_at = NaiveDateTime::parse_from_str(&scheduled_at, "%Y-%m-%dT%H:%M")
        .map_err(|error| ServerFnError::new(format!("Invalid date: {error}")))?
        .and_utc();

    let db =
        use_context::<PgPool>().ok_or_else(|| ServerFnError::new("DB should be in context"))?;

    let user = query_as!(
        User,
        "
INSERT INTO users (first_name,last_name,phone,email)
VALUES ($1,$2,$3,$4)
RETURNING *
        ",
        first_name,
        last_name,
        phone,
        email
    )
    .fetch_one(&db)
    .await
    .map_err(ServerFnError::new)?;

    let user_id: uuid::Uuid = user.id;

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

    let client = use_context::<reqwest::Client>()
        .ok_or_else(|| ServerFnError::new("reqwest client should be in context"))?;
    let telegram = use_context::<Telegram>()
        .ok_or_else(|| ServerFnError::new("Telegram should be in context"))?;

    let appointment_details = appointment.clone();

    let text = format!(
        "
<b>New Appointment</b>

<b>Name:</b> {} {}
<b>Phone:</b> {}
<b>Email:</b> {}
<b>Service:</b> {}
<b>Date & Time:</b> {}
<b>Notes:</b> {}
        ",
        user.first_name,
        user.last_name,
        user.phone,
        user.email,
        appointment_details
            .services
            .unwrap_or_else(|| "N/A".to_owned()),
        appointment_details.scheduled_at,
        appointment_details.notes.unwrap_or_else(String::new)
    );

    message_telegram(
        client,
        telegram.token,
        telegram.chat_id,
        text,
        appointment.id,
    )
    .await?;

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

#[cfg(feature = "ssr")]
#[derive(Clone)]
pub struct Telegram {
    pub token: String,
    pub chat_id: String,
}

#[cfg(feature = "ssr")]
pub async fn message_telegram(
    client: reqwest::Client,
    token: String,
    chat_id: String,
    text: String,
    appointment_id: uuid::Uuid,
) -> Result<(), ServerFnError> {
    client
        .post(format!("https://api.telegram.org/bot{token}/sendMessage"))
        .json(&serde_json::json!({
            "chat_id":chat_id,
            "text":text,
            "parse_mode":"HTML",
            "reply_markup": {
                "inline_keyboard": [[
                    {"text": "✅ Accept", "callback_data": format!("accept:{appointment_id}")},
                    {"text": "❌ Deny", "callback_data": format!("deny:{appointment_id}")}
                ]]
            }
        }))
        .send()
        .await
        .map_err(ServerFnError::new)?;

    Ok(())
}
