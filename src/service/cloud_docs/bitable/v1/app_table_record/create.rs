use log::error;
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
        standard_response::StandardResponse,
        SDKResult,
    },
    service::bitable::v1::Record,
};

/// 新增记录请求
#[derive(Debug, Serialize, Default, Clone)]
pub struct CreateRecordRequest {
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
    /// 要新增的记录的数据
    #[serde(flatten)]
    fields: Record,
}

impl CreateRecordRequest {
    pub fn builder() -> CreateRecordRequestBuilder {
        CreateRecordRequestBuilder::default()
    }

    pub fn new(app_token: impl ToString, table_id: impl ToString) -> Self {
        Self {
            app_token: app_token.to_string(),
            table_id: table_id.to_string(),
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct CreateRecordRequestBuilder {
    request: CreateRecordRequest,
}

impl CreateRecordRequestBuilder {
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

    /// 记录字段数据
    pub fn fields(mut self, fields: Record) -> Self {
        self.request.fields = fields;
        self
    }

    /// 添加单个字段
    pub fn add_field(mut self, field_name: impl ToString, value: Value) -> Self {
        self.request
            .fields
            .fields
            .insert(field_name.to_string(), value);
        self
    }

    pub fn build(mut self) -> CreateRecordRequest {
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
        match serde_json::to_vec(&self.request) {
            Ok(bytes) => {
                self.request.api_request.body = bytes;
            }
            Err(e) => {
                error!("Failed to serialize create record request: {}", e);
                self.request.api_request.body = Vec::new();
            }
        }
        self.request
    }
}

// 应用ExecutableBuilder trait到CreateRecordRequestBuilder
crate::impl_executable_builder_owned!(
    CreateRecordRequestBuilder,
    super::AppTableRecordService,
    CreateRecordRequest,
    CreateRecordResponse,
    create
);

/// 新增记录响应
#[derive(Debug, Deserialize)]
pub struct CreateRecordResponse {
    /// 新增的记录
    pub record: Record,
}

impl ApiResponseTrait for CreateRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 新增记录
///
/// <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/create>
pub async fn create_record(
    request: CreateRecordRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<CreateRecordResponse> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::POST;
    api_req.api_path = BITABLE_V1_RECORDS
        .replace("{app_token}", &request.app_token)
        .replace("{table_id}", &request.table_id);
    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let api_resp: BaseResponse<CreateRecordResponse> =
        Transport::request(api_req, config, option).await?;
    api_resp.into_result()
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_create_record_request_builder() {
        let record = Record {
            record_id: None,
            fields: std::collections::HashMap::from([
                ("标题".to_string(), json!("测试记录")),
                ("状态".to_string(), json!("进行中")),
            ]),
            created_by: None,
            created_time: None,
            last_modified_by: None,
            last_modified_time: None,
        };

        let request = CreateRecordRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .table_id("tblsRc9GRRXKqhvW")
            .user_id_type("open_id")
            .fields(record)
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
        assert!(request.fields.fields.contains_key("标题"));
    }
}
