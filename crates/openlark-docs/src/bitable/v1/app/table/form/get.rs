/// Bitable 获取表单API
///
/// API文档: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app/table/form/get
use openlark_core::{
    api::ApiRequest,
    config::Config,
    error::{validation_error, SDKResult},
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 获取表单请求
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GetFormRequest {
    config: Config,
    api_request: ApiRequest<GetFormResponse>,
    app_token: String,
    table_id: String,
    form_id: String,
}

impl GetFormRequest {
    /// 创建获取表单请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            api_request: ApiRequest::get(""),
            app_token: String::new(),
            table_id: String::new(),
            form_id: String::new(),
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

    /// 设置表单ID
    pub fn form_id(mut self, form_id: String) -> Self {
        self.form_id = form_id;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetFormResponse> {
        // 参数验证
        if self.app_token.trim().is_empty() {
            return Err(validation_error("app_token", "应用token不能为空"));
        }
        if self.table_id.trim().is_empty() {
            return Err(validation_error("table_id", "数据表ID不能为空"));
        }
        if self.form_id.trim().is_empty() {
            return Err(validation_error("form_id", "表单ID不能为空"));
        }

        // 构建完整的API URL
        let api_url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables/{}/forms/{}",
            self.config.base_url, self.app_token, self.table_id, self.form_id
        );

        // 设置API URL
        let mut api_request = self.api_request;
        api_request.url = api_url;

        // 发送请求 - 转换为ApiRequest<()>以匹配Transport::request签名
        let request_for_transport: openlark_core::api::ApiRequest<()> =
            openlark_core::api::ApiRequest::get(api_request.url.clone())
                .body(openlark_core::api::RequestData::Empty);

        // 发送请求并解析响应
        let response = Transport::request(request_for_transport, &self.config, None).await?;

        // 手动解析响应数据
        let response_data: GetFormResponse =
            serde_json::from_value(response.data.ok_or_else(|| {
                openlark_core::error::validation_error("response", "响应数据为空")
            })?)?;
        Ok(response_data)
    }
}

/// 获取表单Builder
pub struct GetFormRequestBuilder {
    request: GetFormRequest,
}

impl GetFormRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: GetFormRequest::new(config),
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

    /// 设置表单ID
    pub fn form_id(mut self, form_id: String) -> Self {
        self.request = self.request.form_id(form_id);
        self
    }

    /// 构建请求
    pub fn build(self) -> GetFormRequest {
        self.request
    }
}

/// 表单结构
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Form {
    /// 表单ID
    pub form_id: String,
    /// 表单名称
    pub name: String,
    /// 表单项列表
    pub items: Option<Vec<FormItem>>,
}

/// 表单项
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FormItem {
    /// 字段名称
    pub field_name: String,
    /// 字段类型
    pub field_type: String,
    /// 是否必填
    pub required: bool,
}

/// 获取表单响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetFormResponse {
    /// 表单信息
    pub form: Option<Form>,
    /// 操作结果
    pub success: bool,
}
