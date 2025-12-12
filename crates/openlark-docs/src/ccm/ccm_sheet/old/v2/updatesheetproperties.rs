//! 更新工作表属性
//!
//! 根据 spreadsheetToken 和 updateSheetPropertiesRequest 更新指定工作表的属性。
//! API文档: https://open.feishu.cn/document/server-docs/docs/sheets-v3/sheets/update-sheet-properties

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 更新工作表属性请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSheetPropertiesParams {
    /// 电子表格的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 更新工作表属性请求
    pub properties: SheetPropertyUpdate,
}

/// 工作表属性更新请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheetPropertyUpdate {
    /// 工作表ID
    #[serde(rename = "sheetId")]
    pub sheet_id: i64,
    /// 工作表标题
    pub title: Option<String>,
    /// 工作表索引
    pub index: Option<i32>,
    /// 工作表是否隐藏
    pub hidden: Option<bool>,
    /// 工作表颜色
    pub color: Option<SheetColor>,
    /// 冻结行数
    #[serde(rename = "frozenRowCount")]
    pub frozen_row_count: Option<i32>,
    /// 冻结列数
    #[serde(rename = "frozenColumnCount")]
    pub frozen_column_count: Option<i32>,
    /// 网格线是否显示
    #[serde(rename = "gridLinesVisible")]
    pub grid_lines_visible: Option<bool>,
}

/// 工作表颜色
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheetColor {
    /// 红色分量 (0-1)
    pub red: f32,
    /// 绿色分量 (0-1)
    pub green: f32,
    /// 蓝色分量 (0-1)
    pub blue: f32,
}

/// 更新工作表属性响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSheetPropertiesResponse {
    /// 更新结果
    pub data: Option<UpdateSheetPropertiesResult>,
}

/// 更新工作表属性结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSheetPropertiesResult {
    /// 电子表格的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 更新的工作表属性
    pub properties: Option<SheetProperties>,
}

/// 工作表属性
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheetProperties {
    /// 工作表ID
    #[serde(rename = "sheetId")]
    pub sheet_id: i64,
    /// 工作表标题
    pub title: String,
    /// 工作表索引
    pub index: i32,
    /// 工作表是否隐藏
    pub hidden: bool,
    /// 工作表颜色
    pub color: Option<SheetColor>,
    /// 冻结行数
    #[serde(rename = "frozenRowCount")]
    pub frozen_row_count: i32,
    /// 冻结列数
    #[serde(rename = "frozenColumnCount")]
    pub frozen_column_count: i32,
    /// 网格线是否显示
    #[serde(rename = "gridLinesVisible")]
    pub grid_lines_visible: bool,
}

impl ApiResponseTrait for UpdateSheetPropertiesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新工作表属性请求
pub struct UpdateSheetPropertiesRequest {
    config: Config,
}

impl UpdateSheetPropertiesRequest {
    /// 创建更新工作表属性请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/server-docs/docs/sheets-v3/sheets/update-sheet-properties
    pub async fn execute(
        self,
        params: UpdateSheetPropertiesParams,
    ) -> SDKResult<UpdateSheetPropertiesResponse> {
        // 验证必填字段
        validate_required!(params.spreadsheet_token, "电子表格token不能为空");
  
        // 使用enum+builder系统生成API端点
        let api_endpoint = CcmSheetApiOld::UpdateSheetProperties(params.spreadsheet_token.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<UpdateSheetPropertiesResponse> =
            ApiRequest::post(&api_endpoint.to_url())
                .body(serde_json::to_value(params).map_err(|e| {
                    openlark_core::error::validation_error(
                        "参数序列化失败",
                        &format!("无法序列化请求参数: {}", e)
                    )
                })?);

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}