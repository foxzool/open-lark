//! 查询日历列表
//!
//! docPath: https://open.feishu.cn/document/server-docs/calendar-v4/calendar/list-2

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::validation_error,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CalendarApiV4;

/// 查询日历列表请求
pub struct ListCalendarRequest {
    config: Config,
    query_params: Vec<(String, String)>,
}

/// 查询日历列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListCalendarResponse {
    /// 日历列表
    pub calendar: Vec<CalendarData>,
    /// 分页令牌
    pub page_token: Option<String>,
    /// 是否有更多数据
    pub has_more: Option<bool>,
}

/// 日历数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CalendarData {
    /// 日历 ID
    pub calendar_id: String,
    /// 日历摘要
    pub summary: String,
    /// 日历描述
    pub description: Option<String>,
    /// 日历颜色
    pub color: Option<String>,
    /// 权限
    pub permissions: Option<CalendarPermissions>,
    /// 是否为主日历
    pub primary: Option<bool>,
    /// 日历类型
    pub calendar_type: Option<String>,
}

/// 日历权限
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CalendarPermissions {
    /// 是否可读
    pub is_readable: Option<bool>,
    /// 是否可写
    pub is_writable: Option<bool>,
}

impl ApiResponseTrait for ListCalendarResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ListCalendarRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            query_params: Vec::new(),
        }
    }

    /// 追加查询参数（用于覆盖文档中可选 query）
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/calendar-v4/calendar/list-2
    pub async fn execute(self) -> SDKResult<ListCalendarResponse> {
        // 使用新的枚举+builder系统
        let api_endpoint = CalendarApiV4::CalendarList;

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<ListCalendarResponse> =
            ApiRequest::get(api_endpoint.to_url());

        // 添加查询参数
        for (key, value) in self.query_params {
            api_request = api_request.query(key, value);
        }

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        let data: ListCalendarResponse = response
            .data
            .ok_or_else(|| validation_error("响应数据为空", "服务器没有返回有效的数据"))?;
        Ok(data)
    }
}
