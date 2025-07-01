use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        req_option, SDKResult,
    },
    service::sheets::v2::SpreadsheetService,
};

/// 写入图片
#[derive(Serialize, Debug, Default)]
pub struct WriteImageRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    #[serde(skip)]
    spreadsheet_token: String,
    /// 查询范围 range=`<sheetId>!<开始格子>:<结束格子>`
    /// 如：xxxx!A1:D5，详见在线表格开发指南。此处限定为一个格子，如: xxxx!A1:A1
    range: String,
    /// 需要写入的图片二进制流，支持 "PNG", "JPEG", "JPG", "GIF", "BMP", "JFIF", "EXIF", "TIFF",
    /// "BPG", "HEIC" 等图片格式
    image: Vec<u8>,
    /// 写入的图片名字
    name: String,
}

impl WriteImageRequest {
    pub fn builder() -> WriteImageRequestBuilder {
        WriteImageRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct WriteImageRequestBuilder {
    request: WriteImageRequest,
}

impl WriteImageRequestBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    pub fn range(mut self, range: impl ToString) -> Self {
        self.request.range = range.to_string();
        self
    }

    pub fn image(mut self, image: Vec<u8>) -> Self {
        self.request.image = image;
        self
    }

    pub fn name(mut self, name: impl ToString) -> Self {
        self.request.name = name.to_string();
        self
    }

    pub fn build(mut self) -> WriteImageRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

/// 写入图片响应体
#[derive(Debug, Deserialize)]
pub struct WriteImageResponse {
    /// spreadsheet 的 token
    #[serde(rename = "spreadsheetToken")]
    pub spread_sheet_token: String,
    /// spreadsheet 的版本号
    pub revision: i32,
    /// 写入图片的range
    #[serde(rename = "updateRange")]
    pub update_range: String,
}

impl ApiResponseTrait for WriteImageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl SpreadsheetService {
    /// 该接口用于根据 spreadsheetToken 和 range 向单个格子写入图片。
    pub async fn write_image(
        &self,
        request: WriteImageRequest,
        option: Option<req_option::RequestOption>,
    ) -> SDKResult<BaseResponse<WriteImageResponse>> {
        let mut api_req = request.api_request;
        api_req.api_path = format!(
            "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values_image",
            spreadsheet_token = request.spreadsheet_token
        );
        api_req.http_method = reqwest::Method::POST;
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::App];

        let api_resp = crate::core::http::Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}
