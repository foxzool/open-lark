/// 获取云文档信息V2
///
/// 获取云文档的详细信息。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v2/file/get
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 获取云文档信息V2请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMetaRequest {
    /// 文件token
    pub file_token: String,
    /// 用户ID类型
    pub user_id_type: Option<String>,
}

impl GetMetaRequest {
    /// 创建获取云文档信息V2请求
    ///
    /// # 参数
    /// * `file_token` - 文件token
    pub fn new(file_token: impl Into<String>) -> Self {
        Self {
            file_token: file_token.into(),
            user_id_type: None,
        }
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }
}

/// 文件信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    /// 文件token
    pub file_token: String,
    /// 文件名
    pub name: String,
    /// 文件类型
    pub r#type: String,
    /// 文件大小
    pub size: i64,
    /// 父文件夹token
    pub parent_folder_token: String,
    /// 创建时间
    pub create_time: String,
    /// 修改时间
    pub modify_time: String,
    /// 创建者信息
    pub creator: Option<CreatorInfo>,
    /// 文件URL
    pub url: Option<String>,
}

/// 创建者信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatorInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名
    pub name: Option<String>,
}

/// 获取云文档信息V2响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMetaResponse {
    /// 文件信息
    pub data: FileInfo,
}

impl ApiResponseTrait for GetMetaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取云文档信息V2
///
/// 获取云文档的详细信息。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v2/file/get
pub async fn get_meta(
    request: GetMetaRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<GetMetaResponse>> {
    let url = format!("/open-apis/drive/v2/files/{}", request.file_token);

      let mut api_request: ApiRequest<GetMetaResponse> = ApiRequest::get(&url);

    if let Some(user_id_type) = &request.user_id_type {
        api_request = api_request.query("user_id_type", user_id_type);
    }

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    Transport::request(api_request, config, None).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_meta_request() {
        let request = GetMetaRequest::new("file_token_123")
            .user_id_type("open_id");

        assert_eq!(request.file_token, "file_token_123");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_file_info_structure() {
        let creator = CreatorInfo {
            user_id: "user_123".to_string(),
            name: Some("张三".to_string()),
        };

        let file_info = FileInfo {
            file_token: "file_token_123".to_string(),
            name: "测试文档.docx".to_string(),
            r#type: "docx".to_string(),
            size: 1024000,
            parent_folder_token: "folder_token_123".to_string(),
            create_time: "2023-01-01T00:00:00Z".to_string(),
            modify_time: "2023-01-02T00:00:00Z".to_string(),
            creator: Some(creator),
            url: Some("https://example.com/file".to_string()),
        };

        assert_eq!(file_info.file_token, "file_token_123");
        assert_eq!(file_info.name, "测试文档.docx");
        assert_eq!(file_info.size, 1024000);
        assert!(file_info.creator.is_some());
        assert_eq!(file_info.creator.as_ref().unwrap().name, Some("张三".to_string()));
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(GetMetaResponse::data_format(), ResponseFormat::Data);
    }
}