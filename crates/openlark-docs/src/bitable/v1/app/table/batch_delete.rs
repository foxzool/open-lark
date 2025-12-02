
use openlark_core::{
    api::{ApiRequest, RequestData, Response},
    config::Config,
    error::validation_error,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

// 导入通用批量操作模块
use crate::common::batch::{BatchCommonParams, BatchCommonBody, BatchOperationResult};

/// 批量删除数据表请求
#[derive(Debug, Clone)]pub struct BatchDeleteTableRequest {
    api_request: ApiRequest<Self>,
    /// 多维表格的 app_token
    app_token: String,
    /// 用户 ID 类型
    user_id_type: Option<String>,
    /// 格式为标准的 uuidv4，操作的唯一标识，用于幂等的进行更新操作
    client_token: Option<String>,
    /// 要删除的数据表 ID 列表
    table_ids: Vec<String>,
    /// 配置信息
    config: Config,
}


impl Default for BatchDeleteTableRequest {
    fn default() -> Self {
        Self {
            api_request: ApiRequest::delete("https://open.feishu.cn/open-apis/bitable/v1/apps/{}/tables/batch_delete"),
            config: Config::default(),
            app_token: String::new(),
            user_id_type: None,
            client_token: None,
            table_ids: Vec::new(),
        }
    }
}

impl BatchDeleteTableRequest {
    pub fn new(config: Config) -> Self {
        let mut request = Self::default();
        request.config = config;
        request
    }
}

pub struct BatchDeleteTableRequestBuilder {
    request: BatchDeleteTableRequest,
}

impl Default for BatchDeleteTableRequestBuilder {
    fn default() -> Self {
        Self {
            request: BatchDeleteTableRequest::default(),
        }
    }
}

impl BatchDeleteTableRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置多维表格的 app_token
    pub fn app_token(mut self, app_token: impl ToString) -> Self {
        self.request.app_token = app_token.to_string();
        self
    }

    /// 设置用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: impl ToString) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 设置客户端令牌，用于幂等操作
    pub fn client_token(mut self, client_token: impl ToString) -> Self {
        self.request.client_token = Some(client_token.to_string());
        self
    }

    /// 设置要删除的数据表 ID 列表
    pub fn table_ids(mut self, table_ids: Vec<String>) -> Self {
        self.request.table_ids = table_ids;
        self
    }

    /// 构建请求
    pub fn build(self) -> BatchDeleteTableRequest {
        self.request
    }
}

// 执行批量删除数据表操作的实现将在后续版本中完成

/// 批量删除数据表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchDeleteTableResponse {
    /// 删除的数据表结果列表
    pub results: Vec<BatchDeleteTableResult>,
}

/// 批量删除数据表的结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchDeleteTableResult {
    /// 数据表 ID
    pub table_id: String,
    /// 是否删除成功
    pub success: bool,
    /// 错误信息（如果删除失败）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// 请求体结构
#[derive(Serialize)]
struct BatchDeleteTableRequestBody {
    table_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_token: Option<String>,
}

/// 批量删除数据表
pub async fn batch_delete_table(
    request: BatchDeleteTableRequest,
    _config: &Config,
    _option: Option<RequestOption>,
) -> SDKResult<Response<BatchDeleteTableResponse>> {
    // 构建完整的API URL
    let api_url = format!("https://open.feishu.cn/open-apis/bitable/v1/apps/{}/tables:batch_delete", &request.app_token);

    // 构建请求体
    let body = BatchDeleteTableRequestBody {
        table_ids: request.table_ids,
        user_id_type: request.user_id_type,
        client_token: request.client_token,
    };

    // 创建API请求
    let api_request = ApiRequest::<()>::delete(api_url)
        .body(RequestData::Json(serde_json::to_value(&body)?));

    // 发送请求 - 转换为ApiRequest<()>以匹配Transport::request签名
    let mut request_for_transport: ApiRequest<()> = ApiRequest::delete(api_request.url.clone())
        .body(api_request.body.unwrap_or(RequestData::Empty));

    let response = Transport::request(request_for_transport, &request.config, None).await?;

    // 解析响应
    let data: BatchDeleteTableResponse = response.data
        .and_then(|data| serde_json::from_value(data).ok())
        .ok_or_else(|| validation_error("解析失败", "数据格式不正确"))?;

    Ok(Response {
        data: Some(data),
        raw_response: response.raw_response,
    })
}

