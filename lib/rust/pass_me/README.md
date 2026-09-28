# pass_me

**Declarative validation for commands and DTOs.**

Annotate named fields, `#[derive(Validate)]`, then call `validate()` or async `validate_with(checker)`. This crate is the **public facade**—apps should depend on `pass_me` only.

| | |
|---|---|
| **Crate** | `pass_me` |
| **Nx project** | `pass_me` |
| **Internals** | `pass_me_core`, `pass_me_macros` |
| **Integrates with** | [mediatr](../mediatr/README.md) handler `validate` |
| **Used by** | [prutaj_board](../../../apps/rust/prutaj_board/README.md) |

← [TheOnionOcean](../../../README.md)

---

## Table of contents

- [Crate split](#crate-split)
- [How validation works](#how-validation-works)
- [Quick start](#quick-start)
- [Attribute reference](#attribute-reference)
- [Errors](#errors)
- [MustChecker](#mustchecker)
- [Dependency](#dependency)

---

## Crate split

| Crate | Role |
|-------|------|
| **pass_me** | Re-exports checkers, `ValidationError`, `MustChecker`, and `Validate` |
| `pass_me_core` | Runtime implementations (`Email`, `Length`, Luhn credit-card check, …) |
| `pass_me_macros` | Expands `#[derive(Validate)]` into `validate` / `validate_with` |

---

## How validation works

```
#[derive(Validate)] + field attributes
              │
              ▼
   validate()                 ── sync rules only
   validate_with(&checker)    ── sync + #[must] via MustChecker
              │
              ▼
   Vec<ValidationError>   (empty = success)
```

- Target must be a **struct with named fields**
- Multiple attributes on one field all run; errors accumulate
- Sync attributes feed `validate()`; `#[must]` is async-only and needs `validate_with`

---

## Quick start

```rust
use pass_me::{MustChecker, Validate, ValidationError};

#[derive(Validate)]
struct CreateUser {
    #[non_null_or_empty]
    #[max_length(max = 64)]
    name: String,

    #[email]
    email: String,

    #[matches(pattern = r"^\d+$")]
    age: String,

    #[must]
    username: String,
}

struct UniqueUsernameChecker;

impl MustChecker for UniqueUsernameChecker {
    async fn must(&self, field: &str, value: &str) -> bool {
        // true = accept; false = ValidationError for that field
        field != "username" || value != "taken"
    }
}

async fn run(cmd: &CreateUser) -> Vec<ValidationError> {
    cmd.validate_with(&UniqueUsernameChecker).await
}
```

Typical mediatr wiring: implement `CommandHandler::validate` by calling `command.validate_with(...)`.

---

## Attribute reference

| Attribute | Behavior |
|-----------|----------|
| `#[non_null_or_empty]` | Rejects empty strings |
| `#[equal(value = "...")]` | Must equal the literal |
| `#[not_equal(value = "...")]` | Must not equal the literal |
| `#[length(min = N, max = M)]` | Inclusive length range |
| `#[min_length(min = N)]` | Minimum length |
| `#[max_length(max = N)]` | Maximum length |
| `#[less_then(value = N)]` | Numeric upper bound (exclusive) |
| `#[less_then_or_equal(value = N)]` | Numeric upper bound (inclusive) |
| `#[greater_then(value = N)]` | Numeric lower bound (exclusive) |
| `#[greater_then_or_equal(value = N)]` | Numeric lower bound (inclusive) |
| `#[exclusive_between(min = N, max = M)]` | Numeric range, exclusive (`min < x < max`) |
| `#[inclusive_between(min = N, max = M)]` | Numeric range, inclusive (`min <= x <= max`) |
| `#[matches(pattern = r"...")]` | Regex must match |
| `#[email]` | Email format |
| `#[credit_card]` | Luhn check (spaces/dashes stripped) |
| `#[must]` | Async rule via `MustChecker::must(field, value)` |

Attribute spellings (`less_then`, `greater_then`, …) match the macro API as implemented.

---

## Errors

```rust
pub struct ValidationError {
    pub field: &'static str,
    pub error_message: String,
    pub code: String,
}
```

Display form: `[code] field: message`. `mediatr` wraps a list as `MediatrError::ValidationFailed`.

---

## MustChecker

```rust
pub trait MustChecker {
    fn must(&self, field: &str, value: &str)
        -> impl Future<Output = bool> + Send;
}
```

Use for DB uniqueness, remote checks, or any rule that needs state. Branch on `field` (the struct field name) when one checker covers multiple `#[must]` fields.

---

## Dependency

```toml
[dependencies]
pass_me = { path = "lib/rust/pass_me" }
```

```bash
nx build pass_me
cargo build -p pass_me
```
