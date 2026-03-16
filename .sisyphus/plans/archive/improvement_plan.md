# OpenLark Improvement Plan

## Overview
This plan addresses the architectural issues identified in the audit report, focusing on separating concerns between `openlark-core` (infrastructure) and `openlark-auth` (business logic), and fixing critical bugs in `openlark-client`.

## Phase 0: Critical Fixes (Immediate)
**Goal**: Fix blocking bugs and remove fake implementations.

1.  **Fix `AuthService` Configuration in Client**
    *   **Target**: `crates/openlark-client/src/services/auth.rs`
    *   **Issue**: `AuthService::new` ignores the passed `Config` and uses `Default::default()`, resulting in empty credentials.
    *   **Action**: Update `new` to properly store and use the passed `config`.

2.  **Remove Simulated Implementations**
    *   **Target**: `crates/openlark-client/src/services/communication.rs`
    *   **Issue**: Methods like `send_text_message` call `simulate_send_message` which returns fake data.
    *   **Action**: Remove `simulate_*` methods. Replace with calls to the actual `openlark-communication` crate (if available) or unimplemented TODOs. *Note: The audit suggests moving them to examples/tests, but for the SDK source, they must go.*

3.  **Clean up Placeholder Tests**
    *   **Target**: `crates/openlark-auth/tests/`
    *   **Issue**: Files contain `disabled_test()` placeholders.
    *   **Action**: Remove these files or implement at least one basic test case.

## Phase 1: Core & Auth Refactoring
**Goal**: Move token management logic from `core` to `auth`.

1.  **Define `TokenProvider` Trait in Core**
    *   **Target**: `crates/openlark-core/src/auth/token_provider.rs` (New)
    *   **Action**: Create a trait `TokenProvider` that `Transport` can use to get an access token.
    *   **Signature**: `async fn get_token(&self, token_type: AccessTokenType) -> SDKResult<String>`

2.  **Refactor `Transport` to use `TokenProvider`**
    *   **Target**: `crates/openlark-core/src/http.rs`
    *   **Action**:
        *   Remove hardcoded dependencies on `token_manager` and `app_ticket_manager`.
        *   Inject `Arc<dyn TokenProvider>` into `Transport` (or via `Config`).
        *   Update `do_request` to fetch tokens via the provider.

3.  **Implement `TokenManager` in Auth**
    *   **Target**: `crates/openlark-auth/src/token/manager.rs` (New)
    *   **Action**:
        *   Port logic from `crates/openlark-core/src/token_manager.rs`.
        *   Port logic from `crates/openlark-core/src/app_ticket_manager.rs`.
        *   Implement `TokenProvider` for this new `TokenManager`.
        *   Implement actual caching using `quick_cache` (fix the "promised but not implemented" feature).

4.  **Clean up Core**
    *   **Target**: `crates/openlark-core/src/`
    *   **Action**: Remove `token_manager.rs`, `app_ticket_manager.rs`, and legacy `auth/` modules once migration is verified.

## Phase 2: Client Unification
**Goal**: Make `openlark-client` a proper facade.

1.  **Standardize Service Access**
    *   Ensure all services (Auth, Communication, etc.) in `client` are thin wrappers that:
        *   Accept `openlark_client::Config`.
        *   Convert it to `openlark_core::Config` (with the `TokenProvider` from `auth` wired up).
        *   Delegate to the underlying business crate.

2.  **Fix Re-exports**
    *   **Target**: `crates/openlark-core/src/prelude.rs`
    *   **Action**: Remove `pub use anyhow::Result;` to avoid confusion with `SDKResult`.

## Execution Order
1.  **Phase 0** can be executed immediately to fix broken functionality.
2.  **Phase 1** requires careful moving of code to avoid breaking the build.
3.  **Phase 2** depends on Phase 1.

