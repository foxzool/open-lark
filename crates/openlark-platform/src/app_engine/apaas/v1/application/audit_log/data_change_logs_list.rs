//! 查询数据变更日志列表
//!
//! 文档: https://open.feishu.cn/document/apaas-v1/application-audit_log/data_change_logs_list

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 查询数据变更日志列表 Builder
#[derive(Debug, Clone)]
pub struct DataChangeLogsListBuilder {
    config: Config,
    /// 应用命名空间
    namespace: String,
    /// 开始时间
    start_time: Option<i64>,
    /// 结束时间
    end_time: Option<i64>,
    /// 页码
    page: Option<u32>,
    /// 每页数量
    page_size: Option<u32>,
}

impl DataChangeLogsListBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, namespace: impl Into<String>) -> Self {
        Self {
            config,
            namespace: namespace.into(),
            start_time: None,
            end_time: None,
            page: None,
            page_size: None,
        }
    }

    /// 设置开始时间
    pub fn start_time(mut self, start_time: i64) -> Self {
        self.start_time = Some(start_time);
        self
    }

    /// 设置结束时间
    pub fn end_time(mut self, end_time: i64) -> Self {
        self.end_time = Some(end_time);
        self
    }

    /// 设置页码
    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    /// 设置每页数量
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<DataChangeLogsListResponse> {
        let url = format!(
            "/open-apis/apaas/v1/applications/{}/audit_log/data_change_logs_list",
            self.namespace
        );

        let transport = Transport::new(self.config);
        transport
            .get_with_query(
                url,
                vec![
                    ("start_time", self.start_time.map(|t| t.to_string())),
                    ("end_time", self.end_time.map(|t| t.to_string())),
                    ("page", self.page.map(|p| p.to_string())),
                    ("page_size", self.page_size.map(|p| p.to_string())),
                ],
            )
            .await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DataChangeLogsListResponse> {
        let url = format!(
            "/open-apis/apaas/v1/applications/{}/audit_log/data_change_logs_list",
            self.namespace
        );

        let transport = Transport::new(self.config);
        transport
            .get_with_query_and_option(
                url,
                vec![
                    ("start_time", self.start_time.map(|t| t.to_string())),
                    ("end_time", self.end_time.map(|t| t.to_string())),
                    ("page", self.page.map(|p| p.to_string())),
                    ("page_size", self.page_size.map(|p| p.to_string())),
                ],
                option,
            )
            .await
    }
}

/// 数据变更日志信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DataChangeLogInfo {
    /// 日志 ID
    #[serde(rename = "log_id")]
    log_id: String,
    /// 变更类型
    #[serde(rename = "change_type")]
    change_type: String,
    /// 变更对象
    #[serde(rename = "object_type")]
    object_type: String,
    /// 变更对象 ID
    #[serde(rename = "object_id")]
    object_id: String,
    /// 操作人
    #[serde(rename = "operator")]
    operator: String,
    /// 变更时间
    #[serde(rename = "change_time")]
    change_time: i64,
}

/// 查询数据变更日志列表响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DataChangeLogsListResponse {
    /// 数据变更日志列表
    #[serde(rename = "items")]
    items: Vec<DataChangeLogInfo>,
    /// 是否有更多
    #[serde(rename = "has_more")]
    has_more: bool,
    /// 页码
    #[serde(rename = "page")]
    page: u32,
    /// 每页数量
    #[serde(rename = "page_size")]
    page_size: u32,
}

impl ApiResponseTrait for DataChangeLogsListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
