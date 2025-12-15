/// 上传图片
///
/// 词条图片资源上传。
/// docPath: https://open.feishu.cn/document/server-docs/baike-v1/file/upload
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use crate::common::{api_endpoints::BaikeApiV1, api_utils::*};

#[derive(Debug, serde::Deserialize)]
pub struct UploadFileResponse {
    pub data: Option<FileData>,
}

#[derive(Debug, serde::Deserialize)]
pub struct FileData {
    pub file_token: String,
    pub file_url: String,
    pub file_name: String,
    pub file_size: i64,
    pub file_type: String,
    pub upload_time: String,
}

#[derive(Debug, serde::Serialize)]
pub struct UploadFileParams {
    pub file_name: String,
    pub file_type: String,
    pub file_size: i64,
    pub file_content: Vec<u8>,
}

impl ApiResponseTrait for UploadFileResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 上传图片
///
/// 词条图片资源上传。
/// docPath: https://open.feishu.cn/document/server-docs/baike-v1/file/upload
pub async fn upload_file(
    config: &Config,
    params: UploadFileParams,
) -> SDKResult<FileData> {
    // 验证必填字段
    validate_required_field("文件名", Some(&params.file_name), "文件名不能为空")?;
    validate_required_field("文件类型", Some(&params.file_type), "文件类型不能为空")?;

    // 使用enum+builder系统生成API端点
    let api_endpoint = BaikeApiV1::FileUpload;

    // 创建multipart表单数据
    let mut form_data = reqwest::multipart::Form::new();

    // 添加文件
    let file_part = reqwest::multipart::Part::bytes(params.file_content)
        .file_name(params.file_name.clone())
        .mime_str(&params.file_type)?;
    form_data = form_data.part("file", file_part);

    // 添加文件名和文件类型字段
    form_data = form_data.text("file_name", params.file_name);
    form_data = form_data.text("file_type", params.file_type);
    form_data = form_data.text("file_size", params.file_size.to_string());

    // 创建API请求
    let api_request: ApiRequest<UploadFileResponse> =
        ApiRequest::post(&api_endpoint.to_url()).multipart(form_data);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    let resp: UploadFileResponse = response.data.ok_or_else(|| {
        openlark_core::error::validation_error("response_data", "Response data is missing")
    })?;

    resp.data.ok_or_else(|| {
        openlark_core::error::validation_error("file_data", "File data is missing")
    })
}