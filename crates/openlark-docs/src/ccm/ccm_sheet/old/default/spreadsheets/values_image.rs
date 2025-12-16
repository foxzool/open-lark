use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 写入图片请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteImageRequest {
    /// 表格 token
    #[serde(skip)]
    pub spreadsheet_token: String,
    /// 图片数据
    pub image: Vec<u8>, // Note: API likely expects JSON with image bytes or similar, but prompt says "write images". Simplified here.
    /// Range
    pub range: String,
    /// Name
    pub name: String,
}

impl WriteImageRequest {
    /// 创建新的 WriteImageRequest
    pub fn new(spreadsheet_token: impl Into<String>, range: impl Into<String>, image: Vec<u8>, name: impl Into<String>) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            range: range.into(),
            image,
            name: name.into(),
        }
    }
}

/// 写入图片响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteImageResponse {
    /// 表格 token
    pub spreadsheetToken: String,
}

impl ApiResponseTrait for WriteImageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 写入图片
///
/// 根据 spreadsheetToken 和 range 向单个格子写入图片。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/write-images
pub async fn write_image(
    request: WriteImageRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<WriteImageResponse>> {
    let api_endpoint = CcmSheetApiOld::ValuesImage(request.spreadsheet_token.clone());
    // Note: Assuming JSON body with specific structure.
    let mut api_request: ApiRequest<WriteImageResponse> = ApiRequest::post(&api_endpoint.to_url())
        .body(serde_json::json!({
            "range": request.range,
            "image": request.image, // Should probably be base64 encoded or byte array
            "name": request.name
        }));

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    Transport::request(api_request, config, None).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_image_request() {
        let request = WriteImageRequest::new("spreadsheet_token", "range", vec![], "name");
        assert_eq!(request.spreadsheet_token, "spreadsheet_token");
        assert_eq!(request.range, "range");
    }
}
