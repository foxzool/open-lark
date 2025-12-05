//! Bitable V1 列出字段API

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
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

        // 验证分页大小
        if let Some(page_size) = self.page_size {
            if page_size <= 0 {
                return Err(validation_error("page_size", "分页大小必须大于0"));
            }
        }

        // 构建API路径
        let path = format!("/open-apis/bitable/v1/apps/{}/tables/{}/fields", self.app_token, self.table_id);

        // 创建API请求
        let mut api_request: ApiRequest<ListFieldResponse> =
            ApiRequest::get(&format!("https://open.feishu.cn{}", path));

        // 构建查询参数
        if let Some(ref view_id) = self.view_id {
            api_request = api_request.query("view_id", view_id);
        }

        if let Some(text_field_as_array) = self.text_field_as_array {
            api_request = api_request.query("text_field_as_array", &text_field_as_array.to_string());
        }

        if let Some(ref page_token) = self.page_token {
            api_request = api_request.query("page_token", page_token);
        }

        if let Some(page_size) = self.page_size {
            api_request = api_request.query("page_size", &page_size.to_string());
        }

        if let Some(ref user_id_type) = self.user_id_type {
            api_request = api_request.query("user_id_type", user_id_type);
        }

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
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

/// 列出字段数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListFieldData {
    /// 是否还有更多项
    pub has_more: Option<bool>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 总数
    pub total: Option<i32>,
    /// 字段信息列表
    pub items: Option<Vec<Field>>,
}

/// 列出字段响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListFieldResponse {
    /// 字段列表数据
    pub data: ListFieldData,
}

impl ApiResponseTrait for ListFieldResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
