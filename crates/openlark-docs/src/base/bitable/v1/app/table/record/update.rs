//! Bitable 更新记录
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/update

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::SDKResult,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::models::Record;

/// 更新记录请求
#[derive(Debug, Clone)]
pub struct UpdateRecordRequest {
    config: Config,
    app_token: String,
    table_id: String,
    record_id: String,
    user_id_type: Option<String>,
    ignore_consistency_check: Option<bool>,
    fields: Value,
}

impl UpdateRecordRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            table_id: String::new(),
            record_id: String::new(),
            user_id_type: None,
            ignore_consistency_check: None,
            fields: Value::Object(Default::default()),
        }
    }

    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    pub fn table_id(mut self, table_id: String) -> Self {
        self.table_id = table_id;
        self
    }

    pub fn record_id(mut self, record_id: String) -> Self {
        self.record_id = record_id;
        self
    }

    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 是否忽略一致性读写检查
    pub fn ignore_consistency_check(mut self, ignore_consistency_check: bool) -> Self {
        self.ignore_consistency_check = Some(ignore_consistency_check);
        self
    }

    pub fn fields(mut self, fields: Value) -> Self {
        self.fields = fields;
        self
    }

    pub async fn execute(self) -> SDKResult<UpdateRecordResponse> {
        use crate::common::{api_endpoints::BitableApiV1, api_utils::*};

        validate_required_field("app_token", Some(&self.app_token), "app_token 不能为空")?;
        validate_required_field("table_id", Some(&self.table_id), "table_id 不能为空")?;
        validate_required_field("record_id", Some(&self.record_id), "record_id 不能为空")?;

        let api_endpoint = BitableApiV1::RecordUpdate(
            self.app_token.clone(),
            self.table_id.clone(),
            self.record_id,
        );

        let request = ApiRequest::<UpdateRecordResponse>::put(&api_endpoint.to_url())
            .query_opt("user_id_type", self.user_id_type)
            .query_opt(
                "ignore_consistency_check",
                self.ignore_consistency_check.map(|v| v.to_string()),
            )
            .body(serialize_params(
                &UpdateRecordRequestBody {
                    fields: self.fields,
                },
                "更新记录",
            )?);

        let response = Transport::request(request, &self.config, None).await?;
        extract_response_data(response, "更新记录")
    }
}

/// 更新记录 Builder
pub struct UpdateRecordRequestBuilder {
    request: UpdateRecordRequest,
}

impl UpdateRecordRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            request: UpdateRecordRequest::new(config),
        }
    }

    pub fn app_token(mut self, app_token: String) -> Self {
        self.request = self.request.app_token(app_token);
        self
    }

    pub fn table_id(mut self, table_id: String) -> Self {
        self.request = self.request.table_id(table_id);
        self
    }

    pub fn record_id(mut self, record_id: String) -> Self {
        self.request = self.request.record_id(record_id);
        self
    }

    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.request = self.request.user_id_type(user_id_type);
        self
    }

    pub fn ignore_consistency_check(mut self, ignore_consistency_check: bool) -> Self {
        self.request = self
            .request
            .ignore_consistency_check(ignore_consistency_check);
        self
    }

    pub fn fields(mut self, fields: Value) -> Self {
        self.request = self.request.fields(fields);
        self
    }

    pub fn build(self) -> UpdateRecordRequest {
        self.request
    }
}

#[derive(Serialize)]
struct UpdateRecordRequestBody {
    fields: Value,
}

/// 更新记录响应
///
/// 包含更新后记录的完整信息，包括记录ID、更新后的字段值以及更新元数据。
///
/// # 示例
/// ```json
/// {
///   "record": {
///     "record_id": "recxxxxxxxxxxxx",
///     "fields": {
///       "姓名": "张三",
///       "邮箱": "new_email@example.com"
///     },
///     "last_modified_by": {
///       "id": "ou_xxxxxxxxxxxxxxxx",
///       "name": "李四",
///       "en_name": "Li Si"
///     },
///     "last_modified_time": 1234567890000,
///     "record_url": "https://example.feishu.cn/base/xxxxxxxxxxxxx"
///   }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateRecordResponse {
    /// 更新记录的内容
    ///
    /// 包含记录ID、更新后的所有字段值以及元数据信息。
    /// - `record_id`: 记录的唯一标识符
    /// - `fields`: 字段名到更新后字段值的映射
    /// - `last_modified_by` (可选): 最后更新者信息，需要开启 automatic_fields 参数
    /// - `last_modified_time` (可选): 最后更新时间戳（毫秒），需要开启 automatic_fields 参数
    /// - `shared_url` (可选): 记录分享链接
    /// - `record_url` (可选): 记录访问链接
    pub record: Record,
}

impl ApiResponseTrait for UpdateRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
