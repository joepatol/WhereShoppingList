package auth

import (
	"models"
	"errors"
	"gorm.io/gorm"
	"golang.org/x/crypto/bcrypt"
)

func GetUserById(id uint, db *gorm.DB) (*models.User, error) {
	var user models.User

	if err := db.First(&user, id).Error; err != nil {
		return nil, errors.New("user not found")
	}

	return &user, nil
} 

func saveUser(user models.User, db *gorm.DB) (*models.User, error) {
	//turn password into hash
	hashedPassword, err := bcrypt.GenerateFromPassword([]byte(user.Password), bcrypt.DefaultCost)
	if err != nil {
		return nil, err
	}
	user.Password = string(hashedPassword)

	err = db.Create(&user).Error
	if err != nil {
		return nil, err
	}
	return &user, nil
}


func VerifyPassword(password, hashedPassword string) error {
	return bcrypt.CompareHashAndPassword([]byte(hashedPassword), []byte(password))
}

func loginCheck(email string, password string, db *gorm.DB) (string, error) {
	var err error

	user := models.User{}

	err = db.Model(models.User{}).Where("email = ?", email).Take(&user).Error

	if err != nil {
		return "", err
	}

	err = VerifyPassword(password, user.Password)

	if err != nil && err == bcrypt.ErrMismatchedHashAndPassword {
		return "", err
	}

	token, err := GenerateToken(user.ID)

	if err != nil {
		return "", err
	}

	return token,nil
}