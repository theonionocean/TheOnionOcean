# Security macros

**Zitadel role gates for Actix handlers.**

`macros_security_core` is the public facade: import `has_role` and `require_role`, protect routes with a one-line attribute, or check roles manually against an introspected user.

| | |
|---|---|
| **Crate** | `macros_security_core` |
| **Nx project** | `macros_security_core` |
| **Macro crate** | `macros_security` |
| **Auth source** | Zitadel Actix introspection (`IntrospectedUser`) |
| **Used by** | [prutaj_board](../../../../../apps/rust/prutaj_board/README.md) |

← [TheOnionOcean](../../../../../README.md)

---

## Table of contents

- [Crate split](#crate-split)
- [How `#[has_role]` works](#how-has_role-works)
- [Usage](#usage)
- [Requirements](#requirements)
- [Dependency](#dependency)

---

## Crate split

| Crate | Role |
|-------|------|
| **macros_security_core** | `require_role`; re-exports `#[has_role]` |
| `macros_security` | Procedural attribute that rewrites the handler |

Application code should depend on **macros_security_core** only.

---

## How `#[has_role]` works

Applied to an Actix handler, the macro:

1. Inserts `IntrospectedUser` as the **first** parameter  
2. Calls `require_role(&user, "role_name")` before your body  
3. On failure, returns `403 Forbidden` with a JSON message  

```
#[has_role("org_owner")]
async fn handler(/* injected */ user, …) {
        │
        ▼
  require_role(user, "org_owner")
        │
        ├─ Ok  → run handler body
        └─ Err → HttpResponse::Forbidden
}
```

`require_role` looks up `role` in `IntrospectedUser.project_roles`. The name must match a key in the Zitadel project role map on the token.

---

## Usage

### Attribute on a handler

```rust
use macros_security_core::has_role;
use actix_web::{get, HttpResponse, Responder};

#[has_role("org_owner")]
#[get("/org/settings")]
async fn org_settings() -> impl Responder {
    HttpResponse::Ok().finish()
}
```

You can still declare other extractors; the introspected user is prepended automatically.

### Manual check

```rust
use macros_security_core::require_role;
use actix_web::HttpResponse;
use zitadel::actix::introspection::IntrospectedUser;

fn ensure_owner(user: &IntrospectedUser) -> Result<(), HttpResponse> {
    require_role(user, "org_owner")
}
```

---

## Requirements

1. **Zitadel Actix introspection** configured on the `App` (JWT profile / introspection builder—see Prutaj Board `main.rs`).
2. Workspace crates: `actix-web`, `zitadel` with feature `actix`.
3. Caller must present a token that includes the expected project roles after introspection.

Without introspection middleware/app data, the injected `IntrospectedUser` extractor will not resolve.

---

## Dependency

```toml
[dependencies]
macros_security_core = { path = "lib/rust/macros/security/macros_security_core" }
```

```bash
nx build macros_security_core
cargo build -p macros_security_core
```
