# Feishu live HTTP and long connection

Maintainers prove a real Feishu call only with the ignored channel test or an example that is written to open a socket or send a message. Default verification drives the offline gates. Those gates return before any request. Env names are listed here; values are never written down.

## Sub-features

- `channel-offline` runs the ignored card-callback test with live names unset and shows the skip line.
- `channel-live` sends a card into a chat and waits on the long connection for a human click and a form submit.
- `ws-typed-offline` dispatches a fixture and skips `LarkWsClient::open`.
- `ws-echo-live` opens a long connection and can echo messages back through the HTTP API.
- `oauth-offline` builds a user-token option and skips `execute_with_options`.
- `first-api-live` sends a text message and also checks HTTP reachability of the Feishu host first.

## How to get to it (user POV)

- Offline channel gate (no request): `control-openlark e2e channel`.
- Offline typed long-connection example: `control-openlark e2e ws-typed`.
- Offline user-token example: `control-openlark e2e oauth`.
- Live channel card callback, from the repo root, only when a person is in the target chat:

```bash
export OPENLARK_CHANNEL_E2E=1
export OPENLARK_APP_ID
export OPENLARK_APP_SECRET
export OPENLARK_CHANNEL_CHAT_ID
export OPENLARK_CHANNEL_E2E_TIMEOUT_SECS
cargo test -p openlark-client --no-default-features --features channel --lib channel_card_callback_e2e -- --ignored --nocapture
```

- Live typed long connection (set `OPENLARK_WS_CONNECT=1` plus `OPENLARK_APP_ID` and `OPENLARK_APP_SECRET`):

```bash
cargo run --example websocket_typed_handler --no-default-features --features "communication,websocket"
```

- Live echo bot long connection (`OPENLARK_APP_ID` and `OPENLARK_APP_SECRET` required; the process calls `LarkWsClient::open`):

```bash
cargo run --example websocket_echo_bot --no-default-features --features "communication,websocket"
```

- Live user-token send (`OPENLARK_LIVE_USER_CALL=1`, `OPENLARK_APP_ID`, `OPENLARK_APP_SECRET`, `OPENLARK_USER_ACCESS_TOKEN`, `OPENLARK_RECEIVE_OPEN_ID`):

```bash
cargo run --example oauth_user_token --no-default-features --features "auth,communication"
```

- Live first text message (`OPENLARK_APP_ID`, `OPENLARK_APP_SECRET`; optional `OPENLARK_RECEIVE_ID`, `OPENLARK_RECEIVE_ID_TYPE`):

```bash
cargo run --example first_api_call --features "auth,communication"
```

## Driving it with control-openlark

Preconditions:

- `control-openlark doctor` printed `doctor=PASS`.
- Offline drives require the live switches to be unset. The harness unsets them for the child process.
- Live drives require the names in the matching bullet above and, for the channel test, a person who can click the card. Do not invent values. Do not paste values into evidence, commits, or the PR.

- **Channel offline.** Run `control-openlark e2e channel`. Exit code `0`. `e2e-channel.txt` contains `test channel::tests::channel_card_callback_e2e ... ok` and the line `跳过 channel 卡片回调端到端：未设置 OPENLARK_CHANNEL_E2E=1 或缺少 OPENLARK_APP_ID / OPENLARK_APP_SECRET / OPENLARK_CHANNEL_CHAT_ID`.
- **Typed WS offline.** Run `control-openlark e2e ws-typed`. Exit code `0`. The transcript contains `未设置 OPENLARK_WS_CONNECT=1，跳过真实长连接`.
- **OAuth offline.** Run `control-openlark e2e oauth`. Exit code `0`. The transcript contains `未设置 OPENLARK_LIVE_USER_CALL=1，跳过真实 API 调用`.
- **Channel live.** Do not use the harness. Export `OPENLARK_CHANNEL_E2E`, `OPENLARK_APP_ID`, `OPENLARK_APP_SECRET`, and `OPENLARK_CHANNEL_CHAT_ID`, optionally `OPENLARK_CHANNEL_E2E_TIMEOUT_SECS`, then run the `cargo test` line above. The process sends a card and opens the long connection. A person clicks 「确认」 and submits the printed nonce in the reason field. Exit code `0`, the transcript does not contain the skip line, and both the button value and the form reason matched the nonce. If the names are missing, the test returns ok after the skip line — record `verified-unreachable`, not a live pass.
- **Echo bot live.** Do not use `control-openlark example run`. Run the `websocket_echo_bot` command above with `OPENLARK_APP_ID` and `OPENLARK_APP_SECRET` set. The transcript contains `正在建立飞书长连接`. The process stays up until the socket closes. Missing credentials fail before the socket, which is `verified-unreachable`.
- **First API live.** Do not use the harness. Run the `first_api_call` command above. It contacts the Feishu host during its connectivity check and then sends a message. Missing `OPENLARK_APP_ID` or `OPENLARK_APP_SECRET` fails the example. Record that as `verified-unreachable`.
- **Proof.** Offline proof is the skip line plus exit code `0`. Live proof is the success text from that example or test and the absence of its skip line. Keep the transcript. Redact any value that was exported.

## Gotchas

- `cargo test --workspace --all-features` does not run `channel_card_callback_e2e`. The test is `#[ignore]` and the workspace log shows `ignored, 需要 OPENLARK_CHANNEL_E2E=1 和真实群里的人工点击`.
- Without `--nocapture`, cargo hides the skip line. An `ok` in `0.00s` can be the early return. Require the skip sentence before calling the run offline, and require its absence plus the click instructions before calling the run live.
- `OPENLARK_CHANNEL_E2E` must be the character `1`. Any other value takes the skip path even if the other names are set.
- `OPENLARK_CHANNEL_E2E_TIMEOUT_SECS` is optional. Empty, `0`, or a non-integer falls back to 180 seconds. A positive integer is used as-is.
- `OPENLARK_BASE_URL` is optional on the channel test and on `websocket_echo_bot`. Unset keeps the SDK default host.
- `first_api_call` performs HTTP requests whenever it starts. Do not run it to "see if credentials exist".
- `websocket_echo_bot` has no skip flag. With both app names set, it opens the long connection.
- The harness rejects `e2e --live`, `example run first_api_call`, and `example run websocket_echo_bot`.
- Do not claim a live round-trip from an offline skip.
