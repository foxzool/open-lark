//! Bitable V1 列出字段API

use openlark_core::{
    api::ApiRequest,
    config::Config,
    error::{validation_error, SDKResult},
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 重用Field类型
pub use super::create::Field;

/// 列出字段请求
#[derive(Debug, Clone)]
pub struct ListFieldRequest {
    /// 配置信息
    config: Config,
    api_request: ApiRequest<ListFieldResponse>,
    /// 多维表格的 app_token
    app_token: String,
    /// 数据表的 table_id
    table_id: String,
    /// 视图 ID
    view_id: Option<String>,
    /// 控制字段描述数据的返回格式
    text_field_as_array: Option<bool>,
    /// 分页标记
    page_token: Option<String>,
    /// 分页大小
    page_size: Option<i32>,
    /// 用户 ID 类型
    user_id_type: Option<String>,
}

impl ListFieldRequest {
    /// 创建列出字段请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            api_request: ApiRequest::get(""),
            app_token: String::new(),
            table_id: String::new(),
            view_id: None,
            text_field_as_array: None,
            page_token: None,
            page_size: None,
            user_id_type: None,
        }
    }

    /// 设置应用token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    /// 设置数据表ID
    pub fn table_id(mut self, table_id: String) -> Self {
        self.table_id = table_id;
        self
    }

    /// 设置视图ID
    pub fn view_id(mut self, view_id: String) -> Self {
        self.view_id = Some(view_id);
        self
    }

    /// 设置文本字段为数组格式
    pub fn text_field_as_array(mut self, text_field_as_array: bool) -> Self {
        self.text_field_as_array = Some(text_field_as_array);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size.min(100)); // 限制最大100
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListFieldResponse> {
        // 参数验证
        if self.app_token.trim().is_empty() {
            return Err(validation_error("app_token", "应用token不能为空"));
        }
        if self.table_id.trim().is_empty() {
            return Err(validation_error("table_id", "数据表ID不能为空"));
        }

        // 构建完整的API URL
        let api_url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables/{}/fields",
            self.config.base_url, self.app_token, self.table_id
        );

        // 设置API URL
        let mut api_request = self.api_request;
        api_request.url = api_url;

        // 设置查询参数
        let mut separator_added = false;
        if let Some(view_id) = &self.view_id {
            api_request.url = format!("{}?view_id={}", api_request.url, view_id);
            separator_added = true;
        }
        if let Some(text_field_as_array) = self.text_field_as_array {
            let separator = if separator_added { "&" } else { "?" };
            api_request.url = format!(
                "{}{}text_field_as_array={}",
                api_request.url, separator, text_field_as_array
            );
            separator_added = true;
        }
        if let Some(page_token) = &self.page_token {
            let separator = if separator_added { "&" } else { "?" };
            api_request.url = format!("{}{}page_token={}", api_request.url, separator, page_token);
            separator_added = true;
        }
        if let Some(page_size) = self.page_size {
            let separator = if separator_added { "&" } else { "?" };
            api_request.url = format!("{}{}page_size={}", api_request.url, separator, page_size);
            separator_added = true;
        }
        if let Some(user_id_type) = &self.user_id_type {
            let separator = if separator_added { "&" } else { "?" };
            api_request.url = format!(
                "{}{}user_id_type={}",
                api_request.url, separator, user_id_type
            );
        }

        // 发送请求 - 转换为ApiRequest<()>以匹配Transport::request签名
        let request_for_transport: openlark_core::api::ApiRequest<()> =
            openlark_core::api::ApiRequest::get(api_request.url.clone())
                .body(openlark_core::api::RequestData::Empty);

        // 发送请求并解析响应
        let response = Transport::request(request_for_transport, &self.config, None).await?;

        // 手动解析响应数据
        let response_data: ListFieldResponse =
            serde_json::from_value(response.data.ok_or_else(|| {
                openlark_core::error::validation_error("response", "响应数据为空")
            })?)?;
        Ok(response_data)
    }
}

/// 列出字段Builder
pub struct ListFieldRequestBuilder {
    request: ListFieldRequest,
}

impl ListFieldRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: ListFieldRequest::new(config),
        }
    }

    /// 设置应用token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.request = self.request.app_token(app_token);
        self
    }

    /// 设置数据表ID
    pub fn table_id(mut self, table_id: String) -> Self {
        self.request = self.request.table_id(table_id);
        self
    }

    /// 设置视图ID
    pub fn view_id(mut self, view_id: String) -> Self {
        self.request = self.request.view_id(view_id);
        self
    }

    /// 设置文本字段为数组格式
    pub fn text_field_as_array(mut self, text_field_as_array: bool) -> Self {
        self.request = self.request.text_field_as_array(text_field_as_array);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: String) -> Self {
        self.request = self.request.page_token(page_token);
        self
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request = self.request.page_size(page_size);
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.request = self.request.user_id_type(user_id_type);
        self
    }

    /// 构建请求
    pub fn build(self) -> ListFieldRequest {
        self.request
    }
}

/// 列出字段响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListFieldResponse {
    /// 是否还有更多项
    pub has_more: Option<bool>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 总数
    pub total: Option<i32>,
    /// 字段信息列表
    pub items: Option<Vec<Field>>,
    /// 操作结果
    pub success: bool,
}
