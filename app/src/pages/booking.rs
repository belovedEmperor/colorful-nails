use crate::{
    components::{alert::Alert, card::Card, errors::ErrorView, swatch_strip::SwatchStrip},
    pages::services::SERVICE_CATEGORIES,
};
use leptos::prelude::*;
#[component]
pub fn Booking() -> impl IntoView {
    let action = ServerAction::<CreateAppointment>::new();
    let (phone_number, set_phone_number) = signal(String::new());

    view! {
        <ErrorBoundary fallback=|errors| {
            view! { <ErrorView errors=errors /> }
        }>

            // ── Hero ──────────────────────────────────────────────────
            <div class="bg-midnight-ink text-white py-16">
                <div class="page-container flex flex-col gap-3">
                    <p class="text-xs font-sans font-semibold tracking-widest uppercase text-white/40">
                        "Colorful Nails & Spa · Hazleton, PA"
                    </p>
                    <h1 class="font-display text-title font-bold text-white">
                        "Book an Appointment"
                    </h1>
                    <p class="font-sans text-sm text-white/60 max-w-sm">
                        "Fill out the form and we’ll confirm your appointment as soon as possible."
                    </p>
                </div>
            </div>

            <SwatchStrip />

            // ── Form + sidebar ────────────────────────────────────────────
            <section class="section-padding">
                <div class="page-container grid md:grid-cols-[1.5fr_1fr] gap-6 items-start">

                    // ── Form card ──────────────────────────────────
                    <Card class="flex flex-col gap-6">
                        <h2 class="font-display text-xl font-bold text-midnight-ink">
                            "Your details"
                        </h2>

                        <ActionForm action=action>
                            <div class="grid grid-cols-2 gap-x-6 gap-y-5">

                                <div class="input-container">
                                    <label class="input-label" for="first_name">
                                        "First Name "
                                        <span class="text-coral-lacquer">"*"</span>
                                    </label>
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
                                    <label class="input-label" for="last_name">
                                        "Last Name"
                                    </label>
                                    <input
                                        class="input shadow"
                                        type="text"
                                        placeholder="Doe"
                                        id="last_name"
                                        name="last_name"
                                    />
                                </div>

                                <div class="input-container">
                                    <label class="input-label" for="phone">
                                        "Phone "
                                        <span class="text-coral-lacquer">"*"</span>
                                    </label>
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
                                    <label class="input-label" for="email">
                                        "Email "
                                        <span class="text-coral-lacquer">"*"</span>
                                    </label>
                                    <input
                                        class="input shadow"
                                        type="email"
                                        placeholder="jane@example.com"
                                        id="email"
                                        name="email"
                                        required
                                    />
                                </div>

                                <div class="input-container">
                                    <label class="input-label" for="services">
                                        "Service"
                                    </label>
                                    <select class="input shadow" name="services" id="services">
                                        <option value="">"Choose a service"</option>
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
                                                                view! { <option value=*service>{*service}</option> }
                                                            })
                                                            .collect_view()}
                                                    </optgroup>
                                                }
                                            })
                                            .collect_view()}
                                    </select>
                                </div>

                                <div class="input-container">
                                    <label class="input-label" for="scheduled_at">
                                        "Preferred Date "
                                        <span class="text-coral-lacquer">"*"</span>
                                    </label>
                                    <input
                                        class="input shadow"
                                        type="datetime-local"
                                        id="scheduled_at"
                                        name="scheduled_at"
                                        required
                                    />
                                </div>

                                <div class="input-container col-span-2">
                                    <label class="input-label" for="notes">
                                        "Notes"
                                    </label>
                                    <textarea
                                        class="input h-24 shadow"
                                        id="notes"
                                        name="notes"
                                        rows="4"
                                        placeholder="Anything we should know? Design ideas, allergies, etc."
                                    />
                                </div>

                                // Status message
                                <div class="col-span-2">
                                    {move || {
                                        if action.pending().get() {
                                            view! {
                                                <Alert variant="info">"Submitting your request…"</Alert>
                                            }
                                                .into_any()
                                        } else {
                                            match action.value().get() {
                                                None => view! { <div /> }.into_any(),
                                                Some(Ok(appointment)) => {
                                                    view! {
                                                        <Alert variant="success" title="Request received!">
                                                            "Booking ID: "
                                                            {appointment.id.to_string()}
                                                            ". We’ll email you to confirm shortly."
                                                        </Alert>
                                                    }
                                                        .into_any()
                                                }
                                                Some(Err(error)) => {
                                                    view! {
                                                        <Alert variant="error" title="Something went wrong">
                                                            {error.to_string()}
                                                            " Try again or call "
                                                            <a class="link" href="tel:+15704552799">
                                                                "(570) 455-2799"
                                                            </a>
                                                            "."
                                                        </Alert>
                                                    }
                                                        .into_any()
                                                }
                                            }
                                        }
                                    }}
                                </div>

                                // Submit
                                <button
                                    class="btn btn-primary btn-lg col-span-2 w-full"
                                    type="submit"
                                    disabled=move || action.pending().get()
                                >
                                    "Book Now"
                                </button>

                            </div>
                        </ActionForm>
                    </Card>

                    // ── Sidebar ─────────────────────────────────────
                    <div class="flex flex-col gap-4 md:sticky md:top-24">

                        <Card tint="blush" class="flex flex-col gap-4">
                            <h3 class="font-display text-base font-bold text-midnight-ink">
                                "Contact us directly"
                            </h3>
                            <div class="flex flex-col gap-2 font-sans text-sm text-midnight-ink/70">
                                <a
                                    href="tel:+15704552799"
                                    class="flex items-center gap-2 hover:text-coral-lacquer transition-colors"
                                >
                                    <span class="text-base">"☎️"</span>
                                    "(570) 455-2799"
                                </a>
                                <a
                                    rel="external"
                                    href="https://maps.app.goo.gl/ZxRttxppY3V1qUxm8"
                                    class="flex items-start gap-2 hover:text-coral-lacquer transition-colors"
                                >
                                    <span class="text-base mt-px">"📍"</span>
                                    "546 W Broad St, Hazleton, PA 18201"
                                </a>
                            </div>
                        </Card>

                        <Card tint="champagne" class="flex flex-col gap-4">
                            <h3 class="font-display text-base font-bold text-midnight-ink">
                                "Hours"
                            </h3>
                            <table class="hours-table">
                                <tbody>
                                    <tr>
                                        <td>"Mon – Sat"</td>
                                        <td>"9:30 AM – 7:30 PM"</td>
                                    </tr>
                                    <tr>
                                        <td>"Sunday"</td>
                                        <td>"11:00 AM – 6:00 PM"</td>
                                    </tr>
                                </tbody>
                            </table>
                            <p class="font-sans text-xs text-midnight-ink/50">
                                "Walk-ins welcome. "
                                "We’re often open on holidays — call to check."
                            </p>
                        </Card>

                        <Card tint="mint" class="flex flex-col gap-3">
                            <h3 class="font-display text-base font-bold text-midnight-ink">
                                "What happens next?"
                            </h3>
                            <ol class="flex flex-col gap-2 font-sans text-sm text-midnight-ink/70 list-none">
                                <li class="flex gap-2">
                                    <span class="font-bold text-midnight-ink shrink-0">
                                        "1. "
                                    </span>
                                    "We receive your request and check availability."
                                </li>
                                <li class="flex gap-2">
                                    <span class="font-bold text-midnight-ink shrink-0">
                                        "2. "
                                    </span>
                                    "You’ll get an email confirming or a call suggesting a new time."
                                </li>
                                <li class="flex gap-2">
                                    <span class="font-bold text-midnight-ink shrink-0">
                                        "3. "
                                    </span>
                                    "Show up and enjoy your appointment!"
                                </li>
                            </ol>
                        </Card>

                    </div>

                </div>
            </section>

        </ErrorBoundary>
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[cfg_attr(not(feature = "ssr"), expect(dead_code))]
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

    if first_name.trim().is_empty() {
        return Err(ServerFnError::new("First name is required."));
    }
    if email.trim().is_empty() || !email.contains('@') {
        return Err(ServerFnError::new("Valid email is required."));
    }

    let digits: String = phone.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.len() != 10 {
        return Err(ServerFnError::new("Phone must be a 10-digit US number."));
    }

    let scheduled_at = NaiveDateTime::parse_from_str(&scheduled_at, "%Y-%m-%dT%H:%M")
        .map_err(|error| ServerFnError::new(format!("Invalid date: {error}")))?
        .and_utc();
    if scheduled_at <= chrono::Utc::now() {
        return Err(ServerFnError::new("Appointment must be in the future."));
    }

    let db =
        use_context::<PgPool>().ok_or_else(|| ServerFnError::new("DB should be in context."))?;

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
