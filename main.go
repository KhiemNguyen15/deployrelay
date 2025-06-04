package main

import (
	"net/http"
	"os"

	_ "github.com/joho/godotenv/autoload"
	"github.com/khiemnguyen15/deployrelay/internal/webhook"
	"github.com/rs/zerolog"
	"github.com/rs/zerolog/log"
)

func init() {
	env := os.Getenv("APP_ENV")

	if env == "prod" {
		return
	}

	log.Logger = log.Output(zerolog.ConsoleWriter{Out: os.Stderr})
}

func main() {
	http.HandleFunc("/webhooks/keel", webhook.WebhookHandler)

	port := ":3000"
	log.Info().Msgf("Webhook server listening on port %s", port)
	if err := http.ListenAndServe(port, nil); err != nil {
		log.Fatal().Err(err).Msg("Server failed")
	}
}
