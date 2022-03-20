package main

import (
	"context"
	"encoding/json"
	"log"
	"net/http"
	"os"
	"os/signal"
	"syscall"
	"time"

	"github.com/go-chi/chi/v5"
	"github.com/jmoiron/sqlx"
	_ "github.com/lib/pq"
)

const (
	TIMEOUT_SERVER = 30 * time.Second
	TIMEOUT_READ   = 15 * time.Second
	TIMEOUT_WRITE  = 10 * time.Second
	TIMEOUT_IDLE   = 5 * time.Second
	DATABASE_URL   = "postgres://postgres:password@127.0.0.1:5432/starwars?sslmode=disable"
)

type Application struct {
	router *chi.Mux
}

func NewApplication() *Application {
	db, err := sqlx.Connect("postgres", DATABASE_URL)
	if err != nil {
		log.Fatal("Error connecting to database", err)
	}
	log.Println("Connected to database")

	r := chi.NewRouter()
	r.Route("/heartbeat", func(r chi.Router) {
		r.Get("/", heartbeat)
	})
	r.Route("/species", func(r chi.Router) {
		r.Get("/", fetchSpecies(db))
	})

	return &Application{r}
}

func (app *Application) Start() {
	channel := make(chan os.Signal, 1)
	ctx, cancel := context.WithTimeout(
		context.Background(),
		TIMEOUT_SERVER,
	)
	defer cancel()

	server := &http.Server{
		Addr:         "127.0.0.1:8080",
		Handler:      app.router,
		ReadTimeout:  TIMEOUT_READ,
		WriteTimeout: TIMEOUT_WRITE,
		IdleTimeout:  TIMEOUT_IDLE,
	}

	signal.Notify(channel, os.Interrupt, syscall.SIGTSTP)

	log.Printf("Server running on localhost:8080\n")

	go func() {
		if err := server.ListenAndServe(); err != nil {
			if err != http.ErrServerClosed {
				log.Fatalf("Server failed to start: %v\n", err)
			}
		}
	}()

	interrupt := <-channel

	log.Printf("Server is shutting down gracefully: %v\n", interrupt)
	if err := server.Shutdown(ctx); err != nil {
		log.Fatalf("Error during server shutdown: %v\n", err)
	}
}

func heartbeat(w http.ResponseWriter, r *http.Request) {
	w.Write([]byte("Ok"))
}

func fetchSpecies(db *sqlx.DB) func(http.ResponseWriter, *http.Request) {
	var results []*Species
	rows, err := db.Queryx("SELECT id, name FROM species LIMIT 3")
	if err != nil {
		log.Fatalf("Database error: %v\n", err)
	}

	for rows.Next() {
		var row Species
		err := rows.StructScan(&row)
		if err != nil {
			log.Fatalf("Error while deserializing species object: %v\n", err)
		}
		results = append(results, &row)
	}

	return func(w http.ResponseWriter, r *http.Request) {
		data, err := json.Marshal(results)
		if err != nil {
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}
		w.Header().Set("Content-type", "application/json")
		w.WriteHeader(http.StatusOK)
		w.Write(data)
	}
}
