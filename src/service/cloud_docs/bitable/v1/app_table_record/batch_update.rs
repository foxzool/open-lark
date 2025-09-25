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

/// 批量更新记录请求
#[derive(Debug, Serialize, Default, Clone)]
pub struct BatchUpdateRecordRequest {
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
    /// 要更新的记录列表
    records: Vec<UpdateRecord>,
}

/// 要更新的记录数据
#[derive(Debug, Serialize, Default, Clone)]
pub struct UpdateRecord {
    /// 记录 ID
    pub record_id: String,
    /// 要更新的字段
    pub fields: Value,
}

impl BatchUpdateRecordRequest {
    pub fn builder() -> BatchUpdateRecordRequestBuilder {
        BatchUpdateRecordRequestBuilder::default()
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
pub struct BatchUpdateRecordRequestBuilder {
    request: BatchUpdateRecordRequest,
}

impl BatchUpdateRecordRequestBuilder {
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

    /// 要更新的记录列表
    pub fn records(mut self, records: Vec<UpdateRecord>) -> Self {
        self.request.records = records;
        self
    }

    /// 添加单条更新记录
    pub fn add_record(mut self, record_id: impl ToString, fields: Value) -> Self {
        self.request.records.push(UpdateRecord {
            record_id: record_id.to_string(),
            fields,
        });
        self
    }

    pub fn build(mut self) -> BatchUpdateRecordRequest {
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

// 应用ExecutableBuilder trait到BatchUpdateRecordRequestBuilder
crate::impl_executable_builder_owned!(
    BatchUpdateRecordRequestBuilder,
    super::AppTableRecordService,
    BatchUpdateRecordRequest,
    BaseResponse<BatchUpdateRecordResponse>,
    batch_update
);

/// 批量更新记录响应
#[derive(Debug, Deserialize)]
pub struct BatchUpdateRecordResponse {
    /// 更新后的记录列表
    pub records: Vec<Record>,
}

impl ApiResponseTrait for BatchUpdateRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量更新记录
pub async fn batch_update_record(
    request: BatchUpdateRecordRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<BatchUpdateRecordResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::POST;
    api_req.api_path = BITABLE_V1_RECORDS_BATCH_UPDATE
        .replace("{app_token}", &request.app_token)
        .replace("{table_id}", &request.table_id);
    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

impl UpdateRecord {
    /// 创建新的更新记录
    pub fn new(record_id: impl ToString, fields: Value) -> Self {
        Self {
            record_id: record_id.to_string(),
            fields,
        }
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_batch_update_record_request_builder() {
        let update_records = vec![
            UpdateRecord::new(
                "rec123",
                json!({
                    "状态": "已完成",
                    "进度": 100
                }),
            ),
            UpdateRecord::new(
                "rec456",
                json!({
                    "状态": "进行中",
                    "进度": 50
                }),
            ),
        ];

        let request = BatchUpdateRecordRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .table_id("tblsRc9GRRXKqhvW")
            .user_id_type("open_id")
            .records(update_records)
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
        assert_eq!(request.records.len(), 2);
    }
}
