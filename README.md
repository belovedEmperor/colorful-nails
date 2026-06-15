# Colorful Nails & Spa
Website mainly for customers to make online appointments with.

## Environment Variables
An example of the required environment variables are available in `.env.example`.

## Telegram Setup
1. Create a bot.
2. Copy the token associated with the bot.
3. Start a chat with the bot and send a message using the account you intend to link.
4. Go to `https://api.telegram.org/bot<BOT_TOKEN>/getUpdates` and copy the `chat_id`.
5. Add token and ID to `.env`.

## Gmail Setup
1. Create an app password for the email you intend to use.
2. Add that email and app password to `.env`.

## Running Locally
1. Run `docker compose up -d` to run the database.
2. Run `sqlx migrate run` to create tables.
3. Run `cargo leptos watch` for development or `cargo leptos build --release` to build binary.
