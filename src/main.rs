use std::env;

use axum::{Json, Router, routing::post};
use dotenvy::dotenv;
use serde::Deserialize;
use serenity::all::{CreateEmbed, ExecuteWebhook, Http, Timestamp, Webhook};

#[derive(Deserialize, Debug)]
struct KeelWebhookPayload {
    name: String,
    message: String,
    #[serde(rename = "createdAt")]
    created_at: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/webhooks/keel", post(handle_webhook));

    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Webhook server listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}

async fn handle_webhook(Json(payload): Json<KeelWebhookPayload>) {
    let timestamp: Timestamp = payload
        .created_at
        .parse()
        .expect("Failed to parse timestamp");

    let embed = CreateEmbed::new()
        .title(payload.name)
        .field("message", payload.message, false)
        .timestamp(timestamp);

    let webhook_url = match env::var("DISCORD_WEBHOOK_URL") {
        Ok(url) => url,
        Err(_) => {
            eprintln!("Discord webhook URL is not set");
            return;
        }
    };

    let http = Http::new("");
    let webhook = Webhook::from_url(&http, &webhook_url)
        .await
        .expect("Invalid Discord webhook URL");

    let builder = ExecuteWebhook::new()
        .embed(embed)
        .username("Keel Deployments");

    webhook
        .execute(&http, false, builder)
        .await
        .expect("Failed to execute webhook");
}
