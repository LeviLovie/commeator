help:
    @just --list

# Run the client in web locally
web:
    cd client && dx serve --web --port=8000

# Run the macos client
macos:
    open ./client/target/dx/commeator/debug/macos/Commeator.app

# Build the macos client
b_macos:
    cd client && \
        dx build --macos
    cp config/macos/Info.plist client/target/dx/commeator/debug/macos/Commeator.app/Contents/Info.plist

# Publish the macos client
p_macos:
    @sh ./scripts/p_macos.sh

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

# Build web as a local docker image
d_build_web:
    docker build -f Dockerfile.web -t commeator-web:latest .

# Build server as a local docker image
d_build_server:
    docker build -f Dockerfile.server -t commeator-server:latest .

# Build web and publish to ghcr for amd64 in levilovie registry
d_publish_web:
    docker buildx build --platform linux/amd64 -t ghcr.io/levilovie/web-amd64:latest -f Dockerfile.web --push .

# Build server and publish to ghcr for amd64 in levilovie registry
d_publish_server:
    docker buildx build --platform linux/amd64 -t ghcr.io/levilovie/server-amd64:latest -f Dockerfile.server --push .

# Build both web and server docker images locally
d_build: d_build_web d_build_server

# Publish both web and server docker images to ghcr
d_publish: d_publish_web d_publish_server
