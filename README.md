# Oktosync Server

[![GitHub repo](https://img.shields.io/badge/github-nicolae--nicolaev/oktosync--server-18ffd8?style=flat-square&logo=github)](https://github.com/nicolae-nicolaev/oktosync-server)

Oktosync is an open-source, self-hostable file sync system.  
This repository contains the **server component**, which provides the APIs and storage backend for Oktosync clients.

⚠️ **Status:** Work in Progress. Expect breaking changes until v1.0.0.

---

## Roadmap & Releases

We are incrementally developing Oktosync Server.  
Below is the planned release roadmap. Each milestone will be tagged and documented as a [GitHub Release](https://github.com/nicolae-nicolaev/oktosync-server/releases).

### v0.1.0 – MVP Auth (current)
- ✅ User model (users, credentials, sessions)
- ✅ Signup, login, logout endpoints
- ✅ `/health` endpoint

### v0.2.0 – File Upload
- [ ] Prepare-upload API (check missing chunks by hash)
- [ ] Chunk storage (S3/MinIO support)
- [ ] Commit file manifests

### v0.3.0 – Sync Basics
- [ ] Change tracking
- [ ] Conflict handling
- [ ] Version history

### v0.4.0 – Devices
- [ ] Device registration (one row per client)
- [ ] Sessions tied to devices
- [ ] Device revocation

### v0.5.0 – Shared Spaces (experimental)
- [ ] Space model (private/shared)
- [ ] Membership management
- [ ] Encrypted Space Root Keys (SRKs)

---

## Contributing

Contributions, ideas, and feedback are welcome!  
Open an [issue](https://github.com/nicolae-nicolaev/oktosync-server/issues) or start a [discussion](https://github.com/nicolae-nicolaev/oktosync-server/discussions).

---

## Description

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
