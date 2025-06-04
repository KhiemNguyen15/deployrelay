package discord

import (
	"bytes"
	"encoding/json"
	"fmt"
	"net/http"
	"regexp"
)

type KeelWebhookPayload struct {
	Message   string `json:"message"`
	CreatedAt string `json:"createdAt"`
}

type EmbedField struct {
	Name   string `json:"name"`
	Value  string `json:"value"`
	Inline bool   `json:"inline,omitempty"`
}

type Embed struct {
	Title       string       `json:"title,omitempty"`
	Description string       `json:"description,omitempty"`
	URL         string       `json:"url,omitempty"`
	Color       int          `json:"color,omitempty"` // Decimal integer, not hex
	Timestamp   string       `json:"timestamp,omitempty"`
	Fields      []EmbedField `json:"fields,omitempty"`
}

type WebhookMessage struct {
	Username  string  `json:"username,omitempty"`
	AvatarURL string  `json:"avatar_url,omitempty"`
	Content   string  `json:"content,omitempty"`
	Embeds    []Embed `json:"embeds,omitempty"`
}

func SendMessage(webhookURL string, payload KeelWebhookPayload) error {
	embed := createEmbed(payload)

	message := WebhookMessage{
		Username: "Keel Deployments",
		Embeds:   []Embed{embed},
	}

	msgPayload, err := json.Marshal(message)
	if err != nil {
		return err
	}

	resp, err := http.Post(webhookURL, "application/json", bytes.NewBuffer(msgPayload))
	if err != nil {
		return err
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusNoContent {
		return fmt.Errorf("unexpected response: %s", resp.Status)
	}

	return nil
}

func createEmbed(payload KeelWebhookPayload) Embed {
	re := regexp.MustCompile(`^(.*?)\s*\(([^)]+)\)`)
	matches := re.FindStringSubmatch(payload.Message)
	
	if len(matches) >= 3 {
		description := matches[1]
		if description != "" {
			description = regexp.MustCompile(`\s+$`).ReplaceAllString(description, "")
		}
		image := matches[2]
		if image != "" {
			image = regexp.MustCompile(`^\s+|\s+$`).ReplaceAllString(image, "")
		}
		
		return Embed{
			Title:       "Deployment Update",
			Description: description,
			Color:       0x326CE5,
			Timestamp:   payload.CreatedAt,
			Fields: []EmbedField{
				{
					Name:   "Image",
					Value:  image,
					Inline: false,
				},
			},
		}
	}
	
	return Embed{
		Title:       "Deployment Update",
		Description: payload.Message,
		Color:       0x326CE5,
		Timestamp:   payload.CreatedAt,
	}
}
