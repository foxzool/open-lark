/// 替换数据
///
/// 在工作表中替换指定的数据。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/replace
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use serde_json::json;

use crate::common::{api_endpoints::CcmSheetApiOld, api_utils::*};

/// 替换响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindReplaceResponse {
    /// 替换的数量
    pub replace_count: Option<i32>,
    /// 是否成功
    pub success: Option<bool>,
}

/// 替换参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindReplaceParams {
    /// 查找的内容
    pub find: String,
    /// 替换的内容
    pub replacement: String,
    /// 查找范围
    pub range: Option<String>,
    /// 是否区分大小写
    pub case_sensitive: Option<bool>,
    /// 是否匹配整个单元格
    pub match_entire_cell: Option<bool>,
}

impl ApiResponseTrait for FindReplaceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 替换数据
///
/// 在工作表中替换指定的数据。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/replace
pub async fn replace_data(
    config: &Config,
    spreadsheet_token: &str,
    params: FindReplaceParams,
) -> SDKResult<FindReplaceResponse> {
    // 使用enum+builder系统生成API端点
    let api_endpoint = CcmSheetApiOld::ReplaceRange(spreadsheet_token.to_string());

    // 创建API请求 - 使用类型安全的URL生成和标准化的参数序列化
    let api_request: ApiRequest<FindReplaceResponse> = ApiRequest::post(&api_endpoint.to_url())
        .body(json!({
            "find": params.find,
            "replacement": params.replacement,
            "range": params.range,
            "case_sensitive": params.case_sensitive,
            "match_entire_cell": params.match_entire_cell
        }));

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "替换数据")
}
