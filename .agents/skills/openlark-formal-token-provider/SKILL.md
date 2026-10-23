---
name: openlark-formal-token-provider
description: "Formally verify AuthTokenProvider::get_or_fetch concurrency promises (single-flight finding + failed-fetch must not write cache). Use when changing crates/openlark-auth/src/token_provider.rs, token cache/refresh, TokenProvider, or concurrent auth; or when asked to re-run the token-provider TLA+ check."
---

# Formal verification: AuthTokenProvider get_or_fetch

Read this cold mid-change. You do not need to know TLA+. Run the script, read the plain-language result, and only open the `.tla` file when the covered Rust steps changed.

## When to use

Use this skill if your change touches any of:

| Path | Why |
|------|-----|
| `crates/openlark-auth/src/token_provider.rs` | `get_or_fetch`, `get_cached`, `set_cached`, `CachedToken` |
| `crates/openlark-auth/src/token_provider/tests.rs` | only if it documents a new concurrency contract |
| `crates/openlark-core/src/auth/token_provider.rs` | `TokenProvider` trait / request shape that drives cache keys |
| `crates/openlark-core/src/auth/acquisition.rs` | callers that share one `Arc<dyn TokenProvider>` across tasks |
| `formal/token_provider/**` | the model or runner itself |

If none of those change, skip this skill.

## Property (plain language)

**Promise A — failed fetch does not write:** If `fetch().await` returns `Err`, that call must not insert into the token `HashMap`. Harm if broken: a failed refresh poisons the cache with a bogus token (or blocks recovery until skew expiry).

**Promise B — single-flight (intended, currently broken):** For one `cache_key`, at most one in-flight Feishu token HTTP fetch at a time. Harm if broken: concurrent cold/expired callers stampede the token endpoint → 429 / outages; overlapping writes last-writer-wins.

## Tool and why

**TLA+ / TLC** (model checker), not a Rust prover: the risk is bad interleaving of `get_cached` → unlocked `fetch` → `set_cached`, which unit tests almost never enumerate. There is no production-ready code-level model checker for `async` + `tokio::sync::RwLock` here. The model mirrors the Rust steps; TLC explores every order up to the bound.

Pinned tool: **tla2tools v1.8.0** (`tla2tools.jar` from the tlaplus GitHub release). Requires **Java 17+** on `PATH`.

## One command

From the repo root:

```bash
./formal/token_provider/run_tlc.sh
```

Success (exit 0) means:

1. `FailedFetchDoesNotWrite` holds on the model of the current design.
2. `SingleFlight` is still **violated** (known finding in current Rust — see below).

| Exit | Meaning |
|------|---------|
| 0 | Baseline: A holds; B still a finding |
| 1 | A failed, or TLC/tooling error — treat as regression or drift |
| 2 | Java missing |
| 3 | B unexpectedly holds — likely single-flight was implemented; update this skill |

Logs: `formal/token_provider/last-pass.log`, `formal/token_provider/last-singleflight.log`.

## Known finding (do not “fix” by weakening the check)

Current `get_or_fetch` (read lock → drop → `fetch` → write lock) allows a stampede.

Counterexample in code words (from TLC):

1. Caller `p1` and `p2` share one `AuthTokenProvider` / one `cache_key`.
2. Both call `get_cached`; cache empty (or expired) → both miss.
3. Both call `fetch()` with no lock held → two HTTP token requests.
4. Both may call `set_cached`; last writer wins.

**Do not** edit the product to silence this unless the user asks for a fix. **Do not** remove `SingleFlight` from the suite to make the script green. If you implement single-flight on purpose, change the runner expectation (exit 3 path) and record the new sync commit after TLC shows `SingleFlight` holds.

## Code ↔ model map

| Rust (`token_provider.rs`) | TLA+ step |
|----------------------------|-----------|
| `get_cached` + `CachedToken::is_expired` | `CacheHit` / `CacheMiss` |
| `fetch().await` while lock not held | `BeginFetch` + `FetchSucceed` / `FetchFail` |
| `set_cached` after `Ok` | `WriteCache` |
| `?` on fetch error (no insert) | `FetchFail` leaves `cache_token` unchanged |
| `expires_at = now + saturating_sub(expires_in, 60)` | `EffectiveTTL` / `SkewSecs` |

Indivisible in both: one `get_cached` call; one `fetch` await; one `set_cached` insert. Names: `AuthTokenProvider`, `CachedToken`, `get_or_fetch`, `cache_key`.

## Bounds and assumptions

- **2 concurrent callers**, one cache key.
- Discrete clock `MaxTime = 2`; `ExpiresIn = 120`, `SkewSecs = 60` (same 60s skew as `CachedToken::new`).
- Fetch success/failure is nondeterministic; network timing is only “before/after” steps, not wall ms.
- Does **not** model: marketplace nested app→tenant fetch, `client_assertion` path, real HTTP, `RwLock` writer queueing fairness, or cross-key isolation (separate keys are out of scope).
- **#693 user flow is modeled**: procs in `UserProcs` take the ReadRefresh → refresh-fetch → paired-write path (`UserEntryPairsRefresh`); plain and user entries share one modeled cache slot (real Rust uses distinct keys).
- A pass covers this model and bound, not the whole SDK.

## Sync commit

Last confirmed that this verification describes the code:

- **sync commit:** `17f8acc6b` (model + `AuthTokenProvider` confirmed together, incl. #693 user-token flow; advance only after reconfirming)
- **covered functions:** `CachedToken::new`, `CachedToken::new_with_refresh`, `CachedToken::is_expired`, `AuthTokenProvider::get_cached`, `set_cached`, `set_cached_with_refresh`, `cached_refresh_token`, `get_or_fetch`, `get_or_refresh_user_token`

Move the sync commit forward only after you confirm the model still matches those functions at the new commit (re-read the Rust, update the `.tla` if steps changed, re-run `./formal/token_provider/run_tlc.sh`).

## If the check fails

1. Read which invariant failed in the log.
2. Replay the TLC trace as Rust calls (`get_cached` / `fetch` / `set_cached`).
3. **Real bug:** every step is possible in the current Rust → report it; do not weaken the property.
4. **Modeling error:** a step cannot happen in Rust → fix `formal/token_provider/TokenProvider.tla` (or cfg), not the product, unless the user wants a product fix.
5. After intentional single-flight: expect exit 3 until you update the runner to require `SingleFlight` to pass and clear the finding section.

## After you change covered code

1. Diff `token_provider.rs` against the sync commit.
2. Update the TLA+ steps so each still matches one Rust step.
3. Run `./formal/token_provider/run_tlc.sh`.
4. Advance the sync commit in this file only if the run matches the expected baseline (or the new intentional contract you documented).

For drift after merges you did not recheck, use `/maintain-formal-verification-skill`.
