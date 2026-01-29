//! 创建白板节点（v1）

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateWhiteboardNodeBodyV1 {
    pub title: String,
    #[serde(rename = "type")]
    pub node_type: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub parent_id: String,
    #[serde(default)]
    pub content: serde_json::Value,
    #[serde(default)]
    pub position: NodePosition,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NodePosition {
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
    #[serde(default)]
    pub width: f64,
    #[serde(default)]
    pub height: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateWhiteboardNodeResponseV1 {
    pub node_id: String,
    pub title: String,
    pub node_type: String,
    pub create_time: i64,
}

#[derive(Debug, Clone)]
pub struct CreateWhiteboardNodeRequestV1 {
    config: Arc<Config>,
    board_id: String,
    body: CreateWhiteboardNodeBodyV1,
}

impl CreateWhiteboardNodeRequestV1 {
    pub fn new(config: Arc<Config>, board_id: impl Into<String>) -> Self {
        Self {
            config,
            board_id: board_id.into(),
            body: CreateWhiteboardNodeBodyV1::default(),
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.body.title = title.into();
        self
    }

    pub fn node_type(mut self, node_type: impl Into<String>) -> Self {
        self.body.node_type = node_type.into();
        self
    }

    pub fn parent_id(mut self, parent_id: impl Into<String>) -> Self {
        self.body.parent_id = parent_id.into();
        self
    }

    pub fn content(mut self, content: serde_json::Value) -> Self {
        self.body.content = content;
        self
    }

    pub fn position(mut self, x: f64, y: f64, width: f64, height: f64) -> Self {
        self.body.position = NodePosition {
            x,
            y,
            width,
            height,
        };
        self
    }

    pub async fn execute(self) -> SDKResult<CreateWhiteboardNodeResponseV1> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateWhiteboardNodeResponseV1> {
        validate_required!(self.board_id.trim(), "白板 ID 不能为空");
        validate_required!(self.body.title.trim(), "节点标题不能为空");
        validate_required!(self.body.node_type.trim(), "节点类型不能为空");

        let api_endpoint =
            crate::common::api_endpoints::BoardApiV1::WhiteboardNodeCreate(self.board_id);
        let mut request = ApiRequest::<CreateWhiteboardNodeResponseV1>::post(api_endpoint.to_url());

        let body_json = serde_json::to_value(&self.body).map_err(|e| {
            openlark_core::error::validation_error("序列化请求体失败", e.to_string().as_str())
        })?;

        request = request.body(body_json);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

impl ApiResponseTrait for CreateWhiteboardNodeResponseV1 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whiteboard_node_create_v1_url() {
        let endpoint = crate::common::api_endpoints::BoardApiV1::WhiteboardNodeCreate(
            "test_board_id".to_string(),
        );
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/board/v1/whiteboards/test_board_id/nodes"
        );
    }
}
