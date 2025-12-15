/// 删除文档 API
///
/// 根据docToken删除对应的Docs文档
/// API文档: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/file/delete-a-doc

use openlark_core::{
    api::{ApiRequest, HttpMethod},
    constants::AccessTokenType,
    error::validation_error,
    http::Transport,
    SDKResult
};
use serde::{Deserialize, Serialize};

/// 删除文档请求
#[derive(Debug, Clone, Serialize)]
pub struct DeleteDocRequest {
    /// 文档token
    pub doc_token: String,
    /// 用户ID类型
    pub user_id_type: Option<String>,
}

/// 删除文档响应
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteDocResponse {
    /// 是否成功
    pub success: bool,
}

/// 删除文档
///
/// # 参数
/// - `req`: 删除文档请求
/// - `config`: SDK配置
///
/// # 返回
/// - `SDKResult<DeleteDocResponse>`: 删除结果
pub async fn delete_doc(
    req: DeleteDocRequest,
    config: &openlark_core::config::Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<DeleteDocResponse> {
    // 参数验证
    if req.doc_token.is_empty() {
        return Err(validation_error("doc_token", "文档token不能为空"));
    }

    let endpoint = format!("/open-apis/drive/explorer/v2/file/docs/{}", req.doc_token);

    let mut query = std::collections::HashMap::new();
    if let Some(user_id_type) = &req.user_id_type {
        query.insert("user_id_type".to_string(), user_id_type.clone());
    }

    let api_req = ApiRequest {
        query,
        timeout: None,
        _phantom: std::marker::PhantomData,
        method: HttpMethod::Delete,
        url: endpoint,
    };

    let resp = Transport::<DeleteDocResponse>::request(api_req, config, option).await?;

    Ok(resp.data.unwrap_or_else(|| {
        // 如果没有data字段，创建一个空的响应
        DeleteDocResponse { success: false }
    }))
}