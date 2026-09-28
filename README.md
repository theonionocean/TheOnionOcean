# TheOnionOcean

**Layered Rust backends, shared as a monorepo.**

TheOnionOcean is a Cargo + Nx workspace for building HTTP services with a clear onion-style layout: web → application (CQRS) → domain → infrastructure. Apps share libraries for mediation, validation, SurrealDB access, and Zitadel role checks—so each service stays thin and consistent.

| | |
|---|---|
| **Language** | Rust 2021 |
| **HTTP** | Actix Web |
| **Data** | SurrealDB |
| **Auth** | Zitadel (introspection) |
| **Orchestration** | Nx + Bun |
| **Dev shell** | Nix / direnv |
| **License** | [MIT](LICENSE-MIT) · [Apache-2.0](LICENSE-Apache-2.0) |

---

## Table of contents

- [Projects](#projects)
- [How it fits together](#how-it-fits-together)
- [Repository layout](#repository-layout)
- [Prerequisites](#prerequisites)
- [Getting started](#getting-started)
- [Day-to-day development](#day-to-day-development)
- [Infrastructure](#infrastructure)
- [Documentation map](#documentation-map)
- [Contributing](#contributing)
- [License](#license)

---

## Projects

### Application

| Project | What it does |
|---------|----------------|
| [**prutaj_board**](apps/rust/prutaj_board/README.md) | Task-board API: organizations, projects, teams, tasks, and users. OpenAPI + Bruno collection included. |

### Libraries

Depend on these facades from application code. Supporting crates (`*_core`, `*_macros`, derive helpers) stay internal.

| Project | What it does |
|---------|----------------|
| [**mediatr**](lib/rust/mediatr/README.md) | In-process CQRS bus—register handlers, send commands/queries, run validation before handle. |
| [**pass_me**](lib/rust/pass_me/README.md) | `#[derive(Validate)]` and field attributes (length, email, credit card, async `#[must]`, …). |
| [**surrealdb_extensions**](lib/rust/surrealdb_extensions/README.md) | WebSocket `DatabaseContext` plus create / read / update / delete helpers (ULID ids, audit-aware). |
| [**macros_security_core**](lib/rust/macros/security/macros_security_core/README.md) | `#[has_role("…")]` and `require_role` for Zitadel-authenticated Actix handlers. |

---

## How it fits together

```
                    ┌─────────────────────────┐
                    │      prutaj_board       │
                    │   Actix · OpenAPI · env │
                    └───────────┬─────────────┘
                                │
          ┌─────────────────────┼─────────────────────┐
          ▼                     ▼                     ▼
    ┌───────────┐        ┌───────────┐        ┌────────────────┐
    │  mediatr  │───────▶│  pass_me  │        │ surrealdb_ext. │
    │ commands  │        │ validate  │        │  context+CRUD  │
    │  queries  │        └───────────┘        └────────┬───────┘
    └───────────┘                                      │
          │                                            ▼
          │                              auditable entities
          ▼                              (macros_abstraction)
    ┌────────────────────┐
    │ macros_security_*  │◀── Zitadel introspection
    │   #[has_role]      │
    └────────────────────┘
```

Request path in an app:

1. **Web** — Actix route, auth extractors, OpenAPI annotations  
2. **Application** — `mediatr` dispatches a command or query; `pass_me` validates  
3. **Entity** — domain models with audit fields  
4. **Infrastructure** — SurrealDB via `surrealdb_extensions`, Zitadel via security macros / SDK  

---

## Repository layout

```
TheOnionOcean/
├── apps/rust/          # Deployable services
│   └── prutaj_board/
├── lib/rust/           # Shared crates (facades + internals)
│   ├── mediatr/
│   ├── pass_me/          # + pass_me_core, pass_me_macros
│   ├── surrealdb_extensions/
│   └── macros/
│       ├── derive/       # AuditibleEntity helpers
│       └── security/     # has_role + require_role
├── infra/              # Local Docker + env
│   ├── prutaj_board/   # SurrealDB compose + app env
│   └── zitadel/        # Zitadel + Caddy TLS proxy
├── vendor/             # Patched / vendored third-party crates
├── shell.nix           # Dev toolchain
├── Cargo.toml          # Workspace root
├── nx.json
└── package.json
```

---

## Prerequisites

- **Nix** + **direnv** (recommended), *or* a recent Rust toolchain, `cargo`, `clippy`, `rustfmt`
- **Docker** / Docker Compose
- **Bun** (Nx installs and scripts)

```bash
# From the repository root
direnv allow          # loads shell.nix; alternatively: nix-shell
bun install
```

The Nix shell provides Rust, Node/Bun, Docker-adjacent tools (`just`, `watchexec`, `surrealkit`, `mold`, `sccache`, …).

---

## Getting started

Fastest path to a running API:

```bash
# 1. Env (copy examples where needed)
cp infra/prutaj_board/prutaj-board.env.example infra/prutaj_board/prutaj-board.env
# Configure infra/zitadel/zitadel.env for auth

# 2. Data + identity
docker compose -f infra/prutaj_board/docker-compose.yml \
  --env-file infra/prutaj_board/prutaj-board.env up -d
docker compose -f infra/zitadel/docker-compose.yml \
  --env-file infra/zitadel/zitadel.env up -d

# 3. API with hot reload
nx serve prutaj_board
```

Full configuration, ports, and Bruno/OpenAPI details: **[Prutaj Board README](apps/rust/prutaj_board/README.md)**.

---

## Day-to-day development

```bash
# Build / lint a crate
nx build prutaj_board
nx lint pass_me
cargo build -p mediatr
cargo clippy -p surrealdb_extensions

# Run without watch
cargo run -p prutaj_board

# Regenerate OpenAPI YAML for Prutaj Board
nx run prutaj_board:generate-openapi
```

Prefer **public facade crates** from apps (`pass_me`, `macros_security_core`, …). Reach into `*_core` / `*_macros` only when you are changing those layers.

---

## Infrastructure

| Path | Purpose |
|------|---------|
| [`infra/prutaj_board`](infra/prutaj_board) | SurrealDB Compose stack and application env |
| [`infra/zitadel`](infra/zitadel) | Zitadel + Caddy (TLS) for local identity |

Env files are loaded by the app at startup (`prutaj-board.env`, `zitadel.env`). Keep secrets out of git; use `.example` files as templates.

---

## Documentation map

| Doc | Audience |
|-----|----------|
| **This file** | Repo identity, map of projects, bootstrap |
| [prutaj_board](apps/rust/prutaj_board/README.md) | Running and extending the API |
| [mediatr](lib/rust/mediatr/README.md) | CQRS registration and dispatch |
| [pass_me](lib/rust/pass_me/README.md) | Validation attributes and `MustChecker` |
| [surrealdb_extensions](lib/rust/surrealdb_extensions/README.md) | DB context and CRUD traits |
| [macros_security_core](lib/rust/macros/security/macros_security_core/README.md) | Role guards on handlers |

---

## Contributing

1. Enter the Nix/`direnv` shell so toolchains match.
2. Keep a change focused on one app or library when practical.
3. Document behavior and config changes in that project’s README.
4. Do not commit real credentials from `infra/**/*.env`.

---

## License

Copyright © 2026 TheOnionOcean.

Dual-licensed under **[MIT](LICENSE-MIT)** and **[Apache-2.0](LICENSE-Apache-2.0)**. You may choose either license.
