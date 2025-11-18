// WebSocket 客户端模块
//
// 重新导出openlark-core的WebSocket核心实现

#[cfg(feature = "websocket")]
pub use openlark_core::client::ws_client::LarkWsClient;

#[cfg(feature = "websocket")]
pub use openlark_core::client::ws_client::{
    ClientConfig, WsEvent, WsCloseReason, EndPointResponse, WsClientError,
    FrameHandler, WebSocketStateMachine, ConnectionState
};

#[cfg(test)]
mod tests;
