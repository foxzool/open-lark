use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 订阅文件更新
///
/// 订阅文件的更新通知
/// docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/subscribe
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 订阅文件请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeFileRequest {
    #[serde(skip)]
    config: Config,
    /// 文件token
    pub file_token: String,
    /// 文件类型
    pub file_type: String,
    /// 订阅类型
    pub event_type: Option<String>,
}

impl SubscribeFileRequest {
    /// 创建订阅文件请求
    ///
    /// # 参数
    /// * `config` - 配置
    /// * `file_token` - 文件token
    /// * `file_type` - 文件类型（doc/docx/sheet/bitable/file/folder/slides）
    pub fn new(config: Config, file_token: impl Into<String>, file_type: impl Into<String>) -> Self {
        Self {
            config,
            file_token: file_token.into(),
            file_type: file_type.into(),
            event_type: None,
        }
    }

    /// 设置订阅类型
    pub fn event_type(mut self, event_type: impl Into<String>) -> Self {
        self.event_type = Some(event_type.into());
        self
    }

    pub async fn execute(self) -> SDKResult<SubscribeFileResponse> {
        if self.file_token.is_empty() {
            return Err(openlark_core::error::validation_error(
                "file_token",
                "file_token 不能为空",
            ));
        }
        if self.file_type.is_empty() {
            return Err(openlark_core::error::validation_error(
                "file_type",
                "file_type 不能为空",
            ));
        }
        match self.file_type.as_str() {
            "doc" | "docx" | "sheet" | "bitable" | "file" | "folder" | "slides" => {}
            _ => {
                return Err(openlark_core::error::validation_error(
                    "file_type",
                    "file_type 仅支持 doc/docx/sheet/bitable/file/folder/slides",
                ));
            }
        }

        // 文档约束：仅当 file_type 为 folder 时，允许传 event_type，且必须为 file.created_in_folder_v1
        if self.file_type == "folder" {
            match self.event_type.as_deref() {
                Some("file.created_in_folder_v1") => {}
                Some(_) => {
                    return Err(openlark_core::error::validation_error(
                        "event_type",
                        "当 file_type=folder 时，event_type 必须为 file.created_in_folder_v1",
                    ));
                }
                None => {
                    return Err(openlark_core::error::validation_error(
                        "event_type",
                        "当 file_type=folder 时，event_type 不能为空（必须为 file.created_in_folder_v1）",
                    ));
                }
            }
        } else if self.event_type.is_some() {
            return Err(openlark_core::error::validation_error(
                "event_type",
                "当 file_type 不为 folder 时，请勿传入 event_type",
            ));
        }

        let api_endpoint = DriveApi::SubscribeFile(self.file_token.clone());
        let mut request = ApiRequest::<SubscribeFileResponse>::post(&api_endpoint.to_url());

        request = request.query("file_type", &self.file_type);
        if let Some(et) = &self.event_type {
            request = request.query("event_type", et);
        }

        let response = Transport::request(request, &self.config, None).await?;
        extract_response_data(response, "订阅云文档事件")
    }
}

/// 订阅文件响应
///
/// 文档响应体 `data` 为空对象 `{}`。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubscribeFileResponse {}

impl ApiResponseTrait for SubscribeFileResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subscribe_file_request_builder() {
        let config = Config::default();
        let request = SubscribeFileRequest::new(config, "file_token", "docx");

        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.file_type, "docx");
    }

    #[test]
    fn test_subscribe_data_structure() {
        let _subscribe_data = SubscribeFileResponse {};
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(SubscribeFileResponse::data_format(), ResponseFormat::Data);
    }
}
