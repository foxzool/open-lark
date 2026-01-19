//! 获取文件夹下的文件清单
//!
//! docPath: /document/ukTMukTMukTM/uEjNzUjLxYzM14SM2MTN
//! doc: https://open.feishu.cn/document/ukTMukTMukTM/uEjNzUjLxYzM14SM2MTN

use std::collections::HashMap;

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::CcmDriveExplorerApiOld, api_utils::*};

/// 获取文件夹下的文件清单响应（data）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFolderChildrenResp {
    /// 父文件夹 token
    #[serde(rename = "parentToken")]
    pub parent_token: String,
    /// 子项集合（key 为 token，value 为子项信息）
    pub children: HashMap<String, FolderChild>,
}

/// 子项信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderChild {
    /// token
    pub token: String,
    /// 名称
    pub name: String,
    /// 类型
    #[serde(rename = "type")]
    pub type_: String,
    /// 是否为快捷方式
    pub is_shortcut: bool,
}

impl ApiResponseTrait for GetFolderChildrenResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取文件夹下的文件清单请求
pub struct GetFolderChildrenRequest {
    config: Config,
    folder_token: String,
    types: Vec<String>,
}

impl GetFolderChildrenRequest {
    pub fn new(config: Config, folder_token: impl Into<String>) -> Self {
        Self {
            config,
            folder_token: folder_token.into(),
            types: Vec::new(),
        }
    }

    /// 追加过滤类型（query: types=doc&types=sheet...）
    pub fn push_type(mut self, type_: impl Into<String>) -> Self {
        self.types.push(type_.into());
        self
    }

    /// 批量设置过滤类型（query: types=...）
    pub fn types(mut self, types: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.types = types.into_iter().map(Into::into).collect();
        self
    }

    pub async fn execute(self) -> SDKResult<GetFolderChildrenResp> {
            self.execute_with_options(openlark_core::req_option::RequestOption::default()).await
        }

        pub async fn execute_with_options(
            self,
            option: openlark_core::req_option::RequestOption,
        ) -> SDKResult<GetFolderChildrenResp> {

        validate_required!(self.folder_token, "folderToken 不能为空");

        let mut api_request: ApiRequest<GetFolderChildrenResp> =
            ApiRequest::get(&CcmDriveExplorerApiOld::FolderChildren(self.folder_token).to_url());

        for type_ in &self.types {
            api_request = api_request.query("types", type_);
        }

        
            let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "子文件夹")
        }
}
