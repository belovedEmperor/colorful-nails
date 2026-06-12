use app::{Appointment, Telegram};
use axum::{Json, extract::State};
use lettre::{AsyncSmtpTransport, AsyncTransport as _, Tokio1Executor};
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::json;
use sqlx::{PgPool, query, types::uuid};

#[derive(Deserialize)]
pub struct TelegramUpdate {
    pub callback_query: Option<CallbackQuery>,
}

#[derive(Deserialize)]
pub struct CallbackQuery {
    pub id: String,
    pub data: Option<String>,
    pub message: Option<CallbackMessage>,
}

#[derive(Deserialize)]
pub struct CallbackMessage {
    #[serde(rename = "message_id")]
    pub id: i32,
    pub chat: CallbackChat,
}

#[derive(Deserialize)]
pub struct CallbackChat {
    pub id: i64,
}

pub async fn telegram_webhook(
    State(db): State<PgPool>,
    State(client): State<reqwest::Client>,
    State(telegram): State<Telegram>,
    State(gmail): State<Gmail>,
    State(mailer): State<AsyncSmtpTransport<Tokio1Executor>>,
    Json(update): Json<TelegramUpdate>,
) -> StatusCode {
    let Some(callback_query) = update.callback_query else {
        return StatusCode::OK;
    };
    let Some(data) = callback_query.data else {
        return StatusCode::OK;
    };

    let Some((action, id)) = data.split_once(':') else {
        return StatusCode::OK;
    };

    let Ok(appointment_id) = uuid::Uuid::parse_str(id) else {
        return StatusCode::OK;
    };

    let accepted = match action {
        "accept" => true,
        "deny" => false,
        _ => return StatusCode::OK,
    };

    let Ok(appointment) = sqlx::query_as!(
        Appointment,
        "
UPDATE appointments
SET accepted = $1
WHERE id = $2
RETURNING *
        ",
        accepted,
        appointment_id
    )
    .fetch_one(&db)
    .await
    else {
        return StatusCode::OK;
    };

    // Removes buttons from text
    client
        .post(format!(
            "https://api.telegram.org/bot{}/editMessageReplyMarkup",
            telegram.token
        ))
        .json(&json!({
            "chat_id": callback_query.message.as_ref().map(|message| message.chat.id),
            "message_id": callback_query.message.as_ref().map(|message| message.id),
            "reply_markup": { "inline_keyboard": [] }
        }))
        .send()
        .await
        .ok();

    client
        .post(format!(
            "https://api.telegram.org/bot{}/sendMessage",
            telegram.token
        ))
        .json(&json!({
            "chat_id": callback_query.message.as_ref().map(|message| message.chat.id),
            "text": if accepted { "✅ Appointment confirmed." } else { "❌ Appointment denied." }
        }))
        .send()
        .await
        .ok();

    client
        .post(format!(
            "https://api.telegram.org/bot{}/answerCallbackQuery",
            telegram.token,
        ))
        .json(&json!({
            "callback_query_id": callback_query.id
        }))
        .send()
        .await
        .ok();

    email_customer(gmail, db, appointment, mailer).await.ok();

    StatusCode::OK
}

#[derive(Clone)]
pub struct Gmail {
    pub from: String,
    pub app_password: String,
}

pub async fn email_customer(
    gmail: Gmail,
    db: PgPool,
    appointment: Appointment,
    mailer: AsyncSmtpTransport<Tokio1Executor>,
) -> Result<(), Box<dyn std::error::Error>> {
    let user_email = query!(
        "
SELECT email FROM users
WHERE id = $1
        ",
        appointment.user_id
    )
    .fetch_one(&db)
    .await?
    .email;

    let email = lettre::Message::builder()
        .from(gmail.from.parse()?)
        .to(user_email.parse()?)
        .subject(if appointment.accepted {
            "Appointment Accepted"
        } else {
            "Appointment Denied"
        })
        .body(format!(
            "
Your appointment at {} has been {}
Service: {}
Notes: {}
            ",
            appointment.scheduled_at,
            if appointment.accepted {
                "accepted!"
            } else {
                "denied."
            },
            appointment.services.unwrap_or_default(),
            appointment.notes.unwrap_or_default()
        ))?;

    mailer.send(email).await?;

    Ok(())
}
