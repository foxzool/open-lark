//! 解绑标签与群
//!
//! docPath: https://open.feishu.cn/document/tenant-tag/update

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{
    common::{
        api_utils::{extract_response_data, serialize_params},
        models::EmptyData,
    },
    endpoints::IM_V2_BIZ_ENTITY_TAG_RELATION,
    im::im::v2::biz_entity_tag_relation::models::BizEntityTagRelationBody,
};

/// 解绑标签与群请求
pub struct UpdateBizEntityTagRelationRequest {
    config: Config,
}

impl UpdateBizEntityTagRelationRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/tenant-tag/update
    pub async fn execute(self, body: BizEntityTagRelationBody) -> SDKResult<EmptyData> {
        validate_required!(body.tag_biz_type, "tag_biz_type 不能为空");
        validate_required!(body.biz_entity_id, "biz_entity_id 不能为空");

        // url: PUT:/open-apis/im/v2/biz_entity_tag_relation
        let req: ApiRequest<EmptyData> = ApiRequest::put(IM_V2_BIZ_ENTITY_TAG_RELATION)
            .body(serialize_params(&body, "解绑标签与群")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "解绑标签与群")
    }
}
