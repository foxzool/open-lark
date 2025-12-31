/// Bitable 获取表单API
///
/// docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-form/get
/// doc: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-form/get
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::{validation_error, SDKResult},
    http::Transport,
};
use serde::{Deserialize, Serialize};

use super::models::Form;

/// 获取表单请求
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GetFormRequest {
    config: Config,
    app_token: String,
    table_id: String,
    form_id: String,
}

impl GetFormRequest {
    /// 创建获取表单请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
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

        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint = BitableApiV1::FormGet(self.app_token, self.table_id, self.form_id);

        let api_request: ApiRequest<GetFormResponse> = ApiRequest::get(&api_endpoint.to_url());
        let response = Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
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

/// 获取表单响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetFormResponse {
    /// 表单元数据
    pub form: Form,
}

impl ApiResponseTrait for GetFormResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
