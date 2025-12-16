/// 上传图片
///
/// 词条图片资源上传。
/// docPath: https://open.feishu.cn/document/lingo-v1/file/upload
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use crate::common::{api_endpoints::LingoApiV1, api_utils::*};

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
/// docPath: https://open.feishu.cn/document/lingo-v1/file/upload
pub async fn upload_file(
    config: &Config,
    params: UploadFileParams,
) -> SDKResult<FileData> {
    // 验证必填字段
    validate_required_field("文件名", Some(&params.file_name), "文件名不能为空")?;
    validate_required_field("文件类型", Some(&params.file_type), "文件类型不能为空")?;

    // 使用enum+builder系统生成API端点
    let api_endpoint = LingoApiV1::FileUpload;

    // 创建multipart表单数据
    let mut form_data = reqwest::multipart::Form::new();

    // 添加文件
    let file_part = reqwest::multipart::Part::bytes(params.file_content)
        .file_name(params.file_name.clone())
        .mime_str(&params.file_type)
        .map_err(|e| openlark_core::error::CoreError::network_msg(e.to_string()))?;
    form_data = form_data.part("file", file_part);

    // 添加文件名和文件类型字段
    form_data = form_data.text("file_name", params.file_name);
    form_data = form_data.text("file_type", params.file_type);
    form_data = form_data.text("file_size", params.file_size.to_string());

    // 使用 reqwest client 直接发送请求
    let url = api_endpoint.to_url();
    let full_url = format!("{}{}", config.base_url(), url);

    // 获取应用Token (使用空ticket，假设自建应用或ticket已缓存)
    let token = {
        let token_manager = config.token_manager.lock().await;
        token_manager.get_app_access_token(config, "", &config.app_ticket_manager)
            .await
            .map_err(|e| openlark_core::error::CoreError::network_msg(format!("Failed to get token: {}", e)))?
    };

    let response = config.http_client
        .post(&full_url)
        .multipart(form_data)
        .bearer_auth(token)
        .send()
        .await
        .map_err(|e| openlark_core::error::CoreError::network_msg(e.to_string()))?;

    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        return Err(openlark_core::error::CoreError::api_error(status.as_u16() as i32, &url, text, None::<String>));
    }

    let resp: UploadFileResponse = response.json().await.map_err(|e| openlark_core::error::CoreError::network_msg(e.to_string()))?;
    
    resp.data.ok_or_else(|| {
        openlark_core::error::validation_error("file_data", "File data is missing")
    })
}