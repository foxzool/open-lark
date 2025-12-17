/// Bitable 更新表单元数据
///
/// docPath: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-form/patch
/// doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/form/patch-2
///
/// 说明：
/// - 该接口的请求体字段会随表单能力迭代而变化，因此这里以 `serde_json::Value` 透传。
/// - 你可以根据官方文档构造请求体 JSON。
use openlark_core::{
    api::ApiRequest,
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use crate::common::api_endpoints::BitableApiV1;

/// 更新表单元数据请求
#[derive(Debug, Clone)]
pub struct PatchFormRequest {
    config: Config,
    app_token: String,
    table_id: String,
    form_id: String,
    user_id_type: Option<String>,
    payload: serde_json::Value,
}

impl PatchFormRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            table_id: String::new(),
            form_id: String::new(),
            user_id_type: None,
            payload: serde_json::json!({}),
        }
    }

    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.app_token = app_token.into();
        self
    }

    pub fn table_id(mut self, table_id: impl Into<String>) -> Self {
        self.table_id = table_id.into();
        self
    }

    pub fn form_id(mut self, form_id: impl Into<String>) -> Self {
        self.form_id = form_id.into();
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置请求体（透传 JSON）
    pub fn payload(mut self, payload: serde_json::Value) -> Self {
        self.payload = payload;
        self
    }

    pub async fn execute(self) -> SDKResult<PatchFormResponse> {
        validate_required!(self.app_token, "app_token 不能为空");
        validate_required!(self.table_id, "table_id 不能为空");
        validate_required!(self.form_id, "form_id 不能为空");

        let api_endpoint = BitableApiV1::FormPatch(self.app_token, self.table_id, self.form_id);
        let mut api_request: ApiRequest<PatchFormResponse> =
            ApiRequest::patch(&api_endpoint.to_url()).body(serde_json::to_vec(&self.payload)?);

        api_request = api_request.query_opt("user_id_type", self.user_id_type);

        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("response", "响应数据为空")
        })
    }
}

/// 更新表单元数据 Builder
pub struct PatchFormRequestBuilder {
    request: PatchFormRequest,
}

impl PatchFormRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            request: PatchFormRequest::new(config),
        }
    }

    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.request = self.request.app_token(app_token);
        self
    }

    pub fn table_id(mut self, table_id: impl Into<String>) -> Self {
        self.request = self.request.table_id(table_id);
        self
    }

    pub fn form_id(mut self, form_id: impl Into<String>) -> Self {
        self.request = self.request.form_id(form_id);
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request = self.request.user_id_type(user_id_type);
        self
    }

    pub fn payload(mut self, payload: serde_json::Value) -> Self {
        self.request = self.request.payload(payload);
        self
    }

    pub fn build(self) -> PatchFormRequest {
        self.request
    }
}

/// 更新表单元数据响应（透传 data JSON）
pub type PatchFormResponse = serde_json::Value;
