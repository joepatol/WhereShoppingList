package core

import (
	"log"
	"gorm.io/gorm"
)

type Depends struct {
	Database *gorm.DB
	Logger *log.Logger
}
