//! 写入图片
//!
//! docPath: /document/ukTMukTMukTM/uUDNxYjL1QTM24SN0EjN

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::*;
use crate::common::api_endpoints::CcmSheetApiOld;

/// 图片数据：支持二进制数组或 base64 字符串
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ImageData {
    Bytes(Vec<u8>),
    Base64(String),
}

impl Default for ImageData {
    fn default() -> Self {
        Self::Bytes(Vec::new())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ValuesImageRequest {
    pub range: String,
    pub image: ImageData,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ValuesImageResponse {
    pub revision: i32,
    pub spreadsheetToken: String,
    #[serde(alias = "updateRange")]
    pub updatedRange: String,
}

impl ApiResponseTrait for ValuesImageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 写入图片
pub async fn values_image(
    spreadsheet_token: String,
    request: ValuesImageRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<ValuesImageResponse> {
    validate_required!(spreadsheet_token, "spreadsheet_token 不能为空");
    validate_required!(request.range, "range 不能为空");
    validate_required!(request.name, "name 不能为空");
    match &request.image {
        ImageData::Bytes(bytes) => {
            if bytes.is_empty() {
                return Err(openlark_core::error::validation_error(
                    "image",
                    "image 不能为空",
                ));
            }
        }
        ImageData::Base64(s) => {
            if s.trim().is_empty() {
                return Err(openlark_core::error::validation_error(
                    "image",
                    "image 不能为空",
                ));
            }
        }
    }
    let name_lower = request.name.to_ascii_lowercase();
    let allowed = [
        "png", "jpeg", "jpg", "gif", "bmp", "jfif", "exif", "tiff", "bpg", "heic",
    ];
    let ext = name_lower.rsplit('.').next().unwrap_or_default();
    if ext.is_empty() || !allowed.contains(&ext) {
        return Err(openlark_core::error::validation_error(
            "name",
            "不支持的图片后缀名",
        ));
    }

    let api_endpoint = CcmSheetApiOld::ValuesImage(spreadsheet_token);
    let mut api_request: ApiRequest<ValuesImageResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&request, "写入图片")?);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "写入图片")
}
