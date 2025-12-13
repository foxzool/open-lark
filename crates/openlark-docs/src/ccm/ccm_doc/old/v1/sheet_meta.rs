/// 获取旧版文档中的电子表格元数据
///
/// 根据 docToken 获取文档中的电子表格的元数据。
/// API文档: https://open.feishu.cn/document/server-docs/historic-version/docs/document/obtain-sheet-meta-info-in-doc
/// 对应CSV记录: https://open.feishu.cn/document/server-docs/historic-version/docs/document/obtain-sheet-meta-info-in-doc
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmDocApiOld;

/// 获取旧版文档中的电子表格元数据请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSheetMetaParams {
    /// 文档token
    #[serde(rename = "doc_token")]
    pub doc_token: String,
}

/// 获取旧版文档中的电子表格元数据响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSheetMetaResponse {
    /// 电子表格元信息
    pub data: Option<SheetMeta>,
}

/// 电子表格元信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheetMeta {
    /// 电子表格token
    #[serde(rename = "spreadsheet_token")]
    pub spreadsheet_token: String,
    /// 电子表格标题
    pub title: String,
    /// 工作表列表
    pub sheets: Vec<SheetInfo>,
}

/// 工作表信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheetInfo {
    /// 工作表ID
    #[serde(rename = "sheet_id")]
    pub sheet_id: i64,
    /// 工作表标题
    pub title: String,
    /// 工作表索引
    pub index: i64,
}

impl ApiResponseTrait for GetSheetMetaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取旧版文档中的电子表格元数据请求
pub struct GetSheetMetaRequest {
    config: Config,
}

impl GetSheetMetaRequest {
    /// 创建获取旧版文档中的电子表格元数据请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/server-docs/historic-version/docs/document/obtain-sheet-meta-info-in-doc
    /// 对应CSV记录: https://open.feishu.cn/document/server-docs/historic-version/docs/document/obtain-sheet-meta-info-in-doc
    pub async fn execute(self, params: GetSheetMetaParams) -> SDKResult<GetSheetMetaResponse> {
        // 验证必填字段
        validate_required!(params.doc_token, "文档token不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint = CcmDocApiOld::SheetMeta(params.doc_token.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<GetSheetMetaResponse> = ApiRequest::get(&api_endpoint.to_url());

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}
