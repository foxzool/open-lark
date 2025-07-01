use lark_websocket_protobuf::pbbp2::{Frame, Header};
use log::{debug, error, trace};
use serde::{Deserialize, Serialize};
use std::time::Instant;

use crate::{
    client::ws_client::{ClientConfig, WsEvent},
    event::dispatcher::EventDispatcherHandler,
};

/// Frame 类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FrameType {
    Control = 0,
    Data = 1,
}

/// Frame 处理器，负责处理不同类型的 Frame
pub struct FrameHandler;

impl FrameHandler {
    /// 处理接收到的 Frame
    pub async fn handle_frame(
        frame: Frame,
        event_handler: &EventDispatcherHandler,
        event_tx: &tokio::sync::mpsc::UnboundedSender<WsEvent>,
    ) -> Option<Frame> {
        match frame.method {
            0 => Self::handle_control_frame(frame),
            1 => Self::handle_data_frame(frame, event_handler, event_tx).await,
            _ => {
                error!("Unknown frame method: {}", frame.method);
                None
            }
        }
    }

    /// 处理控制帧
    fn handle_control_frame(frame: Frame) -> Option<Frame> {
        let headers = &frame.headers;
        let frame_type = Self::get_header_value(headers, "type")?;

        trace!("Received control frame: {frame_type}");

        match frame_type.as_str() {
            "pong" => Self::handle_pong_frame(frame),
            _ => {
                debug!("Unhandled control frame type: {frame_type}");
                None
            }
        }
    }

    /// 处理 Pong 帧
    fn handle_pong_frame(frame: Frame) -> Option<Frame> {
        let payload = frame.payload.as_ref()?;

        match serde_json::from_slice::<ClientConfig>(payload) {
            Ok(config) => {
                debug!("Received pong with config: {config:?}");
                // 返回配置信息供上层处理
                Some(frame)
            }
            Err(e) => {
                error!("Failed to parse ClientConfig from pong frame: {e:?}");
                None
            }
        }
    }

    /// 处理数据帧
    async fn handle_data_frame(
        mut frame: Frame,
        event_handler: &EventDispatcherHandler,
        _event_tx: &tokio::sync::mpsc::UnboundedSender<WsEvent>,
    ) -> Option<Frame> {
        let headers = &frame.headers;

        // 提取必要的头部信息
        let msg_type = Self::get_header_value(headers, "type").unwrap_or_default();
        let msg_id = Self::get_header_value(headers, "message_id").unwrap_or_default();
        let trace_id = Self::get_header_value(headers, "trace_id").unwrap_or_default();

        let Some(payload) = frame.payload else {
            error!("Data frame missing payload");
            return None;
        };

        debug!(
            "Received data frame - type: {msg_type}, message_id: {msg_id}, trace_id: {trace_id}"
        );

        match msg_type.as_str() {
            "event" => {
                let response = Self::process_event(payload, event_handler).await;

                // 添加处理时间到响应头
                if let Some(biz_rt) = response.headers.get("biz_rt") {
                    frame.headers.push(Header {
                        key: "biz_rt".to_string(),
                        value: biz_rt.clone(),
                    });
                }

                // 序列化响应
                frame.payload = Some(serde_json::to_vec(&response).unwrap_or_else(|e| {
                    error!("Failed to serialize response: {e:?}");
                    vec![]
                }));

                // 返回响应帧供上层发送
                Some(frame)
            }
            "card" => {
                debug!("Card frame received, skipping");
                None
            }
            _ => {
                debug!("Unknown data frame type: {msg_type}");
                None
            }
        }
    }

    /// 处理事件
    async fn process_event(
        payload: Vec<u8>,
        event_handler: &EventDispatcherHandler,
    ) -> NewWsResponse {
        let start = Instant::now();

        match event_handler.do_without_validation(payload) {
            Ok(_) => {
                let elapsed = start.elapsed().as_millis();
                let mut response = NewWsResponse::ok();
                response
                    .headers
                    .insert("biz_rt".to_string(), elapsed.to_string());
                response
            }
            Err(err) => {
                error!("Failed to handle event: {err:?}");
                NewWsResponse::error()
            }
        }
    }

    /// 从头部列表中获取指定键的值
    fn get_header_value(headers: &[Header], key: &str) -> Option<String> {
        headers
            .iter()
            .find(|h| h.key == key)
            .map(|h| h.value.clone())
    }

    /// 构建 ping 帧
    pub fn build_ping_frame(service_id: i32) -> Frame {
        Frame {
            seq_id: 0,
            log_id: 0,
            service: service_id,
            method: 0, // Control frame
            headers: vec![Header {
                key: "type".to_string(),
                value: "ping".to_string(),
            }],
            payload_encoding: None,
            payload_type: None,
            payload: None,
            log_id_new: None,
        }
    }

    /// 构建数据帧响应
    pub fn build_response_frame(service_id: i32, headers: Vec<Header>, payload: Vec<u8>) -> Frame {
        Frame {
            seq_id: 0,
            log_id: 0,
            service: service_id,
            method: 1, // Data frame
            headers,
            payload_encoding: None,
            payload_type: None,
            payload: Some(payload),
            log_id_new: None,
        }
    }
}

/// WebSocket 响应结构
#[derive(Serialize, Deserialize, Debug)]
struct NewWsResponse {
    code: u16,
    headers: std::collections::HashMap<String, String>,
    data: Vec<u8>,
}

impl NewWsResponse {
    fn ok() -> Self {
        Self {
            code: 200,
            headers: Default::default(),
            data: Default::default(),
        }
    }

    fn error() -> Self {
        Self {
            code: 500,
            headers: Default::default(),
            data: Default::default(),
        }
    }
}
