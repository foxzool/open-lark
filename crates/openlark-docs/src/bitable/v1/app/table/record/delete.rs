
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use super::batch_create::Record;

/// 删除记录请求
#[derive(Debug, Clone)]
pub struct DeleteRecordRequest {
    api_request: ApiRequest<Self>,
    /// 多维表格的唯一标识符
    app_token: String,
    /// 多维表格数据表的唯一标识符
    table_id: String,
    /// 记录的唯一标识符
    record_id: String,
    /// 用户 ID 类型
    user_id_type: Option<String>,
}

impl Default for DeleteRecordRequest {
    fn default() -> Self {
        Self {
            api_request: ApiRequest::delete("/open-apis/bitable/v1/apps/{}/tables/{}/records"),
            app_token: String::new(),
            table_id: String::new(),
            record_id: String::new(),
            user_id_type: None,
        }
    }
}

impl DeleteRecordRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::delete("/open-apis/bitable/v1/apps/{}/tables/{}/records")
                
                ,
            app_token: String::new(),
            table_id: String::new(),
            record_id: String::new(),
            user_id_type: None,
        }
    }

    pub fn builder() -> DeleteRecordRequestBuilder {
        DeleteRecordRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct DeleteRecordRequestBuilder {
    request: DeleteRecordRequest,
}

impl DeleteRecordRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.request.app_token = app_token.into();
        self
    }

    pub fn table_id(mut self, table_id: impl Into<String>) -> Self {
        self.request.table_id = table_id.into();
        self
    }

    pub fn record_id(mut self, record_id: impl Into<String>) -> Self {
        self.request.record_id = record_id.into();
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.user_id_type = Some(user_id_type.into());
        self
    }

    pub fn build(self) -> DeleteRecordRequest {
        self.request
    }
}

/// 删除记录响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRecordResponse {
    /// 被删除的记录
    pub record: Record,
}

impl ApiResponseTrait for DeleteRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除记录
pub async fn delete_record(
    request: DeleteRecordRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<DeleteRecordResponse> {
    let url = format!(
        "https://open.feishu.cn/open-apis/bitable/v1/apps/{}/tables/{}/records/{}",
        &request.app_token, &request.table_id, &request.record_id
    );
    let mut api_req = ApiRequest::<()>::delete(&url);

    // 设置查询参数
    if let Some(user_id_type) = &request.user_id_type {
        api_req = api_req.query("user_id_type", user_id_type);
    }

    let api_resp: Response<DeleteRecordResponse> =
        Transport::request(api_req, config, option).await?;
    api_resp.into_result()
}

