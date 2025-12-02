
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
use crate::common::types::{AppToken, TableId};
use crate::common::batch::{BatchCommonParams, BatchCommonBody, BatchOperationResult};

// 导入 TableData 类型
use super::create::TableData;

/// 批量新增数据表请求
#[derive(Debug, Clone)]
pub struct BatchCreateTableRequest {
    api_request: ApiRequest<Self>,
    /// 多维表格的 app_token
    app_token: AppToken,
    /// 通用批量操作参数
    common_params: BatchCommonParams,
    /// 要新增的数据表列表
    tables: Vec<TableData>,
    /// 配置信息
    config: Config,
}

impl BatchCreateTableRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::post(""),
            config,
            app_token: String::new(),
            common_params: BatchCommonParams::new(),
            tables: Vec::new(),
        }
    }

    /// 获取应用令牌
    pub fn app_token(&self) -> &AppToken {
        &self.app_token
    }

    /// 获取用户ID类型
    pub fn user_id_type(&self) -> Option<&str> {
        self.common_params.user_id_type.as_deref()
    }

    /// 获取客户端令牌
    pub fn client_token(&self) -> Option<&str> {
        self.common_params.client_token.as_deref()
    }
}

#[derive(Debug, Clone)]
pub struct BatchCreateTableRequestBuilder {
    request: BatchCreateTableRequest,
}

impl BatchCreateTableRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            request: BatchCreateTableRequest {
                api_request: ApiRequest::post("https://open.feishu.cn/open-apis/bitable/v1/placeholder"),
                app_token: String::new(),
                common_params: BatchCommonParams::new(),
                tables: Vec::new(),
                config,
            },
        }
    }

    /// 设置多维表格的 app_token
    pub fn app_token(mut self, app_token: impl Into<AppToken>) -> Self {
        self.request.app_token = app_token.into();
        self
    }

    /// 设置用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: impl ToString) -> Self {
        self.request.common_params = self.request.common_params.with_user_id_type(user_id_type);
        self
    }

    /// 设置客户端令牌，用于幂等操作
    pub fn client_token(mut self, client_token: impl ToString) -> Self {
        self.request.common_params = self.request.common_params.with_client_token(client_token);
        self
    }

    /// 设置要新增的数据表列表
    pub fn tables(mut self, tables: Vec<TableData>) -> Self {
        self.request.tables = tables;
        self
    }

    /// 构建请求
    pub fn build(self) -> BatchCreateTableRequest {
        self.request
    }
}

// 执行批量创建数据表操作的实现将在后续版本中完成

/// 批量新增数据表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreateTableResponse {
    /// 新增的数据表列表
    pub tables: Vec<BatchCreateTableResult>,
}


/// 批量创建数据表的结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreateTableResult {
    /// 数据表 ID
    #[serde(skip)]
    pub table_id: TableId,
    /// 数据表名称
    pub name: String,
    /// 数据表的默认视图 ID
    pub default_view_id: String,
    /// 批量操作结果
    #[serde(flatten)]
    pub operation: BatchOperationResult,
}

impl BatchCreateTableResult {
    /// 创建成功的结果
    pub fn success(table_id: impl Into<TableId>, name: String, default_view_id: String) -> Self {
        Self {
            table_id: table_id.into(),
            name,
            default_view_id,
            operation: BatchOperationResult::success(),
        }
    }

    /// 创建失败的结果
    pub fn failure(table_id: impl Into<TableId>, name: String, error: impl ToString) -> Self {
        Self {
            table_id: table_id.into(),
            name,
            default_view_id: String::new(),
            operation: BatchOperationResult::failure(error),
        }
    }

    /// 检查是否成功
    pub fn is_success(&self) -> bool {
        self.operation.success
    }

    /// 获取错误信息
    pub fn get_error(&self) -> Option<&str> {
        self.operation.error.as_deref()
    }

    /// 获取表格ID
    pub fn get_table_id(&self) -> &TableId {
        &self.table_id
    }
}

/// 批量新增数据表请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreateTableRequestBody {
    /// 要新增的数据表列表
    pub tables: Vec<TableData>,
}

/// 批量新增数据表
pub async fn batch_create_table(
    request: BatchCreateTableRequest,
    config: &Config,
    _option: Option<RequestOption>,
) -> SDKResult<Response<BatchCreateTableResponse>> {
    // 构建完整的API URL
    let api_url = format!("https://open.feishu.cn/open-apis/bitable/v1/apps/{}/tables:batch_create", request.app_token.as_str());

    // 构建请求体
    let body = BatchCreateTableRequestBody {
        tables: request.tables,
    };

    // 创建API请求
    let mut api_request: ApiRequest<()> = ApiRequest::post(api_url);
    api_request.body = Some(RequestData::Json(serde_json::to_value(&body)?));

    // 发送请求 - 转换为ApiRequest<()>以匹配Transport::request签名
    let mut request_for_transport: ApiRequest<()> = ApiRequest::post(api_request.url.clone())
        .body(api_request.body.unwrap_or(RequestData::Empty));

    let response = Transport::request(request_for_transport, &request.config, None).await?;

    // 解析响应
    let data: BatchCreateTableResponse = response.data
        .and_then(|data| serde_json::from_value(data).ok())
        .ok_or_else(|| validation_error("解析失败", "数据格式不正确"))?;

    Ok(Response {
        data: Some(data),
        raw_response: response.raw_response,
    })
}

