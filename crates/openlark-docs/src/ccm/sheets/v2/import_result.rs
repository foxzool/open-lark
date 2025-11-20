//! Sheets v2 导入结果查询服务
//!
//! 提供飞书电子表格v2版本的导入结果查询功能，支持：
//! - 异步任务状态查询
//! - 导入结果获取
//! - 错误信息处理
//! - 进度跟踪

use openlark_core::{
    api::ApiRequest,
    api::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    error::LarkAPIError,
    http::Transport,
    req_option::RequestOption,
    standard_response::StandardResponse,
    SDKResult,
};
use reqwest::Method;
use serde::{Deserialize, Serialize};

/// 导入结果查询请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImportResultRequest {
    /// 任务票据
    pub ticket: String,
}

/// 导入结果查询响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportResultResponse {
    /// 导入结果数据
    pub data: ImportResultData,
}

/// 导入结果数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportResultData {
    /// 任务状态
    pub status: ImportStatus,
    /// 电子表格token（成功时返回）
    pub spreadsheet_token: Option<String>,
    /// 错误信息（失败时返回）
    pub error: Option<String>,
    /// 进度信息
    pub progress: Option<ImportProgress>,
}

/// 导入状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ImportStatus {
    /// 进行中
    #[serde(rename = "processing")]
    Processing,
    /// 成功
    #[serde(rename = "success")]
    Success,
    /// 失败
    #[serde(rename = "failed")]
    Failed,
    /// 已取消
    #[serde(rename = "cancelled")]
    Cancelled,
}

/// 导入进度信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImportProgress {
    /// 总行数
    pub total_rows: Option<u32>,
    /// 已处理行数
    pub processed_rows: Option<u32>,
    /// 进度百分比
    pub percentage: Option<f32>,
}

impl Default for ImportResultResponse {
    fn default() -> Self {
        Self {
            data: ImportResultData {
                status: ImportStatus::Processing,
                spreadsheet_token: None,
                error: None,
                progress: None,
            },
        }
    }
}

impl ApiResponseTrait for ImportResultResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 导入结果查询服务
#[derive(Clone, Debug)]
pub struct ImportResultService {
    config: Config,
}

impl ImportResultService {
    /// 创建新的导入结果查询服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 查询导入结果
    ///
    /// # 参数
    /// - `ticket`: 任务票据
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::sheets::v2::import_result::{ImportResultRequest, ImportResultService};
    ///
    /// let config = openlark_core::config::Config::new("app_id", "app_secret");
    /// let service = ImportResultService::new(config);
    ///
    /// let request = ImportResultRequest {
    ///     ticket: "ticket_12345".to_string(),
    /// };
    ///
    /// let result = service.get_result(&request);
    /// ```
    pub async fn get_result(
        &self,
        request: &ImportResultRequest,
    ) -> SDKResult<ImportResultResponse> {
        use openlark_core::{api::ApiRequest, http::Transport};
        use reqwest::Method;

        let endpoint = format!(
            "/open-apis/sheets/v2/import/result?ticket={}",
            request.ticket
        );

        let api_request = ApiRequest::with_method_and_path(Method::GET, &endpoint);
        let result_response: StandardResponse<ImportResultResponse> =
            Transport::request(api_request, &self.config, None).await?;

        if let Some(data) = result_response.data {
            Ok(data)
        } else {
            Err(LarkAPIError::InvalidResponse(
                "Missing data field".to_string(),
            ))
        }
    }

