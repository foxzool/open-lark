use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
    service::bitable::v1::Record,
};

/// 更新记录请求
#[derive(Debug, Serialize, Default, Clone)]
pub struct UpdateRecordRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符
    #[serde(skip)]
    app_token: String,
    /// 多维表格数据表的唯一标识符
    #[serde(skip)]
    table_id: String,
    /// 记录的唯一标识符
    #[serde(skip)]
    record_id: String,
    /// 用户 ID 类型
    #[serde(skip)]
    user_id_type: Option<String>,
    /// 要更新的记录的数据
    fields: Value,
}

impl UpdateRecordRequest {
    pub fn builder() -> UpdateRecordRequestBuilder {
        UpdateRecordRequestBuilder::default()
    }

    pub fn new(
        app_token: impl ToString,
        table_id: impl ToString,
        record_id: impl ToString,
    ) -> Self {
        Self {
            app_token: app_token.to_string(),
            table_id: table_id.to_string(),
            record_id: record_id.to_string(),
            fields: Value::Object(Default::default()),
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct UpdateRecordRequestBuilder {
    request: UpdateRecordRequest,
}

impl UpdateRecordRequestBuilder {
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

    /// 记录的唯一标识符
    pub fn record_id(mut self, record_id: impl ToString) -> Self {
        self.request.record_id = record_id.to_string();
        self
    }

    /// 用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: impl ToString) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 记录字段数据
    pub fn fields(mut self, fields: Value) -> Self {
        self.request.fields = fields;
        self
    }

    /// 添加单个字段
    pub fn add_field(mut self, field_name: impl ToString, value: Value) -> Self {
        if let Value::Object(ref mut map) = self.request.fields {
            map.insert(field_name.to_string(), value);
        }
        self
    }

    pub fn build(mut self) -> UpdateRecordRequest {
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

// 应用ExecutableBuilder trait到UpdateRecordRequestBuilder
crate::impl_executable_builder_owned!(
    UpdateRecordRequestBuilder,
    super::AppTableRecordService,
    UpdateRecordRequest,
    BaseResponse<UpdateRecordResponse>,
    update
);

/// 更新记录响应
#[derive(Debug, Deserialize)]
pub struct UpdateRecordResponse {
    /// 更新后的记录
    pub record: Record,
}

impl ApiResponseTrait for UpdateRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新记录
pub async fn update_record(
    request: UpdateRecordRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<UpdateRecordResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::PUT;
    api_req.api_path = BITABLE_V1_RECORD_UPDATE
        .replace("{app_token}", &request.app_token)
        .replace("{table_id}", &request.table_id)
        .replace("{record_id}", &request.record_id);
    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_update_record_request_builder() {
        let request = UpdateRecordRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .table_id("tblsRc9GRRXKqhvW")
            .record_id("rec123456")
            .user_id_type("open_id")
            .fields(json!({
                "标题": "更新后的标题",
                "状态": "已完成"
            }))
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.record_id, "rec123456");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }
}
