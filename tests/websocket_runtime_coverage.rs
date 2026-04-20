//! openlark WebSocket 运行时覆盖测试入口。

#![cfg(feature = "websocket")]

#[path = "unit/websocket/websocket_integration_tests.rs"]
mod websocket_integration_tests;
