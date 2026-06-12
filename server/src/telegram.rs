use app::Telegram;
use axum::{Json, extract::State};
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::json;
use sqlx::{PgPool, types::uuid};

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

    sqlx::query!(
        "
            UPDATE appointments
            SET accepted = $1
            WHERE id = $2
        ",
        accepted,
        appointment_id
    )
    .execute(&db)
    .await
    .ok();

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
            "callback_query_id":id
        }))
        .send()
        .await
        .ok();

    StatusCode::OK
}
