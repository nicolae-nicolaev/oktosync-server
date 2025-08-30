# oktosync-server

Oktosync is a secure, server–client file synchronization platform written in Rust (Axum).  
This repository contains the **server** component (API + DB access + migrations).

## Features (MVP)

- Rust / Axum HTTP server
- Postgres storage (via SQLx)
- Config via file **and** environment variables with clear precedence
- Dockerized dev/prod workflow
- Healthcheck endpoint: `/health`

---

## Quick Start

### Using the helper script

```bash
# one-time
chmod +x run.sh

# start the stack
./run.sh up

# tail logs
./run.sh logs

# stop
./run.sh down

# nuke volumes + local images
./run.sh destroy
```

After up the server should be available at:
```bash
http://127.0.0.1:3000/
http://127.0.0.1:3000/health
```
```
```

## Manual (docker compose)

```bash
# Generate a Postgres password if you don't have one yet:
mkdir -p secrets
openssl rand -base64 32 > secrets/pg_password.txt
chmod 600 secrets/pg_password.txt

# Start the stack
docker compose up -d --build

# View logs
docker compose logs -f server
```

## Configuration

The server reads configuration from (in this order of precedence):
- Environment variables (highest)
- config/default.yml (if present in the image)
- Code defaults do not exist (fail-fast by design)

### Environment Variables
> Important: Because we set prefix_separator("_") and separator("__"), use single underscore after the prefix and double underscore for nesting.
- `OKTOSYNC_SERVER__HOST=0.0.0.0`
- `OKTOSYNC_SERVER__PORT`=3000
- `OKTOSYNC__DATABASE__URL=postgres://okto:***@db:5432/oktosyncdb`

The server also understands DATABASE_URL (prioritized by the app’s resolver). In the provided compose, DATABASE_URL is constructed from the secret at container start.

### Config keys
```yaml
# config/default.yml
server:
  host: "0.0.0.0"   # in Docker, bind to all interfaces
  port: 3000
database:
  url: "postgres://okto:<password>@db:5432/oktosyncdb"
```

## Development
### Local build
```bash
# build
cargo build

# run (expects configuration available)
cargo run
```

### Docker

**Dockerfile** is multi-stage; the runtime image is Debian slim with just the binary + migrations.

If you **don’t** want file-based config in the container, remove this line from the runtime stage:
```Dockerfile
COPY config ./config
```
…and rely entirely on env vars.
