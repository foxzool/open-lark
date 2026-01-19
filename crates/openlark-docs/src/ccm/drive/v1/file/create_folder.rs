//! 新建文件夹
//!
//! 在用户云空间的指定文件夹中创建一个新的空文件夹。
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/folder/create_folder

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use crate::common::{api_endpoints::DriveApi, api_utils::*};
use serde::{Deserialize, Serialize};

/// 新建文件夹请求
#[derive(Debug, Clone, Serialize)]
pub struct CreateFolderRequest {
    #[serde(skip)]
    config: Config,
    /// 文件夹名称
    pub name: String,
    /// 父文件夹的 token。参数为空字符串时，表示在根目录下创建文件夹
    pub folder_token: String,
}

/// 新建文件夹响应（data）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFolderResponse {
    /// 新建的文件夹的 token
    pub token: String,
    /// 新建的文件夹的 URL 链接
    pub url: String,
}

impl ApiResponseTrait for CreateFolderResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CreateFolderRequest {
    pub fn new(config: Config, name: impl Into<String>, folder_token: impl Into<String>) -> Self {
        Self {
            config,
            name: name.into(),
            folder_token: folder_token.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<CreateFolderResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateFolderResponse> {
        let name_len = self.name.len();
        if !(1..=256).contains(&name_len) {
            return Err(openlark_core::error::validation_error(
                "name",
                "name 长度必须在 1~256 字节之间",
            ));
        }

        let api_endpoint = DriveApi::CreateFolder;
        let request = ApiRequest::<CreateFolderResponse>::post(&api_endpoint.to_url())
            .body(serialize_params(&self, "新建文件夹")?);

        let response = Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "新建文件夹")
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
        assert_eq!(request.folder_token, "parent_token");
    }
}
