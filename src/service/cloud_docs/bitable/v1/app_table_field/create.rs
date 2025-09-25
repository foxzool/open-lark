use reqwest::Method;
use serde::{Deserialize, Serialize};

use super::AppTableFieldService;
use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    impl_executable_builder_owned,
    service::bitable::v1::app_table_field::{
        AppTableField, AppTableFieldDescription, AppTableFieldProperty, FieldType, UiType,
    },
};

/// 新增字段请求
#[derive(Debug, Serialize, Default)]
pub struct CreateFieldRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符
    #[serde(skip)]
    app_token: String,
    /// 多维表格数据表的唯一标识符
    #[serde(skip)]
    table_id: String,
    /// 用户 ID 类型
    #[serde(skip)]
    user_id_type: Option<String>,
    /// 格式为标准的 uuidv4，操作的唯一标识，用于幂等的进行更新操作
    #[serde(skip)]
    client_token: Option<String>,
    /// 多维表格字段名
    field_name: String,
    /// 多维表格字段类型
    r#type: FieldType,
    /// 字段属性
    #[serde(skip_serializing_if = "Option::is_none")]
    property: Option<AppTableFieldProperty>,
    /// 字段的描述
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<AppTableFieldDescription>,
    /// 字段在界面上的展示类型
    #[serde(skip_serializing_if = "Option::is_none")]
    ui_type: Option<UiType>,
}

impl CreateFieldRequest {
    pub fn builder() -> CreateFieldRequestBuilder {
        CreateFieldRequestBuilder::default()
    }

    pub fn new(
        app_token: impl ToString,
        table_id: impl ToString,
        field_name: impl ToString,
        field_type: FieldType,
    ) -> Self {
        Self {
            app_token: app_token.to_string(),
            table_id: table_id.to_string(),
            field_name: field_name.to_string(),
            r#type: field_type,
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct CreateFieldRequestBuilder {
    request: CreateFieldRequest,
}

impl CreateFieldRequestBuilder {
    /// 多维表格的唯一标识符
    pub fn app_token(mut self, app_token: impl ToString) -> Self {
        self.request.app_token = app_token.to_string();
        self
    }

    /// 数据表的唯一标识符
    pub fn table_id(mut self, table_id: impl ToString) -> Self {
        self.request.table_id = table_id.to_string();
        self
    }

    /// 用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: impl ToString) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 客户端请求唯一标识
    pub fn client_token(mut self, client_token: impl ToString) -> Self {
        self.request.client_token = Some(client_token.to_string());
        self
    }

    /// 字段名称
    pub fn field_name(mut self, field_name: impl ToString) -> Self {
        self.request.field_name = field_name.to_string();
        self
    }

    /// 字段类型
    pub fn field_type(mut self, field_type: FieldType) -> Self {
        self.request.r#type = field_type;
        self
    }

    /// 字段属性
    pub fn property(mut self, property: AppTableFieldProperty) -> Self {
        self.request.property = Some(property);
        self
    }

    /// 字段描述
    pub fn description(mut self, description: AppTableFieldDescription) -> Self {
        self.request.description = Some(description);
        self
    }

    /// 字段在界面上的展示类型
    pub fn ui_type(mut self, ui_type: UiType) -> Self {
        self.request.ui_type = Some(ui_type);
        self
    }

    pub fn build(mut self) -> CreateFieldRequest {
        if let Some(user_id_type) = &self.request.user_id_type {
            self.request
                .api_request
                .query_params
                .insert("user_id_type", user_id_type.clone());
        }
        if let Some(client_token) = &self.request.client_token {
            self.request
                .api_request
                .query_params
                .insert("client_token", client_token.clone());
        }
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

impl_executable_builder_owned!(
    CreateFieldRequestBuilder,
    AppTableFieldService,
    CreateFieldRequest,
    BaseResponse<CreateFieldResponse>,
    create
);

/// 新增字段响应
#[derive(Debug, Deserialize)]
pub struct CreateFieldResponse {
    /// 新增的字段信息
    pub field: AppTableField,
}

impl ApiResponseTrait for CreateFieldResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 新增字段
pub async fn create_field(
    request: CreateFieldRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<CreateFieldResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::POST;
    api_req.api_path = BITABLE_V1_FIELDS
        .replace("{app_token}", &request.app_token)
        .replace("{table_id}", &request.table_id);
    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_create_field_request_builder() {
        let property = AppTableFieldProperty::text();

        let request = CreateFieldRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .table_id("tblsRc9GRRXKqhvW")
            .user_id_type("open_id")
            .field_name("测试字段")
            .field_type(FieldType::Text)
            .property(property)
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
        assert_eq!(request.field_name, "测试字段");
        assert_eq!(request.r#type, FieldType::Text);
    }
}
