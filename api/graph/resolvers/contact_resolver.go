package resolvers

import (
	"errors"

	"github.com/zrwaite/Insomnizac/mail"
)

func ContactResolver(message string) (bool, error) {
	if mail.ContactMessage(message) {
		return true, nil
	} else {
		return false, errors.New("contact message failed to send")
	}
}
