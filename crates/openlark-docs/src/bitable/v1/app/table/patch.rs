//! 更新数据表模块 (Patch 方式)
//!
//! 提供数据表的增量更新功能，使用 JSON Patch 格式进行部分字段更新。

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat, HttpMethod},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

// 导入 TableField 类型
use super::create::TableField;

/// 更新数据表请求 (Patch)
pub struct PatchTableRequest {
    api_request: ApiRequest<Self>,
    /// 多维表格的 app_token
    pub app_token: String,
    /// 数据表的 table_id
    pub table_id: String,
    /// 表名
    pub name: Option<String>,
    /// 表字段
    pub fields: Option<Vec<TableField>>,
}

impl Default for PatchTableRequest {
    fn default() -> Self {
        Self {
            api_request: ApiRequest::put("https://open.feishu.cn/open-apis/bitable/v1/apps/{}/tables/{}"),
            app_token: String::new(),
            table_id: String::new(),
            name: None,
            fields: None,
        }
    }
}

impl PatchTableRequest {
    /// 创建新的更新请求
    pub fn new(config: Config) -> Self {
        Self::default()
    }

    /// 创建构建器
    pub fn builder() -> PatchTableRequestBuilder {
        PatchTableRequestBuilder::default()
    }
}

/// 更新数据表请求构建器
pub struct PatchTableRequestBuilder {
    request: PatchTableRequest,
}

impl Default for PatchTableRequestBuilder {
    fn default() -> Self {
        Self {
            request: PatchTableRequest::default(),
        }
    }
}

impl PatchTableRequestBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置应用 token
    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.request.app_token = app_token.into();
        self
    }

    /// 设置表格 ID
    pub fn table_id(mut self, table_id: impl Into<String>) -> Self {
        self.request.table_id = table_id.into();
        self
    }

    /// 设置表名
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.request.name = Some(name.into());
        self
    }

    /// 设置字段
    pub fn fields(mut self, fields: Vec<TableField>) -> Self {
        self.request.fields = Some(fields);
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
pub struct PatchTableResponse {
    /// 更新的数据表信息
    pub table: PatchTableResponseData,
}

/// 更新数据表响应数据
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

impl ApiResponseTrait for PatchTableResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

