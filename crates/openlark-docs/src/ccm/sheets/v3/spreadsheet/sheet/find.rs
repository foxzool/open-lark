/// 查找数据
///
/// 在工作表中查找指定的数据。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/find
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::common::{api_endpoints::CcmSheetApiOld, api_utils::*};

/// 查找请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindParams {
    /// 查找内容
    pub find: String,
    /// 查找范围
    pub range: Option<String>,
    /// 是否区分大小写
    pub case_sensitive: Option<bool>,
    /// 是否完全匹配单元格
    pub match_entire_cell: Option<bool>,
}

/// 查找响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindResponse {
    /// 查找结果
    pub data: Option<FindData>,
}

/// 查找数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindData {
    /// 找到的单元格数量
    pub cells_found: i32,
    /// 匹配的内容
    pub matches: Vec<CellMatch>,
}

/// 单元格匹配
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellMatch {
    /// 单元格位置
    pub cell: String,
    /// 匹配的内容
    pub value: String,
}

impl ApiResponseTrait for FindResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查找数据
///
/// 在工作表中查找指定的数据。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/find
pub async fn find_data(
    config: &Config,
    spreadsheet_token: &str,
    params: FindParams,
) -> SDKResult<FindResponse> {
    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::FindReplace(spreadsheet_token.to_string());

    // 创建API请求 - 使用类型安全的URL生成和标准化的参数序列化
    let api_request: ApiRequest<FindResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(json!({
            "find": params.find,
            "range": params.range,
            "case_sensitive": params.case_sensitive,
            "match_entire_cell": params.match_entire_cell
        }));

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "查找数据")
}
