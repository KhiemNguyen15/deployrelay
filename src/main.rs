use std::env;

use axum::{Json, Router, routing::post};
use dotenvy::dotenv;
use regex::Regex;
use serde::Deserialize;
use serenity::all::{CreateEmbed, ExecuteWebhook, Http, Timestamp, Webhook};

#[derive(Deserialize)]
struct KeelWebhookPayload {
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
    let embed = create_message(&payload);

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

fn create_message(payload: &KeelWebhookPayload) -> CreateEmbed {
    let timestamp: Timestamp = payload.created_at.parse().unwrap_or(Timestamp::now());
    let color = 0x326CE5;

    let re = Regex::new(r"^(.*?)\s*\(([^)]+)\)").unwrap();
    if let Some(caps) = re.captures(&payload.message) {
        let description = caps
            .get(1)
            .map(|m| m.as_str().trim().to_string())
            .unwrap_or_default();
        let image = caps
            .get(2)
            .map(|m| m.as_str().trim().to_string())
            .unwrap_or_default();

        let embed = CreateEmbed::new()
            .title("Deployment Update")
            .description(&description)
            .field("Image", &image, false)
            .color(color)
            .timestamp(&timestamp);

        return embed;
    }

    CreateEmbed::new()
        .title("Deployment Update")
        .description(&payload.message)
        .color(color)
        .timestamp(&timestamp)
}
