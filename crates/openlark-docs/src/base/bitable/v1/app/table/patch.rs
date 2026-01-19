//! Bitable 更新数据表
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table/patch
//!
//! 说明：
//! - 该接口用于更新数据表的基本信息（当前主要是更新数据表名称）。

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::SDKResult,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 更新数据表请求 (Patch)
#[derive(Debug, Clone)]
pub struct PatchTableRequest {
    /// 配置信息
    config: Config,
    /// 多维表格的 app_token
    app_token: String,
    /// 数据表的 table_id
    table_id: String,
    /// 表名
    name: Option<String>,
}

impl PatchTableRequest {
    /// 创建新的更新请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            table_id: String::new(),
            name: None,
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

    /// 执行请求
    pub async fn execute(self) -> SDKResult<PatchTableResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<PatchTableResponse> {
        // 参数验证
        validate_required!(self.app_token.trim(), "app_token");

        validate_required!(self.table_id.trim(), "table_id");

        let name = self
            .name
            .ok_or_else(|| openlark_core::error::validation_error("name", "数据表名称不能为空"))?;

        // 验证表名长度
        if name.trim().is_empty() {
            return Err(openlark_core::error::validation_error("name", "数据表名称不能为空"));
        }
        if name.len() > 100 {
            return Err(openlark_core::error::validation_error("name", "数据表名称长度不能超过100个字符"));
        }

        // 名称不允许包含 `/ \\ ? * : [ ]` 等特殊字符
        if name.contains('/') {
            return Err(openlark_core::error::validation_error("name", "数据表名称不能包含 '/'"));
        }
        if name.contains('\\') {
            return Err(openlark_core::error::validation_error("name", "数据表名称不能包含 '\\\\'"));
        }
        if name.contains('?') {
            return Err(openlark_core::error::validation_error("name", "数据表名称不能包含 '?'"));
        }
        if name.contains('*') {
            return Err(openlark_core::error::validation_error("name", "数据表名称不能包含 '*'"));
        }
        if name.contains(':') {
            return Err(openlark_core::error::validation_error("name", "数据表名称不能包含 ':'"));
        }
        if name.contains('[') || name.contains(']') {
            return Err(openlark_core::error::validation_error("name", "数据表名称不能包含 '[' 或 ']'"));
        }

        // 🚀 使用新的enum+builder系统生成API端点
        // 替代传统的字符串拼接方式，提供类型安全和IDE自动补全
        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint = BitableApiV1::TablePatch(self.app_token.clone(), self.table_id.clone());

        // 构建请求体
        let request_body = PatchTableRequestBody { name };

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<PatchTableResponse> =
            ApiRequest::patch(&api_endpoint.to_url()).body(serde_json::to_vec(&request_body)?);

        // 发送请求
        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据"))
    }
}





/// 更新数据表请求体
#[derive(Serialize)]
struct PatchTableRequestBody {
    /// 表名
    name: String,
}

/// 更新数据表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchTableResponse {
    /// 新的数据表名称
    pub name: Option<String>,
}

impl ApiResponseTrait for PatchTableResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
