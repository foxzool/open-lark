//! 导入表格
//!
//! docPath: /document/ukTMukTMukTM/uATO2YjLwkjN24CM5YjN

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::*;

use crate::common::api_endpoints::CcmSheetApiOld;

pub mod result;
pub use result::*;

/// 导入表格请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ImportReq {
    /// 需要导入的文件数据（字节数组），支持 xlsx、csv，最大不超过 20MB
    pub file: Vec<u8>,
    /// 文件名（带拓展名），导入后 sheet 标题会去掉拓展名
    pub name: String,
    /// 导入的文件夹 token，不传默认导入到根目录
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folderToken: Option<String>,
}

/// 导入表格响应（返回导入 ticket，后续可用 `import/result` 查询导入结果）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportResp {
    pub ticket: String,
}

impl ApiResponseTrait for ImportResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 导入表格
pub async fn import(
    request: ImportReq,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<ImportResp> {
    validate_required!(request.name, "name 不能为空");
    validate_required!(request.file, "file 不能为空");
    let name_lower = request.name.to_ascii_lowercase();
    if !(name_lower.ends_with(".xlsx") || name_lower.ends_with(".csv")) {
        return Err(openlark_core::error::validation_error(
            "name",
            "仅支持 xlsx、csv 格式",
        ));
    }
    if request.file.len() > 20 * 1024 * 1024 {
        return Err(openlark_core::error::validation_error(
            "file",
            "file 最大不超过 20MB",
        ));
    }

    let mut api_request: ApiRequest<ImportResp> =
        ApiRequest::post(&CcmSheetApiOld::Import.to_url())
            .body(serialize_params(&request, "导入表格")?);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "导入表格")
}
