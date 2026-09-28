# Prutaj Board

**Multi-tenant task-board API for TheOnionOcean.**

Prutaj Board is the workspace’s first application: an Actix Web service for organizations, projects, teams, tasks, and users. It uses CQRS (`mediatr`), attribute validation (`pass_me`), SurrealDB helpers, and Zitadel introspection—wired in the onion layout described in the [repository README](../../../README.md).

| | |
|---|---|
| **Crate** | `prutaj_board` |
| **Nx project** | `prutaj_board` |
| **Default bind** | `localhost:5278` (via env) |
| **API prefix** | `/api/{resource}/…` |
| **OpenAPI** | [`specification.yml`](specification.yml) |
| **HTTP client** | [Bruno collection](bruno/Prutaj%20Board%20API) |

← [TheOnionOcean](../../../README.md)

---

## Table of contents

- [What you get](#what-you-get)
- [API surface](#api-surface)
- [Source layout](#source-layout)
- [Request flow](#request-flow)
- [Prerequisites](#prerequisites)
- [Configuration](#configuration)
- [Run](#run)
- [Develop](#develop)
- [Explore the API](#explore-the-api)
- [Related libraries](#related-libraries)

---

## What you get

- CRUD for **organization**, **project**, **team**, **task**, and **user**
- Feature folders: command/query + handler per use case
- Validation on write paths via `pass_me` (including async uniqueness-style `MustChecker` rules)
- Auditable entities (`created_at` / `modified_at` / `*_by`)
- Problem-details style HTTP errors from `MediatrError`
- Generated OpenAPI and a Bruno collection for local calls

---

## API surface

Each resource is mounted under its own scope. Pattern:

| Method | Path | Action |
|--------|------|--------|
| `POST` | `/api/{resource}/create` | Create |
| `GET` | `/api/{resource}/get/{id}` | Read by id |
| `PUT` | `/api/{resource}/update` | Update |
| `DELETE` | `/api/{resource}/delete` | Delete |

Resources: `organization` · `project` · `team` · `task` · `user`.

Exact bodies and response schemas live in OpenAPI / Bruno—prefer those over guessing field names.

---

## Source layout

```
apps/rust/prutaj_board/
├── src/
│   ├── main.rs              # Boot: env, SurrealDB, Zitadel, Actix
│   ├── lib.rs
│   ├── application/         # Features: commands, queries, handlers
│   │   └── features/
│   │       ├── organization/
│   │       ├── project/
│   │       ├── team/
│   │       ├── task/
│   │       └── user/
│   ├── entity/              # Domain models
│   ├── web/                 # Actix endpoints + scope registration
│   ├── openapi/             # utoipa assembly
│   └── bin/generate_openapi.rs
├── bruno/                   # HTTP collection
├── specification.yml        # Generated OpenAPI
└── project.json             # Nx targets
```

---

## Request flow

```
HTTP JSON
   │
   ▼
Actix endpoint  ──▶  Mediatr::send_command / send_query
   │                        │
   │                        ├─ handler.validate()  (pass_me)
   │                        └─ handler.handle()    (SurrealDB)
   ▼
Problem details ←── MediatrError
   or
JSON body ←── Response entity
```

Handlers register once at startup (`register_endpoints`). Endpoints only deserialize, dispatch, and map results to HTTP.

---

## Prerequisites

From the **repository root**:

1. Nix/`direnv` (or equivalent Rust toolchain) and `bun install`
2. Env files: `infra/prutaj_board/prutaj-board.env`, `infra/zitadel/zitadel.env`
3. Docker Compose stacks for SurrealDB and Zitadel

---

## Configuration

Loaded at process start (paths relative to repo root):

- `infra/prutaj_board/prutaj-board.env`
- `infra/zitadel/zitadel.env`

| Variable | Purpose |
|----------|---------|
| `APP_HOST` / `APP_PORT` | HTTP bind address |
| `SURREALDB_HOST` | SurrealDB WebSocket host (e.g. `localhost:8000`) |
| `SURREALDB_USER` / `SURREALDB_PASSWORD` | Root credentials |
| `SURREALDB_NAMESPACE` / `SURREALDB_NAME` | Namespace and database |
| `ZITADEL_EXTERNAL_DOMAIN` | Issuer host used for introspection |
| `ZITADEL_APPLICATION_KEY_ID` | JWT profile key id |
| `ZITADEL_APPLICATION_KEY` | JWT profile private key |
| `ZITADEL_APPLICATION_APP_ID` | Application id |
| `ZITADEL_APPLICATION_CLIENT_ID` | Client id |

Start from [`infra/prutaj_board/prutaj-board.env.example`](../../../infra/prutaj_board/prutaj-board.env.example). Do not commit real secrets.

---

## Run

```bash
# SurrealDB
docker compose -f infra/prutaj_board/docker-compose.yml \
  --env-file infra/prutaj_board/prutaj-board.env up -d

# Zitadel (+ Caddy) — required for introspection wiring
docker compose -f infra/zitadel/docker-compose.yml \
  --env-file infra/zitadel/zitadel.env up -d

# Hot reload (watches apps/rust/prutaj_board and lib/rust)
nx serve prutaj_board

# One-shot
cargo run -p prutaj_board
```

---

## Develop

```bash
nx build prutaj_board
nx lint prutaj_board
nx run prutaj_board:generate-openapi   # refreshes specification.yml
```

When adding a feature: command/query + handler under `application/features/…`, endpoint under `web/…`, register on the resource scope, then regenerate OpenAPI if the contract changed.

---

## Explore the API

| Artifact | Use |
|----------|-----|
| [`specification.yml`](specification.yml) | OpenAPI 3 contract (utoipa) |
| [`bruno/Prutaj Board API`](bruno/Prutaj%20Board%20API) | Ready-made requests + env |

---

## Related libraries

| Library | Role in this app |
|---------|------------------|
| [mediatr](../../../lib/rust/mediatr/README.md) | Command/query bus |
| [pass_me](../../../lib/rust/pass_me/README.md) | DTO validation |
| [surrealdb_extensions](../../../lib/rust/surrealdb_extensions/README.md) | DB context + CRUD |
| [macros_security_core](../../../lib/rust/macros/security/macros_security_core/README.md) | Role guards on handlers |
