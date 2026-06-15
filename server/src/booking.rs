use app::{Appointment, Telegram};
use axum::{Json, extract::State};
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
    State(resend_api_key): State<String>,
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

    // Idempotency: only update if still pending (accepted IS NULL)
    let Ok(appointment) = sqlx::query_as!(
        Appointment,
        "
UPDATE appointments
SET accepted = $1
WHERE id = $2 AND accepted IS NULL
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

    // Remove buttons from message
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

    email_customer(client, resend_api_key, db, appointment)
        .await
        .ok();

    StatusCode::OK
}

/// Send accept/deny email to customer via Resend
///
/// # Errors
/// Returns `Err` if db fails to fetch user email or email fails to send
pub async fn email_customer(
    client: reqwest::Client,
    resend_api_key: String,
    db: PgPool,
    appointment: Appointment,
) -> Result<(), Box<dyn std::error::Error>> {
    let user_email: String = query!(
        "
SELECT email FROM users
WHERE id = $1
        ",
        appointment.user_id
    )
    .fetch_one(&db)
    .await?
    .email;

    let accepted = appointment.accepted == Some(true);
    let subject = if accepted {
        "Your Appointment is Confirmed ✅ - Colorful Nails & Spa"
    } else {
        "Appointment Update ❌ - Colorful Nails & Spa"
    };
    let status_color = if accepted { "#16a34a" } else { "#dc2626" };
    let status_label = if accepted {
        "Confirmed ✅"
    } else {
        "Denied ❌"
    };
    let status_verb = if accepted { "accepted" } else { "denied" };
    let scheduled_at = appointment.scheduled_at.to_string();
    let services = appointment.services.clone().unwrap_or_default();
    let notes = appointment.notes.clone().unwrap_or_default();

    let html_body = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<body style="margin:0;padding:0;background:#fdf2f8;font-family:Arial,sans-serif;">
  <table width="100%" cellpadding="0" cellspacing="0" style="background:#fdf2f8;padding:40px 0;">
    <tr><td align="center">
      <table width="520" cellpadding="0" cellspacing="0" style="background:#ffffff;border-radius:16px;overflow:hidden;box-shadow:0 4px 16px rgba(0,0,0,0.08);">

        <!-- Header -->
        <tr>
          <td style="background:#db2777;padding:36px 32px;text-align:center;">
            <p style="margin:0 0 4px;color:#fce7f3;font-size:13px;letter-spacing:2px;text-transform:uppercase;">Colorful Nails &amp; Spa</p>
            <h1 style="margin:0;color:#ffffff;font-size:26px;font-weight:700;">Appointment Update</h1>
          </td>
        </tr>

        <!-- Status badge -->
        <tr>
          <td style="padding:36px 32px 24px;text-align:center;">
            <span style="display:inline-block;background:{status_color};color:#ffffff;padding:10px 32px;border-radius:999px;font-size:17px;font-weight:700;letter-spacing:0.5px;">{status_label}</span>
            <p style="margin:18px 0 0;color:#374151;font-size:15px;">Your appointment has been <strong>{status_verb}</strong>.</p>
          </td>
        </tr>

        <!-- Details card -->
        <tr>
          <td style="padding:0 32px 36px;">
            <table width="100%" cellpadding="0" cellspacing="0" style="background:#fdf2f8;border-radius:10px;">
              <tr><td style="padding:24px;">
                <table width="100%" cellpadding="0" cellspacing="0">
                  <tr>
                    <td style="padding:8px 0;border-bottom:1px solid #fce7f3;">
                      <p style="margin:0;font-size:11px;color:#9d174d;font-weight:700;text-transform:uppercase;letter-spacing:1px;">Date &amp; Time</p>
                      <p style="margin:4px 0 0;font-size:14px;color:#1f2937;">{scheduled_at}</p>
                    </td>
                  </tr>
                  <tr>
                    <td style="padding:8px 0;border-bottom:1px solid #fce7f3;">
                      <p style="margin:0;font-size:11px;color:#9d174d;font-weight:700;text-transform:uppercase;letter-spacing:1px;">Services</p>
                      <p style="margin:4px 0 0;font-size:14px;color:#1f2937;">{services}</p>
                    </td>
                  </tr>
                  <tr>
                    <td style="padding:8px 0;">
                      <p style="margin:0;font-size:11px;color:#9d174d;font-weight:700;text-transform:uppercase;letter-spacing:1px;">Notes</p>
                      <p style="margin:4px 0 0;font-size:14px;color:#1f2937;">{notes}</p>
                    </td>
                  </tr>
                </table>
              </td></tr>
            </table>
          </td>
        </tr>

        <!-- Footer -->
        <tr>
          <td style="background:#fdf2f8;padding:20px 32px;text-align:center;border-top:1px solid #fce7f3;">
            <p style="margin:0;color:#9d174d;font-size:13px;">Questions? Give us a call - (570) 455-2799.</p>
            <p style="margin:6px 0 0;color:#be185d;font-size:12px;">Colorful Nails &amp; Spa &middot; Hazleton, PA</p>
          </td>
        </tr>

      </table>
    </td></tr>
  </table>
</body>
</html>"#,
    );

    client
        .post("https://api.resend.com/emails")
        .bearer_auth(&resend_api_key)
        .json(&json!({
            "from": "Colorful Nails & Spa <onboarding@resend.dev>",
            "to": [user_email],
            "subject": subject,
            "html": html_body
        }))
        .send()
        .await?;

    Ok(())
}
