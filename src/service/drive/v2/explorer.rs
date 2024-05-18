use crate::core::{
    api_req::ApiReq,
    api_resp::{ApiResponse, ApiResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

pub struct ExplorerService {
    config: Config,
}

impl ExplorerService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn meta_data(&self) -> SDKResult<ApiResponse<ExplorerMeta>> {
        let mut api_req = ApiReq::default();
        api_req.http_method = "GET".to_string();
        api_req.api_path = "/open-apis/drive/explorer/v2/root_folder/meta".to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];

        let api_resp = Transport::request(api_req, &self.config, None)?;

        Ok(api_resp)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExplorerMeta {
    /// 文件夹的 token
    pub token: String,
    /// 文件夹的 id
    pub id: String,
    /// 文件夹的所有者 id
    pub user_id: String,
}

impl ApiResponseFormat for ExplorerMeta {
    fn standard_data_format() -> bool {
        true
    }
}
