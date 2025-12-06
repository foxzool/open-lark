//! Bitable 更新数据表API
///
/// API文档: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app/table/patch
///
/// 提供数据表的增量更新功能，使用 JSON Patch 格式进行部分字段更新。

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, RequestData, ResponseFormat},
    config::Config,
    error::{validation_error, SDKResult},
    http::Transport,
};
use serde::{Deserialize, Serialize};

// 导入 TableField 类型
use super::create::TableField;

/// 更新数据表请求 (Patch)
#[allow(dead_code)]
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

        // 🚀 使用新的enum+builder系统生成API端点
        // 替代传统的字符串拼接方式，提供类型安全和IDE自动补全
        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint = BitableApiV1::table_patch(&self.app_token, &self.table_id);

        // 构建请求体
        let request_body = PatchTableRequestBody {
            name: self.name,
            fields: self.fields,
        };

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<PatchTableResponse> =
            ApiRequest::put(&api_endpoint.to_url())
                .body(RequestData::Binary(serde_json::to_vec(&request_body)?));

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            validation_error("响应数据为空", "服务器没有返回有效的数据")
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
    pub data: PatchTableResponseData,
}

impl ApiResponseTrait for PatchTableResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
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