    /// 轮询导入结果直到完成
    ///
    /// # 参数
    /// - `ticket`: 任务票据
    /// - `max_attempts`: 最大轮询次数（默认60次）
    /// - `interval_seconds`: 轮询间隔秒数（默认2秒）
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::sheets::v2::import_result::ImportResultService;
    ///
    /// let config = openlark_core::config::Config::new("app_id", "app_secret");
    /// let service = ImportResultService::new(config);
    ///
    /// let result = service.poll_until_complete("ticket_12345", Some(30), Some(1));
    /// ```
    pub async fn poll_until_complete(
        &self,
        ticket: &str,
        max_attempts: Option<u32>,
        interval_seconds: Option<u64>,
    ) -> SDKResult<ImportResultResponse> {
        let max_attempts = max_attempts.unwrap_or(60);
        let interval_seconds = interval_seconds.unwrap_or(2);

        for attempt in 1..=max_attempts {
            let request = ImportResultRequest {
                ticket: ticket.to_string(),
            };

            let result = self.get_result(&request).await?;

            match result.data.status {
                ImportStatus::Success | ImportStatus::Failed | ImportStatus::Cancelled => {
                    return Ok(result);
                }
                ImportStatus::Processing => {
                    if attempt < max_attempts {
                        tokio::time::sleep(tokio::time::Duration::from_secs(interval_seconds))
                            .await;
                    }
                }
            }
        }

        Err(LarkAPIError::Timeout(format!(
            "Import task polling timeout after {} attempts",
            max_attempts
        )))
    }
}

impl openlark_core::core::trait_system::Service for ImportResultService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "ImportResultService"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::trait_system::Service;

    #[test]
    fn test_import_result_service_creation() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = ImportResultService::new(config);
        assert_eq!(service.service_name(), "ImportResultService");
    }

    #[test]
    fn test_import_result_request() {
        let request = ImportResultRequest {
            ticket: "test_ticket_123".to_string(),
        };
        assert_eq!(request.ticket, "test_ticket_123");
    }

    #[test]
    fn test_import_status_variants() {
        assert_eq!(ImportStatus::Processing, ImportStatus::Processing);
        assert_eq!(ImportStatus::Success, ImportStatus::Success);
        assert_eq!(ImportStatus::Failed, ImportStatus::Failed);
        assert_eq!(ImportStatus::Cancelled, ImportStatus::Cancelled);
    }

    #[test]
    fn test_import_progress() {
        let progress = ImportProgress {
            total_rows: Some(1000),
            processed_rows: Some(500),
            percentage: Some(50.0),
        };

        assert_eq!(progress.total_rows, Some(1000));
        assert_eq!(progress.processed_rows, Some(500));
        assert_eq!(progress.percentage, Some(50.0));
    }

    #[test]
    fn test_import_result_data() {
        let data = ImportResultData {
            status: ImportStatus::Processing,
            spreadsheet_token: None,
            error: None,
            progress: Some(ImportProgress {
                total_rows: Some(100),
                processed_rows: Some(50),
                percentage: Some(50.0),
            }),
        };

        assert_eq!(data.status, ImportStatus::Processing);
        assert!(data.spreadsheet_token.is_none());
        assert!(data.error.is_none());
        assert!(data.progress.is_some());

        let progress = data.progress.unwrap();
        assert_eq!(progress.percentage, Some(50.0));
    }

    #[test]
    fn test_import_result_response_default() {
        let response = ImportResultResponse::default();
        assert_eq!(response.data.status, ImportStatus::Processing);
        assert!(response.data.spreadsheet_token.is_none());
        assert!(response.data.error.is_none());
        assert!(response.data.progress.is_none());
    }

    #[test]
    fn test_import_status_serialization() {
        let status = ImportStatus::Success;
        let json_str = serde_json::to_string(&status);
        assert!(json_str.is_ok());

        let parsed: Result<ImportStatus, _> = serde_json::from_str(&json_str.unwrap());
        assert!(parsed.is_ok());
        assert_eq!(parsed.unwrap(), ImportStatus::Success);
    }

    #[test]
    fn test_api_response_trait() {
        assert_eq!(ImportResultResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_import_result_data_serialization() {
        let data = ImportResultData {
            status: ImportStatus::Success,
            spreadsheet_token: Some("spreadsheet_token_123".to_string()),
            error: None,
            progress: Some(ImportProgress {
                total_rows: Some(1000),
                processed_rows: Some(1000),
                percentage: Some(100.0),
            }),
        };

        let json_str = serde_json::to_string(&data);
        assert!(json_str.is_ok());

        let parsed: Result<ImportResultData, _> = serde_json::from_str(&json_str.unwrap());
        assert!(parsed.is_ok());

        let parsed_data = parsed.unwrap();
        assert_eq!(parsed_data.status, ImportStatus::Success);
        assert_eq!(
            parsed_data.spreadsheet_token,
            Some("spreadsheet_token_123".to_string())
        );
    }
}
