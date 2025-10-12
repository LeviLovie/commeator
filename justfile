serve:
    @dx serve --web --port=3000

up:
    sea-orm-cli migrate up

down:
    sea-orm-cli migrate down

fresh:
    sea-orm-cli migrate fresh

migration NAME:
    sea-orm-cli migrate generate {{ NAME }}

generate:
    sea-orm-cli generate entity -o src/backend/entities
