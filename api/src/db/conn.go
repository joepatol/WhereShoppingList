package db

import (
	"context"
	"fmt"
	"os"

	"github.com/jackc/pgx/v5/pgxpool"
)

const CONN_URL = "postgres://postgresuser:postgrespwd@localhost:5432/supermarkt";

func ConnectDb() *pgxpool.Pool {
	conn, err := pgxpool.New(context.Background(), CONN_URL)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Unable to connect to database: %v\n", err)
		os.Exit(1)
	}
	return conn
}
