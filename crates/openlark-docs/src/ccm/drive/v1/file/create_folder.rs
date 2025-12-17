/// 新建文件夹
///
/// 在用户云空间的指定文件夹中创建一个新的空文件夹。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/folder/create_folder
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

// 导入序列化支持
use crate::common::api_endpoints::DriveApi;
use serde::{Deserialize, Serialize};

/// 新建文件夹请求
#[derive(Debug)]
pub struct CreateFolderRequest {
    config: Config,
    /// 文件夹名称
    pub name: String,
    /// 父文件夹token
    pub parent_folder_token: String,
}

/// 文件夹信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderInfo {
    /// 文件夹token
    pub file_token: String,
    /// 文件夹名称
    pub name: String,
    /// 文件夹类型
    pub r#type: String,
    /// 创建时间
    pub created_time: String,
    /// 修改时间
    pub modified_time: String,
    /// 父文件夹token
    pub parent_folder_token: String,
}

/// 新建文件夹响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFolderResponse {
    /// 文件夹信息
    pub data: Option<FolderInfo>,
}

impl ApiResponseTrait for CreateFolderResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CreateFolderRequest {
    pub fn new(
        config: Config,
        name: impl Into<String>,
        parent_folder_token: impl Into<String>,
    ) -> Self {
        Self {
            config,
            name: name.into(),
            parent_folder_token: parent_folder_token.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<Response<CreateFolderResponse>> {
        let api_endpoint = DriveApi::CreateFolder;
        let mut request = ApiRequest::<CreateFolderResponse>::post(&api_endpoint.to_url());

        let mut body = serde_json::Map::new();
        body.insert(
            "name".to_string(),
            serde_json::Value::String(self.name.clone()),
        );
        body.insert(
            "folder_token".to_string(),
            serde_json::Value::String(self.parent_folder_token.clone()),
        );

        request = request.json_body(&body);

        Transport::request(request, &self.config, None).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_folder_request() {
        let config = Config::default();
        let request = CreateFolderRequest::new(config, "test_folder", "parent_token");
        assert_eq!(request.name, "test_folder");
        assert_eq!(request.parent_folder_token, "parent_token");
    }
}
