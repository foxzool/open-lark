use std::sync::Arc;
use openlark_core::SDKResult;
use crate::service::WorkflowService;
use serde_json::Value;
use reqwest::Method;

#[derive(Clone)]
pub struct WhiteboardNode {
    service: Arc<WorkflowService>,
}

impl WhiteboardNode {
    pub fn new(service: Arc<WorkflowService>) -> Self { Self { service } }

    /// 文档参考: https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/board-v1/whiteboard-node/create_plantuml
    pub async fn post_open_apis_board_v1_whiteboards_by_whiteboard_id_nodes_plantuml(&self, whiteboard_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/board/v1/whiteboards/:whiteboard_id/nodes/plantuml".to_string();
        path = path.replace(":whiteboard_id", whiteboard_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/board-v1/whiteboard-node/create
    pub async fn post_open_apis_board_v1_whiteboards_by_whiteboard_id_nodes(&self, whiteboard_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/board/v1/whiteboards/:whiteboard_id/nodes".to_string();
        path = path.replace(":whiteboard_id", whiteboard_id.as_ref());
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/docs/board-v1/whiteboard-node/list
    pub async fn get_open_apis_board_v1_whiteboards_by_whiteboard_id_nodes(&self, whiteboard_id: impl AsRef<str>, payload: Option<&Value>) -> SDKResult<Value> {
        let mut path = "/open-apis/board/v1/whiteboards/:whiteboard_id/nodes".to_string();
        path = path.replace(":whiteboard_id", whiteboard_id.as_ref());
        let method = Method::GET;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
