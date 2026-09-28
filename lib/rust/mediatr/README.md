# mediatr

**In-process CQRS for Rust services.**

`mediatr` is a typed command/query bus: register handlers once, then `send_command` / `send_query` from anywhere you hold a `Mediatr` (typically Actix `web::Data`). Optional async validation runs before `handle`, using [pass_me](../pass_me/README.md) error types.

| | |
|---|---|
| **Crate** | `mediatr` |
| **Nx project** | `mediatr` |
| **Depends on** | `pass_me` |
| **Used by** | [prutaj_board](../../../apps/rust/prutaj_board/README.md) |

← [TheOnionOcean](../../../README.md)

---

## Table of contents

- [Why use it](#why-use-it)
- [Core types](#core-types)
- [Dispatch pipeline](#dispatch-pipeline)
- [Usage](#usage)
- [Errors](#errors)
- [Dependency](#dependency)

---

## Why use it

- Keeps HTTP handlers thin: deserialize → dispatch → map result
- One registration site for all use cases (easy to audit)
- Validation is first-class: failed checks never reach `handle`
- `Clone` + `Send` + `Sync` — safe to share across Actix workers

---

## Core types

| Item | Role |
|------|------|
| `Command` / `Query` | Message markers with `type Response = …` |
| `CommandHandler` / `QueryHandler` | `validate` (default empty) + `handle` |
| `Mediatr` | Type-map of handlers; `register_*` / `send_*` |
| `MediatrError` | Missing handler, handler failure, or validation |

Handlers are keyed by the message’s `TypeId`. Registering twice for the same message type replaces the previous handler.

---

## Dispatch pipeline

```
send_command(C) / send_query(Q)
        │
        ▼
  lookup handler by TypeId
        │
        ▼
  handler.validate(&message)  ──▶  Vec<ValidationError>
        │                              │
        │                         non-empty?
        │                              │
        │                              └──▶ MediatrError::ValidationFailed
        ▼
  handler.handle(message)  ──▶  Result<Response, MediatrError>
```

---

## Usage

```rust
use mediatr::{Command, CommandHandler, Mediatr, MediatrError};
use pass_me::ValidationError;

struct CreateThing {
    pub name: String,
}

impl Command for CreateThing {
    type Response = String;
}

struct CreateThingHandler;

impl CommandHandler<CreateThing> for CreateThingHandler {
    async fn validate(&self, command: &CreateThing) -> Vec<ValidationError> {
        if command.name.is_empty() {
            vec![ValidationError {
                field: "name",
                error_message: "must not be empty".into(),
                code: "NotNullOrEmpty".into(),
            }]
        } else {
            vec![]
        }
    }

    async fn handle(&self, command: CreateThing) -> Result<String, MediatrError> {
        Ok(command.name)
    }
}

async fn example() -> Result<String, MediatrError> {
    let mut bus = Mediatr::default();
    bus.register_command(CreateThingHandler);

    bus.send_command(CreateThing {
        name: "board".into(),
    })
    .await
}
```

Queries mirror the same pattern with `Query`, `QueryHandler`, `register_query`, and `send_query`.

In apps, inject the DB (or other deps) into the handler struct at registration time—see Prutaj Board’s `register_endpoints`.

Pairing with derive validation:

```rust
// Inside CommandHandler::validate
command.validate_with(&MyMustChecker { db: &self.db }).await
```

---

## Errors

```rust
pub enum MediatrError {
    HandlerNotFound(String),
    HandlerFailed(String),
    ValidationFailed(Vec<ValidationError>),
}
```

Map these at the edge (Prutaj Board turns them into problem-details HTTP responses).

---

## Dependency

```toml
[dependencies]
mediatr = { path = "lib/rust/mediatr" }
```

```bash
nx build mediatr
cargo build -p mediatr
```
