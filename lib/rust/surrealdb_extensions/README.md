# surrealdb_extensions

**SurrealDB WebSocket context and CRUD traits for auditable records.**

Connect once with `DatabaseContext`, then create / read / update / delete through extension traits that speak Surreal’s record API and play nicely with audit fields from `macros_abstraction`.

| | |
|---|---|
| **Crate** | `surrealdb_extensions` |
| **Nx project** | `surrealdb_extensions` |
| **Engine** | SurrealDB remote WebSocket (`Client`) |
| **Used by** | [prutaj_board](../../../apps/rust/prutaj_board/README.md) |

← [TheOnionOcean](../../../README.md)

---

## Table of contents

- [What it provides](#what-it-provides)
- [DatabaseContext](#databasecontext)
- [CRUD traits](#crud-traits)
- [Audit fields](#audit-fields)
- [Errors](#errors)
- [Dependency](#dependency)

---

## What it provides

| Piece | Purpose |
|-------|---------|
| `DatabaseContext` | Connect, root sign-in, `use_ns` / `use_db`, expose `Surreal<Client>` |
| `CreateRecord` | Insert with a generated ULID record id |
| `ReadRecord` | Select one record by type + id |
| `ReadAllRecords` | List records of a type |
| `UpdateRecord` | Update by id (stamps `set_modified`) |
| `DeleteRecord` | Delete by type + id |
| `SurrealDbError` | Shared error type for these helpers |

Types used with create/update should implement `SurrealValue` and `AuditableEntity`.

---

## DatabaseContext

```rust
use surrealdb_extensions::DatabaseContext;

let ctx = DatabaseContext::new(
    "localhost:8000".into(),   // host for Ws
    "root".into(),
    "secret".into(),
    "prutaj_board".into(),     // namespace
    "prutaj-surrealdb".into(), // database
)
.await?;

let db = ctx.db(); // &Surreal<Client>
```

`DatabaseContext` is `Clone`—share it (or the inner client) when registering mediatr handlers.

---

## CRUD traits

Bring the traits into scope, then call them on your entity type / value:

```rust
use surrealdb_extensions::{CreateRecord, ReadRecord /* , … */};

// Create — id is a ULID string under the given table/record type
let saved = entity.create_record(db, "user").await?;

// Read
let user = User::read_record(db, "user", &id).await?;
```

| Trait | Behavior |
|-------|----------|
| `CreateRecord` | `db.create((type, ulid)).content(self)` |
| `ReadRecord` | `db.select((type, id))` |
| `ReadAllRecords` | Select all of a record type |
| `UpdateRecord` | `db.update((type, id)).content(…)` after `set_modified("Anonymous")` |
| `DeleteRecord` | Delete `(type, id)` |

Apply `set_created(actor)` (from `AuditableEntity`) before create when you want real actor names instead of defaults.

---

## Audit fields

Entities typically derive `AuditibleEntity` (via `macros_abstraction` / `macros_derive`) and include:

- `created_by` / `modified_by`: `String`
- `created_at` / `modified_at`: `chrono::DateTime<Utc>`

```rust
use macros_abstraction::AuditableEntity;

let entity = Entity::default().set_created("alice");
let saved = entity.create_record(db, "project").await?;
```

---

## Errors

Operations return `Result<T, SurrealDbError>`. Missing rows and Surreal client failures surface through this type—map them in your handler to `MediatrError` (or HTTP) as your app prefers.

---

## Dependency

```toml
[dependencies]
surrealdb_extensions = { path = "lib/rust/surrealdb_extensions" }
```

The consuming app should enable the SurrealDB features it needs (Prutaj Board uses workspace `surrealdb` with `kv-rocksdb` for the server side; the client here is remote WS).

```bash
nx build surrealdb_extensions
cargo build -p surrealdb_extensions
```
