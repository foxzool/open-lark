---
name: openlark-formal-event-inbound
description: "Formally verify HttpEventInbound::handle auth-gate promises (no unauthenticated dispatch + decrypt-before-verify finding). Use when changing crates/openlark-client/src/event_inbound/handler.rs, crypto.rs, HTTP event inbound, challenge/signature/decrypt order; or when asked to re-run the event-inbound TLA+ check."
---

# Formal verification: HttpEventInbound handle

Read this cold mid-change. You do not need to know TLA+. Run the script, read the plain-language result, and only open the `.tla` file when the covered Rust steps changed.

## When to use

Use this skill if your change touches any of:

| Path | Why |
|------|-----|
| `crates/openlark-client/src/event_inbound/handler.rs` | `handle`, `extract_payload`, `verify_sign`, `auth_challenge` |
| `crates/openlark-client/src/event_inbound/crypto.rs` | `decrypt_event`, `verify_inbound_signature` (call sites / contract) |
| `crates/openlark-client/src/event_inbound/mod.rs` | exports / wiring of inbound |
| `formal/event_inbound/**` | the model or runner itself |

If none of those change, skip this skill.

## Property (plain language)

**Promise A — no unauthenticated dispatch:** When `encrypt_key` is non-empty and `skip_sign_verify` is false, an event (not `url_verification`) must not reach `dispatcher.dispatch_with_response` unless the inbound signature is Good. Harm if broken: forged Feishu HTTP callbacks run bot handlers.

**Promise B — challenge never dispatches as event:** `type == url_verification` must finish via `auth_challenge`, never the event dispatcher. Harm if broken: challenge traffic is treated as a business event.

**Promise C — verify before decrypt (intended, currently broken):** Traffic that still needs a signature check and presents Missing/Bad signature must not call `decrypt_event` first. Harm if broken: unauthenticated clients force AES decrypt + JSON parse before reject (CPU / oracle cost).

## Tool and why

**TLA+ / TLC** (model checker), not a Rust prover: the risk is the **decision table** of config × body shape × req type × signature × decrypt outcome. Unit tests cover happy paths; they do not enumerate every combination. There is no production code-level model checker for this pipeline. Crypto (AES/SHA256) is abstracted; TLC explores control-flow orders and input combinations up to the bound.

Pinned tool: **tla2tools v1.8.0** (`tla2tools.jar` from the tlaplus GitHub release). Requires **Java 17+** on `PATH`.

## One command

From the repo root:

```bash
./formal/event_inbound/run_tlc.sh
```

Success (exit 0) means:

1. `NoUnauthenticatedDispatch` and `ChallengeNeverDispatches` hold on the model of the current design.
2. `VerifyBeforeDecrypt` is still **violated** (known finding in current Rust — see below).

| Exit | Meaning |
|------|---------|
| 0 | Baseline: A+B hold; C still a finding |
| 1 | A/B failed, or TLC/tooling error — treat as regression or drift |
| 2 | Java missing |
| 3 | C unexpectedly holds — likely verify moved before decrypt; update this skill |

Logs: `formal/event_inbound/last-pass.log`, `formal/event_inbound/last-decrypt-order.log`.

## Known finding (do not “fix” by weakening the check)

Current `handle` order (envelope + key set):

1. `extract_payload` → ciphertext string
2. `decrypt_event` (**before** signature check)
3. `FuzzyEvent` JSON parse
4. `verify_sign` on the **raw** HTTP body
5. dispatch or reject

Counterexample in code words (from TLC):

1. `encrypt_key` set, `skip_sign_verify` false, body is `{"encrypt":"..."}`, `type` is an event (not challenge).
2. Signature header Missing or Bad.
3. `handle` still enters `decrypt_event` (`did_decrypt`) before `AuthGate` / `verify_sign`.

**Do not** edit the product to silence this unless the user asks for a fix. **Do not** remove `VerifyBeforeDecrypt` from the suite to make the script green. If you move `verify_sign` before `decrypt_event` on purpose, change the runner expectation (exit 3 path) and record the new sync commit after TLC shows `VerifyBeforeDecrypt` holds.

## Code ↔ model map

| Rust (`handler.rs` / `crypto.rs`) | TLA+ step |
|-----------------------------------|-----------|
| `extract_payload` (empty key / envelope / blank / plain) | `Extract` |
| `decrypt_event` when key set + envelope | `Decrypt` (`did_decrypt`) |
| `FuzzyEvent` parse + empty-key nested encrypt guard | `Parse` |
| `verify_sign` when event + key + !skip | `AuthGate` / `NeedsSign` |
| `auth_challenge` | `Finish` → `ChallengeEcho` |
| `dispatcher.dispatch_with_response` | `Finish` → `Dispatched` |
| `verify_inbound_signature` Ok/Err / missing headers | `sig` ∈ {Good, Bad, Missing} |

Indivisible in both: one `extract_payload`; one `decrypt_event`; one parse; one `verify_sign`; one finish. Names: `HttpEventInbound`, `handle`, `extract_payload`, `verify_sign`, `auth_challenge`, `decrypt_event`.

## Bounds and assumptions

- **One request** (no concurrent `handle` interleaving).
- Discrete abstract outcomes: body kind, req type, signature class, cipher Ok/Fail, challenge token Ok/Fail — not real AES/SHA256 bits.
- Does **not** model: constant-time compare internals, header case folding, UTF-8 rejection, typed dispatcher routing, WebSocket path, or empty-`encrypt_key` plaintext (platform: no signature) beyond “dispatch allowed when key unset”.
- A pass covers this model and decision table, not the whole SDK.

## Sync commit

Last confirmed that this verification describes the code:

- **sync commit:** `REPLACE_AFTER_COMMIT` (model + `HttpEventInbound::handle` confirmed together; advance only after reconfirming)
- **covered functions:** `HttpEventInbound::handle`, `extract_payload`, `verify_sign`, `auth_challenge`, `decrypt_event` (call site / ordering only)

Move the sync commit forward only after you confirm the model still matches those functions at the new commit (re-read the Rust, update the `.tla` if steps changed, re-run `./formal/event_inbound/run_tlc.sh`).

## If the check fails

1. Read which invariant failed in the log.
2. Replay the TLC trace as Rust calls (`extract_payload` / `decrypt_event` / `verify_sign` / dispatch).
3. **Real bug:** every step is possible in the current Rust → report it; do not weaken the property.
4. **Modeling error:** a step cannot happen in Rust → fix `formal/event_inbound/EventInbound.tla` (or cfg), not the product, unless the user wants a product fix.
5. After intentional verify-before-decrypt: expect exit 3 until you update the runner to require `VerifyBeforeDecrypt` to pass and clear the finding section.

## After you change covered code

1. Diff `handler.rs` (and decrypt call order in it) against the sync commit.
2. Update the TLA+ steps so each still matches one Rust step.
3. Run `./formal/event_inbound/run_tlc.sh`.
4. Advance the sync commit in this file only if the run matches the expected baseline (or the new intentional contract you documented).

For drift after merges you did not recheck, use `/maintain-formal-verification-skill`.
