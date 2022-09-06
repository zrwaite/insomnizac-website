package mail

import (
	"log"

	"github.com/sendgrid/sendgrid-go"
	"github.com/sendgrid/sendgrid-go/helpers/mail"
	"github.com/zrwaite/Insomnizac/settings"
)

func SendMessage(toEmail string, toName string, subject string, content string) (success bool) {
	from := mail.NewEmail("Zac Waite", settings.CONFIG.FromEmail)
	to := mail.NewEmail(toName, toEmail)
	plainTextContent := content
	htmlContent := content
	message := mail.NewSingleEmail(from, subject, to, plainTextContent, htmlContent)
	client := sendgrid.NewSendClient(settings.CONFIG.SendgridAPIKey)
	_, err := client.Send(message)
	if err != nil {
		log.Println(err)
		return false
	}
	return true
}

func ContactMessage(content string) (success bool) {
	subject := "New Insomnizac.xyz contact message"
	return SendMessage(settings.CONFIG.ContactEmail, "Zac Waite", subject, content)
}
