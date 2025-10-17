help:
    @just --list

# Run the client in web locally
web:
    cd client && dx serve --web --port=8000

# Run the server locally
server:
    cd server && cargo run

# Run centrifugo server locally
centrifugo:
    centrifugo --config ./config/centrifugo/config.yaml

# Test the server with neocurl
neocurl:
    cd server/neocurl && \
        ncurl test

# Run the full stack in docker compose
compose *args:
    docker compose {{ args }}

# Run only auth, postgres, and centrifugo services in docker compose
compose_dev *args:
    docker compose {{ args }} kratos centrifugo

# Remove rust build artifacts, those can get quite large
clean:
    find . -type d -name target -exec rm -rf {} +

# Migrate database up
m_up:
    sea-orm-cli migrate up

# Migrate database down
m_down:
    sea-orm-cli migrate down

# Destroy and migrate the database
m_fresh:
    sea-orm-cli migrate fresh

# Create a new migration
m_new NAME:
    sea-orm-cli migrate generate {{ NAME }}

# Generate rust entity files from the database schema
m_generate:
    sea-orm-cli generate entity -o server/src/entities
