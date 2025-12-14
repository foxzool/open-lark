/// 批量获取文件元数据
///
/// 该接口用于根据 token 获取各类文件的元数据
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/file/batch_query
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 批量获取文件元数据请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchQueryMetaRequest {
    /// 文件token列表，最多50个
    pub file_tokens: Vec<String>,
}

/// 文件元数据信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMeta {
    /// 文件token
    pub file_token: String,
    /// 文件名
    pub name: String,
    /// 文件类型
    pub r#type: String,
    /// 文件大小（字节）
    pub size: Option<i64>,
    /// 父文件夹token
    pub parent_folder_token: Option<String>,
    /// 创建时间
    pub create_time: Option<String>,
    /// 修改时间
    pub modify_time: Option<String>,
    /// 创建者信息
    pub creator: Option<serde_json::Value>,
    /// 文件URL
    pub url: Option<String>,
    /// 文件预览URL
    pub preview_url: Option<String>,
}

/// 批量获取文件元数据响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchQueryMetaResponse {
    /// 文件元数据列表
    pub data: Option<Vec<FileMeta>>,
}

impl ApiResponseTrait for BatchQueryMetaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量获取文件元数据
///
/// 该接口用于根据 token 获取各类文件的元数据
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/file/batch_query
pub async fn batch_query_meta(
    request: BatchQueryMetaRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<BatchQueryMetaResponse>> {
    // 构建API端点
    let url = "/open-apis/drive/v1/metas/batch_query";

    // 创建API请求
    let mut api_request: ApiRequest<BatchQueryMetaResponse> =
        ApiRequest::post(url)
            .body(serde_json::to_value(request)?);

    // 如果有请求选项，应用它们
    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    // 发送请求
    Transport::request(api_request, config, None).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_query_meta_request() {
        let request = BatchQueryMetaRequest {
            file_tokens: vec!["token1".to_string(), "token2".to_string()],
        };

        assert_eq!(request.file_tokens.len(), 2);
        assert_eq!(request.file_tokens[0], "token1");
    }

    #[test]
    fn test_file_meta_structure() {
        let meta = FileMeta {
            file_token: "test_token".to_string(),
            name: "test_file.txt".to_string(),
            r#type: "file".to_string(),
            size: Some(1024),
            parent_folder_token: Some("parent_token".to_string()),
            create_time: Some("2023-01-01T00:00:00Z".to_string()),
            modify_time: Some("2023-01-02T00:00:00Z".to_string()),
            creator: None,
            url: None,
            preview_url: None,
        };

        assert_eq!(meta.file_token, "test_token");
        assert_eq!(meta.name, "test_file.txt");
        assert_eq!(meta.size, Some(1024));
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(BatchQueryMetaResponse::data_format(), ResponseFormat::Data);
    }
}