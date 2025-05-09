use std::env;

use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::post};
use dotenvy::dotenv;
use regex::Regex;
use serde::Deserialize;
use serenity::all::{CreateEmbed, ExecuteWebhook, Http, Timestamp, Webhook};
use tracing_subscriber::{EnvFilter, fmt};

#[derive(Deserialize)]
struct KeelWebhookPayload {
    message: String,
    #[serde(rename = "createdAt")]
    created_at: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    init_tracing();

    let app = Router::new().route("/webhooks/keel", post(handle_webhook));

    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    tracing::info!("Webhook server listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}

async fn handle_webhook(Json(payload): Json<KeelWebhookPayload>) -> impl IntoResponse {
    let embed = create_message(&payload);

    let webhook_url = match env::var("DISCORD_WEBHOOK_URL") {
        Ok(url) => url,
        Err(err) => {
            tracing::error!("Discord webhook URL is not set: {}", err);
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };

    let http = Http::new("");
    let webhook = match Webhook::from_url(&http, &webhook_url).await {
        Ok(w) => w,
        Err(err) => {
            tracing::error!("Failed to parse Discord webhook: {}", err);
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };

    let builder = ExecuteWebhook::new()
        .embed(embed)
        .username("Keel Deployments");

    if let Err(err) = webhook.execute(&http, false, builder).await {
        tracing::error!("Failed to send webhook: {}", err);
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    tracing::info!("Discord webhook message sent successfully");
    StatusCode::OK
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

fn init_tracing() {
    let app_env = env::var("APP_ENV").unwrap_or("dev".into());

    let default_filter = match app_env.as_str() {
        "prod" => "info",
        _ => "debug",
    };

    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default_filter));

    fmt().with_env_filter(filter).init();
}
