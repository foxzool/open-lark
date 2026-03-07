# OpenLark Webhook Examples

This directory contains examples demonstrating how to use the OpenLark webhook bot module.

## Examples

### 1. Text Message (`webhook_text_message.rs`)

Send a simple text message via webhook.

```bash
cargo run --example webhook_text_message -p openlark-webhook
```

**Features demonstrated:**
- Basic webhook client setup
- Sending text messages
- Response handling

### 2. Card Message (`webhook_card_message.rs`)

Send an interactive card message via webhook (requires `card` feature).

```bash
cargo run --example webhook_card_message -p openlark-webhook --features card
```

**Features demonstrated:**
- Card message creation
- JSON payload construction
- Feature-gated functionality

### 3. Signature Verification (`webhook_with_signature.rs`)

Send messages with signature verification (requires `signature` feature).

```bash
cargo run --example webhook_with_signature -p openlark-webhook --features signature
```

**Features demonstrated:**
- HMAC-SHA256 signature verification
- Security best practices
- Feature-gated security features

### 4. Error Handling (`webhook_error_handling.rs`)

Proper error handling patterns for webhook operations.

```bash
cargo run --example webhook_error_handling -p openlark-webhook
```

**Features demonstrated:**
- Error handling with match expressions
- Error type inspection
- Graceful failure handling

## Setup

Before running examples, replace placeholder webhook URLs:

```rust
let webhook_url = "https://open.feishu.cn/open-apis/bot/v2/hook/YOUR_WEBHOOK_KEY".to_string();
```

Get your webhook URL from:
1. Open Feishu/Lark workspace
2. Create a custom bot in group settings
3. Copy the webhook URL

## Features

Enable features in Cargo.toml or via command line:

```bash
cargo run --example webhook_text_message -p openlark-webhook --features "card,signature"
```

Available features:
- `card` - Interactive card message support
- `signature` - HMAC-SHA256 signature verification
- `robot` - Webhook robot functionality (default)

## Common Patterns

### Builder Pattern

All examples use the builder pattern for clean, chainable API:

```rust
SendWebhookMessageRequest::new(webhook_url)
    .text("Hello".to_string())
    .execute()
    .await?
```

### Error Handling

Use `Result` type for proper error propagation:

```rust
match request.execute().await {
    Ok(response) => println!("Success: {}", response.msg),
    Err(e) => eprintln!("Error: {}", e),
}
```

## Testing

Run all examples to verify compilation:

```bash
cargo check -p openlark-webhook --examples
cargo check -p openlark-webhook --examples --features "card,signature"
```
