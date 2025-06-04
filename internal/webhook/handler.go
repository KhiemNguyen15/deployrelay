package webhook

import (
	"encoding/json"
	"io"
	"net/http"
	"os"

	"github.com/khiemnguyen15/deployrelay/internal/discord"
	"github.com/rs/zerolog/log"
)

func WebhookHandler(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	body, err := io.ReadAll(r.Body)
	if err != nil {
		http.Error(w, "Failed to read body", http.StatusBadRequest)
		return
	}
	defer r.Body.Close()

	log.Info().Str("body", string(body)).Msg("Received webhook")

	var payload discord.KeelWebhookPayload
	if err := json.Unmarshal(body, &payload); err != nil {
		log.Error().Err(err).Msg("Failed to parse incoming payload")
		http.Error(w, "Failed to parse payload", http.StatusBadRequest)
		return
	}

	webhookURL := os.Getenv("DISCORD_WEBHOOK_URL")
	if webhookURL == "" {
		http.Error(w, "Failed to retrieve Discord webhook URL", http.StatusInternalServerError)
		log.Fatal().Msg("Discord webhook URL is not set")
	}

	if err := discord.SendMessage(webhookURL, payload); err != nil {
		log.Error().Err(err).Msg("Failed to send Discord webhook message")
		http.Error(w, "Failed to send Discord webhook message", http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
	w.Write([]byte("Webhook received"))
}
