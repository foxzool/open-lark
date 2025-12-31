//! 获取旧版文档元信息
//!
//! docPath: /document/ukTMukTMukTM/uczN3UjL3czN14yN3cTN
//! doc: https://open.feishu.cn/document/ukTMukTMukTM/uczN3UjL3czN14yN3cTN

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::*;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetDocMetaReq {}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetDocMetaResponse {
    /// 创建日期
    pub create_date: String,
    /// 创建时间戳
    pub create_time: i64,
    /// 创建者 open_id
    pub creator: String,
    /// 创建者用户名
    pub create_user_name: String,
    /// 删除标志：0正常、1回收站、2彻底删除
    pub delete_flag: i32,
    /// 最后编辑时间戳
    pub edit_time: i64,
    /// 最后编辑者用户名
    pub edit_user_name: String,
    /// 是否外部文档
    #[serde(default)]
    pub is_external: bool,
    /// 是否置顶（快速访问）
    #[serde(default)]
    pub is_pined: bool,
    /// 是否收藏
    #[serde(default)]
    pub is_stared: bool,
    /// 是否已升级为新版文档（docx）
    #[serde(default)]
    pub is_upgraded: bool,
    /// 文档类型（示例：doc）
    #[serde(default)]
    pub obj_type: String,
    /// 当前所有者 open_id
    #[serde(default)]
    pub owner: String,
    /// 当前所有者用户名
    pub owner_user_name: String,
    /// 处理请求时服务器时间戳
    pub server_time: i64,
    /// 文档所在租户 id
    pub tenant_id: String,
    /// 文档名称
    pub title: String,
    /// 文档类型（固定 2）
    #[serde(rename = "type")]
    pub type_: i32,
    /// 升级后的文档 token（新版文档 document_id）
    #[serde(default)]
    pub upgraded_token: String,
    /// 文档 URL
    pub url: String,
}

impl ApiResponseTrait for GetDocMetaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取旧版文档元信息请求
pub struct GetDocMetaRequest {
    config: Config,
    doc_token: String,
}

impl GetDocMetaRequest {
    pub fn new(config: Config, doc_token: impl Into<String>) -> Self {
        Self {
            config,
            doc_token: doc_token.into(),
        }
    }

    pub async fn send(self) -> SDKResult<GetDocMetaResponse> {
        use crate::common::api_endpoints::CcmDocApiOld;
        validate_required!(self.doc_token, "doc_token 不能为空");

        let api_request: ApiRequest<GetDocMetaResponse> =
            ApiRequest::get(&CcmDocApiOld::Meta(self.doc_token).to_url());
        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "获取旧版文档元信息")
    }
}
