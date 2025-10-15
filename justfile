serve:
    @cd web && dx serve --web --port=3000

neocurl:
    cd server/neocurl && \
        ncurl test

m_up:
    sea-orm-cli migrate up

m_down:
    sea-orm-cli migrate down

m_fresh:
    sea-orm-cli migrate fresh

m_new NAME:
    sea-orm-cli migrate generate {{ NAME }}

m_generate:
    sea-orm-cli generate entity -o server/src/entities
