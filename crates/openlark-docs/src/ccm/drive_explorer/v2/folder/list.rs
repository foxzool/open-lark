/// 获取文件夹下的文档清单 API
///
/// 根据folderToken获取该文件夹的文档清单，如doc、sheet、file、bitable、folder
/// API文档: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/folder/get-folder-children

use openlark_core::{
    api::{ApiRequest, HttpMethod},
    constants::AccessTokenType,
    error::validation_error,
    http::Transport,
    SDKResult
};
use serde::{Deserialize, Serialize};

/// 获取文件夹子文档请求
#[derive(Debug, Clone, Serialize)]
pub struct ListFolderChildrenRequest {
    /// 文件夹token
    pub folder_token: String,
    /// 用户ID类型
    pub user_id_type: Option<String>,
    /// 页面大小
    pub page_size: Option<i32>,
    /// 页面token
    pub page_token: Option<String>,
    /// 文件类型过滤
    pub file_type: Option<String>,
    /// 搜索关键字
    pub search_key: Option<String>,
}

/// 文件信息
#[derive(Debug, Clone, Deserialize)]
pub struct FileInfo {
    /// 文件token
    pub file_token: String,
    /// 文件类型
    #[serde(rename = "type")]
    pub file_type: String,
    /// 文件名称
    pub name: String,
    /// 创建时间
    pub create_time: String,
    /// 修改时间
    pub modify_time: String,
}

/// 获取文件夹子文档响应
#[derive(Debug, Clone, Deserialize)]
pub struct ListFolderChildrenResponse {
    /// 文件列表
    pub files: Vec<FileInfo>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 下一页token
    pub page_token: Option<String>,
}

/// 获取文件夹下的文档清单
///
/// # 参数
/// - `req`: 获取文件夹子文档请求
/// - `config`: SDK配置
///
/// # 返回
/// - `SDKResult<ListFolderChildrenResponse>`: 子文档列表
pub async fn list_folder_children(
    req: ListFolderChildrenRequest,
    config: &openlark_core::config::Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<ListFolderChildrenResponse> {
    // 参数验证
    if req.folder_token.is_empty() {
        return Err(validation_error("folder_token", "文件夹token不能为空"));
    }

    let endpoint = format!("/open-apis/drive/explorer/v2/folder/{}/children", req.folder_token);

    let mut query = std::collections::HashMap::new();
    if let Some(user_id_type) = &req.user_id_type {
        query.insert("user_id_type".to_string(), user_id_type.clone());
    }
    if let Some(page_size) = req.page_size {
        query.insert("page_size".to_string(), page_size.to_string());
    }
    if let Some(page_token) = &req.page_token {
        query.insert("page_token".to_string(), page_token.clone());
    }
    if let Some(file_type) = &req.file_type {
        query.insert("type".to_string(), file_type.clone());
    }
    if let Some(search_key) = &req.search_key {
        query.insert("search_key".to_string(), search_key.clone());
    }

    let api_req = ApiRequest {
        query,
        timeout: None,
        _phantom: std::marker::PhantomData,
        method: HttpMethod::Get,
        url: endpoint,
    };

    let resp = Transport::<ListFolderChildrenResponse>::request(api_req, config, option).await?;

    Ok(resp.data.unwrap_or_else(|| {
        // 如果没有data字段，创建一个空的响应
        ListFolderChildrenResponse {
            files: vec![],
            has_more: false,
            page_token: None,
        }
    }))
}