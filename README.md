# Colorful Nails & Spa
Website mainly for customers to make online appointments with.

## Environment Variables
An example of the required environment variables are available in `.env.example`.

## Telegram Setup
1. Create a bot.
2. Copy the token associated with the bot.
3. Start a chat with the bot and send a message using the account you intend to link.
4. Go to/fetch `https://api.telegram.org/bot<BOT_TOKEN>/getUpdates` and copy the `chat_id`.
5. Add token and ID to `.env`.

### Set Webhook
Go to/fetch `https://api.telegram.org/bot<BOT_TOKEN>/setWebhook?url=<DEPLOYED_URL>//colorful-nails-production.up.railway.app/telegram/webhook`.
For local testing, `DEPLOYED_URL` can be a tunnel created with a tool such as `cloudflared`.

## Email Setup
1. Get Resend API key.
2. Add key to `.env`.

## Running Locally
1. Run `docker compose up -d` to run the database.
2. Run `sqlx migrate run` to create tables.
3. Run `cargo leptos watch` for development or `cargo leptos build --release` to build binary.
