/// 增加保护范围
///
/// 为电子表格中的单元格范围添加保护，防止未经授权的修改。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v2/protection/add-protected-range
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, validation_error, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 增加保护范围请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddProtectedRangeParams {
    /// 电子表格的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 工作表ID
    #[serde(rename = "sheetId")]
    pub sheet_id: i64,
    /// 保护范围列表
    #[serde(rename = "protectedRanges")]
    pub protected_ranges: Vec<ProtectedRange>,
}

/// 保护范围
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtectedRange {
    /// 起始行索引（从0开始）
    #[serde(rename = "startRowIndex")]
    pub start_row_index: i32,
    /// 结束行索引（不包含）
    #[serde(rename = "endRowIndex")]
    pub end_row_index: i32,
    /// 起始列索引（从0开始）
    #[serde(rename = "startColumnIndex")]
    pub start_column_index: i32,
    /// 结束列索引（不包含）
    #[serde(rename = "endColumnIndex")]
    pub end_column_index: i32,
    /// 保护范围的名称
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 保护范围的描述
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 谁可以编辑此范围: "USERS" 或 "EVERYONE"
    #[serde(rename = "editors", skip_serializing_if = "Option::is_none")]
    pub editors: Option<Editors>,
    /// 是否被警告用户编辑
    #[serde(rename = "warningOnly", skip_serializing_if = "Option::is_none")]
    pub warning_only: Option<bool>,
}

/// 编辑者信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Editors {
    /// 用户列表
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<String>>,
    /// 用户组列表
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    /// 域列表
    #[serde(rename = "domainUsersCanEdit", skip_serializing_if = "Option::is_none")]
    pub domain_users_can_edit: Option<bool>,
}

/// 增加保护范围响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddProtectedRangeResponse {
    /// 操作结果
    pub data: Option<AddProtectedRangeResult>,
}

/// 增加保护范围结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddProtectedRangeResult {
    /// 电子表格的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 创建的保护范围列表
    #[serde(rename = "protectedRanges")]
    pub protected_ranges: Vec<ProtectedRangeResult>,
}

/// 保护范围结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtectedRangeResult {
    /// 保护范围ID
    #[serde(rename = "protectedRangeId")]
    pub protected_range_id: String,
    /// 起始行索引
    #[serde(rename = "startRowIndex")]
    pub start_row_index: i32,
    /// 结束行索引
    #[serde(rename = "endRowIndex")]
    pub end_row_index: i32,
    /// 起始列索引
    #[serde(rename = "startColumnIndex")]
    pub start_column_index: i32,
    /// 结束列索引
    #[serde(rename = "endColumnIndex")]
    pub end_column_index: i32,
    /// 名称
    pub name: Option<String>,
    /// 描述
    pub description: Option<String>,
    /// 创建者
    pub creator: Option<UserInfo>,
    /// 创建时间
    #[serde(rename = "createTime")]
    pub create_time: Option<i64>,
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// 用户ID
    #[serde(rename = "userId")]
    pub user_id: String,
    /// 用户名称
    pub name: Option<String>,
}

impl ApiResponseTrait for AddProtectedRangeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 增加保护范围请求
pub struct AddProtectedRangeRequest {
    config: Config,
}

impl AddProtectedRangeRequest {
    /// 创建增加保护范围请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v2/protection/add-protected-range
    pub async fn execute(
        self,
        params: AddProtectedRangeParams,
    ) -> SDKResult<AddProtectedRangeResponse> {
        // 验证必填字段
        validate_required!(params.spreadsheet_token, "电子表格token不能为空");
        if params.sheet_id <= 0 {
            return Err(validation_error("sheet_id", "工作表ID不能为空或小于等于0"));
        }
        validate_required!(params.protected_ranges, "保护范围不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint = CcmSheetApiOld::ProtectedDimension(params.spreadsheet_token.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<AddProtectedRangeResponse> = ApiRequest::post(
            &api_endpoint.to_url(),
        )
        .body(serde_json::to_value(params).map_err(|e| {
            openlark_core::error::validation_error(
                "参数序列化失败",
                &format!("无法序列化请求参数: {}", e),
            )
        })?);

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}
