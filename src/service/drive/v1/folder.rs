use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

/// 文件夹服务
pub struct FolderService {
    config: Config,
}

impl FolderService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取我的空间（root folder）元数据
    ///
    /// 该接口用于根据用户的访问凭证获取用户的根目录信息，包括根目录的token等。
    ///
    /// <https://open.feishu.cn/document/server-docs/docs/drive-v1/folder/get-root-folder-meta>
    pub async fn get_root_folder_meta(
        &self,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetRootFolderMetaRespData>> {
        let mut api_req = ApiRequest::default();
        api_req.http_method = Method::GET;
        api_req.api_path = "/open-apis/drive/v1/folders/root_folder_meta".to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

/// 获取我的空间（root folder）元数据响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRootFolderMetaRespData {
    /// 用户空间的根目录 token
    pub token: String,
    /// 用户 ID
    pub user_id: String,
}

impl ApiResponseTrait for GetRootFolderMetaRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
