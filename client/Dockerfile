# Phase 1: Cache deps
FROM rust:1 AS chef
RUN cargo install cargo-chef --locked
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json



# Phase 2: Load deps and build
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .

## Install the dx cli
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall dioxus-cli --version 0.7.0-rc.1 --root /.cargo --no-confirm --force
ENV PATH="/.cargo/bin:$PATH"

## Build...
RUN dx bundle @client --web --release



# Phase 3: Create the final image
FROM nginx:alpine AS runtime

COPY --from=builder /app/target/dx/commeator/release/web/public/ /usr/share/nginx/html/

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
