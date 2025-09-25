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

/// 更新字段请求
#[derive(Debug, Serialize, Default)]
pub struct UpdateFieldRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符
    #[serde(skip)]
    app_token: String,
    /// 多维表格数据表的唯一标识符
    #[serde(skip)]
    table_id: String,
    /// 字段的唯一标识符
    #[serde(skip)]
    field_id: String,
    /// 用户 ID 类型
    #[serde(skip)]
    user_id_type: Option<String>,
    /// 多维表格字段名
    #[serde(skip_serializing_if = "Option::is_none")]
    field_name: Option<String>,
    /// 多维表格字段类型
    #[serde(skip_serializing_if = "Option::is_none")]
    r#type: Option<FieldType>,
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

impl UpdateFieldRequest {
    pub fn builder() -> UpdateFieldRequestBuilder {
        UpdateFieldRequestBuilder::default()
    }

    pub fn new(app_token: impl ToString, table_id: impl ToString, field_id: impl ToString) -> Self {
        Self {
            app_token: app_token.to_string(),
            table_id: table_id.to_string(),
            field_id: field_id.to_string(),
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct UpdateFieldRequestBuilder {
    request: UpdateFieldRequest,
}

impl UpdateFieldRequestBuilder {
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

    /// 字段的唯一标识符
    pub fn field_id(mut self, field_id: impl ToString) -> Self {
        self.request.field_id = field_id.to_string();
        self
    }

    /// 用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: impl ToString) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 字段名称
    pub fn field_name(mut self, field_name: impl ToString) -> Self {
        self.request.field_name = Some(field_name.to_string());
        self
    }

    /// 字段类型
    pub fn field_type(mut self, field_type: FieldType) -> Self {
        self.request.r#type = Some(field_type);
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

    pub fn build(mut self) -> UpdateFieldRequest {
        if let Some(user_id_type) = &self.request.user_id_type {
            self.request
                .api_request
                .query_params
                .insert("user_id_type", user_id_type.clone());
        }
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

impl_executable_builder_owned!(
    UpdateFieldRequestBuilder,
    AppTableFieldService,
    UpdateFieldRequest,
    BaseResponse<UpdateFieldResponse>,
    update
);

/// 更新字段响应
#[derive(Debug, Deserialize)]
pub struct UpdateFieldResponse {
    /// 更新后的字段信息
    pub field: AppTableField,
}

impl ApiResponseTrait for UpdateFieldResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新字段
pub async fn update_field(
    request: UpdateFieldRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<UpdateFieldResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::PUT;
    api_req.api_path = BITABLE_V1_FIELD_UPDATE
        .replace("{app_token}", &request.app_token)
        .replace("{table_id}", &request.table_id)
        .replace("{field_id}", &request.field_id);
    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_update_field_request_builder() {
        let request = UpdateFieldRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .table_id("tblsRc9GRRXKqhvW")
            .field_id("fldxxxxxx")
            .user_id_type("open_id")
            .field_name("更新后的字段名称")
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.field_id, "fldxxxxxx");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
        assert_eq!(request.field_name, Some("更新后的字段名称".to_string()));
    }
}
