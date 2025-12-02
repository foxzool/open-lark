//! 更新数据表模块 (Patch 方式)
//!
//! 提供数据表的增量更新功能，使用 JSON Patch 格式进行部分字段更新。

use openlark_core::{
    api::{ApiRequest, RequestData},
    config::Config,
    error::{validation_error, SDKResult},
    http::Transport,
};
use serde::{Deserialize, Serialize};

// 导入 TableField 类型
use super::create::TableField;

/// 更新数据表请求 (Patch)
#[derive(Debug, Clone)]
pub struct PatchTableRequest {
    /// 配置信息
    config: Config,
    api_request: ApiRequest<PatchTableResponse>,
    /// 多维表格的 app_token
    app_token: String,
    /// 数据表的 table_id
    table_id: String,
    /// 表名
    name: Option<String>,
    /// 表字段
    fields: Option<Vec<TableField>>,
}

impl PatchTableRequest {
    /// 创建新的更新请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            api_request: ApiRequest::put(""),
            app_token: String::new(),
            table_id: String::new(),
            name: None,
            fields: None,
        }
    }

    /// 设置应用token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    /// 设置数据表ID
    pub fn table_id(mut self, table_id: String) -> Self {
        self.table_id = table_id;
        self
    }

    /// 设置表名
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// 设置字段
    pub fn fields(mut self, fields: Vec<TableField>) -> Self {
        self.fields = Some(fields);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<PatchTableResponse> {
        // 参数验证
        if self.app_token.trim().is_empty() {
            return Err(validation_error("app_token", "应用token不能为空"));
        }

        if self.table_id.trim().is_empty() {
            return Err(validation_error("table_id", "数据表ID不能为空"));
        }

        // 验证至少有一个更新字段
        if self.name.is_none() && self.fields.is_none() {
            return Err(validation_error(
                "更新字段",
                "至少需要提供一个更新字段（name或fields）",
            ));
        }

        // 验证表名长度
        if let Some(ref name) = self.name {
            if name.trim().is_empty() {
                return Err(validation_error("name", "数据表名称不能为空"));
            }
            if name.len() > 100 {
                return Err(validation_error("name", "数据表名称长度不能超过100个字符"));
            }
        }

        // 构建完整的API URL
        let api_url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables/{}",
            self.config.base_url, self.app_token, self.table_id
        );

        // 设置API URL
        let mut api_request = self.api_request;
        api_request.url = api_url;

        // 构建请求体
        let request_body = PatchTableRequestBody {
            name: self.name,
            fields: self.fields,
        };

        // 设置请求体
        api_request.body = Some(RequestData::Json(serde_json::to_value(&request_body)?));

        // 发送请求 - 转换为ApiRequest<()>以匹配Transport::request签名
        let request_for_transport: ApiRequest<()> = ApiRequest::put(api_request.url.clone())
            .body(api_request.body.unwrap_or(RequestData::Empty));

        let response = Transport::request(request_for_transport, &self.config, None).await?;

        // 解析响应数据
        let table_data: PatchTableResponseData = response
            .data
            .and_then(|data| serde_json::from_value(data).ok())
            .ok_or_else(|| validation_error("解析更新数据表响应失败", "响应数据格式不正确"))?;

        Ok(PatchTableResponse {
            table: table_data,
            success: response.raw_response.is_success(),
        })
    }
}

/// 更新数据表请求构建器
pub struct PatchTableRequestBuilder {
    request: PatchTableRequest,
}

impl PatchTableRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: PatchTableRequest::new(config),
        }
    }

    /// 设置应用token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.request = self.request.app_token(app_token);
        self
    }

    /// 设置数据表ID
    pub fn table_id(mut self, table_id: String) -> Self {
        self.request = self.request.table_id(table_id);
        self
    }

    /// 设置表名
    pub fn name(mut self, name: String) -> Self {
        self.request = self.request.name(name);
        self
    }

    /// 设置字段
    pub fn fields(mut self, fields: Vec<TableField>) -> Self {
        self.request = self.request.fields(fields);
        self
    }

    /// 构建请求
    pub fn build(self) -> PatchTableRequest {
        self.request
    }
}

/// 更新数据表请求体
#[derive(Serialize)]
struct PatchTableRequestBody {
    /// 表名
    name: Option<String>,
    /// 表字段
    fields: Option<Vec<TableField>>,
}

/// 更新数据表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchTableResponse {
    /// 更新的数据表信息
    pub table: PatchTableResponseData,
    /// 操作结果
    pub success: bool,
}

/// 更新数据表响应数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchTableResponseData {
    /// 数据表的 table_id
    pub table_id: String,
    /// 数据表的名字
    pub name: String,
    /// 数据表的版本号
    pub revision: i32,
    /// 数据表字段列表
    pub fields: Vec<TableField>,
    /// 数据表记录数量
    pub record_count: i32,
}
