/// 获取表格元数据
///
/// 根据 spreadsheetToken 获取电子表格的元信息。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheets/get-spreadsheet-meta
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 获取表格元数据请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSpreadsheetMetaParams {
    /// 电子表格的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
}

/// 获取表格元数据响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSpreadsheetMetaResponse {
    /// 表格元信息
    pub data: Option<SpreadsheetMeta>,
}

/// 表格元信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpreadsheetMeta {
    /// 电子表格的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 表格标题
    pub title: String,
    /// 创建者信息
    pub creator: Option<UserInfo>,
    /// 更新者信息
    pub updater: Option<UserInfo>,
    /// 创建时间
    #[serde(rename = "createTime")]
    pub create_time: i64,
    /// 更新时间
    #[serde(rename = "updateTime")]
    pub update_time: i64,
    /// 工作表列表
    pub sheets: Vec<SheetInfo>,
    /// 区域设置
    #[serde(rename = "locale")]
    pub locale: Option<String>,
    /// 时区设置
    #[serde(rename = "timeZone")]
    pub time_zone: Option<String>,
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// 用户ID
    #[serde(rename = "userId")]
    pub user_id: String,
    /// 用户名称
    pub name: String,
}

/// 工作表信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheetInfo {
    /// 工作表ID
    #[serde(rename = "sheetId")]
    pub sheet_id: i64,
    /// 工作表标题
    pub title: String,
    /// 工作表类型
    #[serde(rename = "sheetType")]
    pub sheet_type: String,
    /// 工作表索引
    pub index: i32,
    /// 是否隐藏
    pub hidden: bool,
    /// 工作表网格大小
    #[serde(rename = "gridProperties")]
    pub grid_properties: Option<GridProperties>,
}

/// 网格属性
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridProperties {
    /// 行数
    #[serde(rename = "rowCount")]
    pub row_count: i32,
    /// 列数
    #[serde(rename = "columnCount")]
    pub column_count: i32,
    /// 是否冻结行
    #[serde(rename = "frozenRowCount")]
    pub frozen_row_count: Option<i32>,
    /// 是否冻结列
    #[serde(rename = "frozenColumnCount")]
    pub frozen_column_count: Option<i32>,
}

impl ApiResponseTrait for GetSpreadsheetMetaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取表格元数据请求
pub struct GetSpreadsheetMetaRequest {
    config: Config,
}

impl GetSpreadsheetMetaRequest {
    /// 创建获取表格元数据请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheets/get-spreadsheet-meta
    pub async fn execute(
        self,
        params: GetSpreadsheetMetaParams,
    ) -> SDKResult<GetSpreadsheetMetaResponse> {
        // 验证必填字段
        validate_required!(params.spreadsheet_token, "电子表格token不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint = CcmSheetApiOld::Metainfo(params.spreadsheet_token.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<GetSpreadsheetMetaResponse> = ApiRequest::post(
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
