package services

func RegisterUser(email string, password string) error {
	hash := HashPassword(password)

	_ = hash
	return nil
}
