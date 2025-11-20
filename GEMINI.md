# GEMINI.md

This file provides a comprehensive overview of the `open-lark` project, its structure, and development conventions to be used as instructional context for future interactions.

## Project Overview

`open-lark` is an unofficial, enterprise-grade SDK for the [Lark (Feishu) Open Platform](https://open.feishu.cn/), written in Rust. It provides a comprehensive, modular, and high-performance interface for interacting with Lark's various services.

The project recently underwent a major architectural migration from a monolithic structure to a modern, multi-crate workspace. This modular design enhances maintainability, performance, and allows developers to include only the necessary components for their applications.

### Key Technologies and Features

*   **Asynchronous:** Built on top of `tokio`, all API calls are asynchronous.
*   **Modular Architecture:** The project is a Cargo workspace with 22 distinct crates, each corresponding to a specific Lark service (e.g., `openlark-communication`, `openlark-docs`, `openlark-hr`).
*   **Feature Flags:** The SDK uses feature flags extensively to allow for conditional compilation of its various services, minimizing the final binary size.
*   **Builder Pattern:** A fluent builder pattern is used for constructing API requests, providing a more ergonomic and type-safe development experience.
*   **Comprehensive Error Handling:** The SDK includes a unified error handling mechanism.
*   **WebSocket Support:** Includes support for Lark's WebSocket-based event subscription service.
*   **Observability:** The project has built-in support for tracing and OpenTelemetry.
*   **Configuration:** Application credentials and settings are managed via a `.env` file.

### Core Crates

*   `openlark-core`: Provides the foundational infrastructure, including the HTTP client, configuration management, and error handling.
*   `openlark-client`: A high-level client that encapsulates service registration and provides a unified entry point to all the available services.
*   `openlark-protocol`: Defines the WebSocket protocol for real-time event handling.

## Building and Running

The project uses `just` as a command runner to simplify common development tasks.

### Prerequisites

Ensure you have `just` and the Rust toolchain installed.

### Common Commands

*   **Format Code:**
    ```bash
    just fmt
    ```
*   **Check Formatting:**
    ```bash
    just fmt-check
    ```
*   **Lint Code:**
    ```bash
    just lint
    ```
*   **Run Tests:**
    ```bash
    just test
    ```
*   **Run Tests for Specific Feature Combinations:**
    ```bash
    just test-features-quick
    ```
*   **Build Project:**
    ```bash
    just build
    ```
*   **Build in Release Mode:**
    ```bash
    just build-release
    ```
*   **Generate Documentation:**
    ```bash
    just docs
    ```
*   **Run Code Coverage Analysis:**
    ```bash
    just coverage
    ```
*   **Run Security Audit:**
    ```bash
    just audit
    ```
*   **Run All Pre-release Checks:**
    ```bash
    just check-all
    ```

### Running Examples

To run an example, first, create a `.env` file from the `.env-example` and fill in your Lark application credentials. Then, you can run an example using `cargo run --example <example_name>`. For instance:

```bash
cp .env-example .env
# Edit .env with your app_id and app_secret
cargo run --example basic_introduction
```

## Development Conventions

*   **Formatting:** The project uses `rustfmt` for code formatting. Configuration is in `.cargo/config.toml`.
*   **Linting:** `clippy` is used for linting. Configuration is in `.clippy.toml`. All warnings are treated as errors.
*   **Testing:** The project has a comprehensive test suite in the `/tests` directory, including integration and property-based tests. The `just test-features` command ensures that different feature combinations work as expected.
*   **Documentation:** All public APIs are documented. The `just docs` command can be used to generate the documentation locally.
*   **Conventional Commits:** The project follows the Conventional Commits specification for its commit messages.
*   **Changelog:** The `CHANGELOG.md` is manually updated for each release.
*   **Releasing:** The `just release <VERSION>` command automates the release process, which includes running all checks, tagging the release, and pushing it to the remote repository.
