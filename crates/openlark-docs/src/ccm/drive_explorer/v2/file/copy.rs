/// 复制文档 API
///
/// 根据文件 token 复制 Doc 或 Sheet 到目标文件夹中
/// API文档: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/file/copy-a-doc-or-sheet

use openlark_core::{
    api::{ApiRequest, HttpMethod},
    constants::AccessTokenType,
    error::validation_error,
    http::Transport,
    SDKResult
};
use serde::{Deserialize, Serialize};

/// 复制文档请求
#[derive(Debug, Clone, Serialize)]
pub struct CopyFileRequest {
    /// 文件token
    pub file_token: String,
    /// 目标文件夹token
    pub folder_token: String,
    /// 复制文件名称，不填则默认原文件名称
    pub name: Option<String>,
    /// 复制文件类型，可选值: doc/sheet
    #[serde(rename = "type")]
    pub file_type: String,
    /// 用户ID类型
    pub user_id_type: Option<String>,
}

/// 复制文档响应
#[derive(Debug, Clone, Deserialize)]
pub struct CopyFileResponse {
    /// 复制后的文件token
    pub file_token: String,
}

/// 复制文件
///
/// # 参数
/// - `req`: 复制文件请求
/// - `config`: SDK配置
///
/// # 返回
/// - `SDKResult<CopyFileResponse>`: 复制结果
pub async fn copy_file(
    req: CopyFileRequest,
    config: &openlark_core::config::Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<CopyFileResponse> {
    // 参数验证
    if req.file_token.is_empty() {
        return Err(validation_error("file_token", "文件token不能为空"));
    }
    if req.folder_token.is_empty() {
        return Err(validation_error("folder_token", "目标文件夹token不能为空"));
    }
    if req.file_type.is_empty() {
        return Err(validation_error("type", "文件类型不能为空"));
    }

    let endpoint = format!("/open-apis/drive/explorer/v2/file/copy/files/{}", req.file_token);

    let mut query = std::collections::HashMap::new();
    if let Some(user_id_type) = &req.user_id_type {
        query.insert("user_id_type".to_string(), user_id_type.clone());
    }

    let api_req = ApiRequest {
        query,
        timeout: None,
        _phantom: std::marker::PhantomData,
        method: HttpMethod::Post,
        url: endpoint,
        body: Some(serde_json::to_vec(&req)?),
    };

    let resp = Transport::<CopyFileResponse>::request(api_req, config, option).await?;

    Ok(resp.data.unwrap_or_else(|| {
        // 如果没有data字段，创建一个空的响应
        CopyFileResponse {
            file_token: "".to_string(),
        }
    }))
}