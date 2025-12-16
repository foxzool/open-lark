/// 取消云文档事件订阅
///
/// 该接口**仅支持文档拥有者**取消订阅自己文档的通知事件，可订阅的文档类型为**旧版文档**、**新版文档**、**电子表格**和**多维表格**。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/event/delete_subscribe
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

// use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 取消云文档事件订阅请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSubscribeRequest {
    /// 文件token
    pub file_token: String,
}

impl DeleteSubscribeRequest {
    /// 创建取消云文档事件订阅请求
    ///
    /// # 参数
    /// * `file_token` - 文件token
    pub fn new(file_token: impl Into<String>) -> Self {
        Self {
            file_token: file_token.into(),
        }
    }
}

/// 取消云文档事件订阅响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSubscribeResponse {
    /// 操作结果
    pub result: String,
    /// 文件token
    pub file_token: String,
}

impl ApiResponseTrait for DeleteSubscribeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 取消云文档事件订阅
///
/// 该接口**仅支持文档拥有者**取消订阅自己文档的通知事件，可订阅的文档类型为**旧版文档**、**新版文档**、**电子表格**和**多维表格**。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/event/delete_subscribe
pub async fn delete_subscribe(
    request: DeleteSubscribeRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<DeleteSubscribeResponse>> {
    // 创建API请求
    let mut api_request: ApiRequest<DeleteSubscribeResponse> =
        ApiRequest::delete(&format!("/open-apis/drive/v1/files/{}/delete_subscribe", request.file_token));

    // 如果有请求选项，应用它们
    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    // 发送请求
    Transport::request(api_request, config, None).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_subscribe_request_builder() {
        let request = DeleteSubscribeRequest::new("file_token");

        assert_eq!(request.file_token, "file_token");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(DeleteSubscribeResponse::data_format(), ResponseFormat::Data);
    }
}