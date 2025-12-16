/// 创建筛选器
///
/// 此接口用于在工作表中创建筛选器，支持多种筛选条件和规则。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets/v3/spreadsheet/sheet/filter/create
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::validation_error,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use super::{FilterRange, FilterCondition};

/// 创建筛选器请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFilterRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 工作表ID
    pub sheet_id: String,
    /// 筛选器范围
    pub range: FilterRange,
    /// 筛选条件
    pub condition: FilterCondition,
}

/// 创建筛选器响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFilterResponse {
    /// 筛选器ID
    pub filter_id: String,
    /// 是否成功
    pub success: bool,
}

impl ApiResponseTrait for CreateFilterResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CreateFilterRequest {
    /// 创建创建筛选器请求
    ///
    /// # 参数
    /// * `spreadsheet_token` - 电子表格token
    /// * `sheet_id` - 工作表ID
    /// * `range` - 筛选器范围
    /// * `condition` - 筛选条件
    pub fn new(
        spreadsheet_token: impl Into<String>,
        sheet_id: impl Into<String>,
        range: FilterRange,
        condition: FilterCondition,
    ) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            sheet_id: sheet_id.into(),
            range,
            condition,
        }
    }

    /// 发送请求
    pub async fn send(&self, config: &Config) -> SDKResult<CreateFilterResponse> {
        let endpoint = format!(
            "{}/spreadsheets/{}/sheets/{}/filters",
            crate::common::api_endpoints::SHEETS_API_V3,
            self.spreadsheet_token,
            self.sheet_id
        );

        let json_value = serde_json::to_value(self).map_err(|e| {
            validation_error("parameter",format!("请求序列化失败: {}", e))
        })?;
        let api_request: ApiRequest<CreateFilterResponse> = ApiRequest::post(&endpoint).body(json_value);
        let response = Transport::request(api_request, config, None).await?;

        response.data.ok_or_else(|| {
            validation_error("parameter", "响应数据为空")
        })
    }
}
