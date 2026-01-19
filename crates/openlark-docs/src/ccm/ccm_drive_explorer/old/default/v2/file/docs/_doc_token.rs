//! 删除 Doc
//!
//! docPath: /document/ukTMukTMukTM/uATM2UjLwEjN14CMxYTN
//! doc: https://open.feishu.cn/document/ukTMukTMukTM/uATM2UjLwEjN14CMxYTN

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::CcmDriveExplorerApiOld, api_utils::*};

/// 删除 Doc 响应（data）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDocResp {
    /// 被删除的对象 id
    pub id: String,
    /// 是否成功
    pub result: bool,
}

impl ApiResponseTrait for DeleteDocResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除 Doc 请求
pub struct DeleteDocRequest {
    config: Config,
    doc_token: String,
}

impl DeleteDocRequest {
    pub fn new(config: Config, doc_token: impl Into<String>) -> Self {
        Self {
            config,
            doc_token: doc_token.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<DeleteDocResp> {
            self.execute_with_options(openlark_core::req_option::RequestOption::default()).await
        }

        pub async fn execute_with_options(
            self,
            option: openlark_core::req_option::RequestOption,
        ) -> SDKResult<DeleteDocResp> {

        validate_required!(self.doc_token, "docToken 不能为空");

        let api_request: ApiRequest<DeleteDocResp> =
            ApiRequest::delete(&CcmDriveExplorerApiOld::FileDocs(self.doc_token).to_url());

        
            let response = Transport::request(api_request, &self.config, 
Some(option)).await?;
                extract_response_data(response, "操作")}
}
