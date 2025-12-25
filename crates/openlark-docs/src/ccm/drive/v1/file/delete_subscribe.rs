/// 取消云文档事件订阅
///
/// 该接口**仅支持文档拥有者**取消订阅自己文档的通知事件，可订阅的文档类型为**旧版文档**、**新版文档**、**电子表格**和**多维表格**。
/// docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/delete_subscribe
/// doc: https://open.feishu.cn/document/server-docs/docs/drive-v1/event/delete_subscribe
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 取消云文档事件订阅请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSubscribeRequest {
    #[serde(skip)]
    config: Config,
    /// 文件token
    pub file_token: String,
}

impl DeleteSubscribeRequest {
    /// 创建取消云文档事件订阅请求
    ///
    /// # 参数
    /// * `config` - 配置
    /// * `file_token` - 文件token
    pub fn new(config: Config, file_token: impl Into<String>) -> Self {
        Self {
            config,
            file_token: file_token.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<DeleteSubscribeResponse> {
        if self.file_token.is_empty() {
            return Err(openlark_core::error::validation_error(
                "file_token",
                "file_token 不能为空",
            ));
        }

        let api_endpoint = DriveApi::DeleteFileSubscribe(self.file_token.clone());
        let request = ApiRequest::<DeleteSubscribeResponse>::delete(&api_endpoint.to_url());

        let response = Transport::request(request, &self.config, None).await?;
        extract_response_data(response, "取消云文档事件订阅")
    }
}

/// 取消云文档事件订阅响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSubscribeResponse {
    /// 操作结果
    pub result: Option<String>,
    /// 文件token
    pub file_token: Option<String>,
}

impl ApiResponseTrait for DeleteSubscribeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_subscribe_request_builder() {
        let config = Config::default();
        let request = DeleteSubscribeRequest::new(config, "file_token");

        assert_eq!(request.file_token, "file_token");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(DeleteSubscribeResponse::data_format(), ResponseFormat::Data);
    }
}
