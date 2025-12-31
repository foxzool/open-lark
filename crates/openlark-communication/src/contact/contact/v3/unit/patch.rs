//! 修改单位信息
//!
//! docPath: https://open.feishu.cn/document/server-docs/contact-v3/unit/patch

use openlark_core::{
    api::ApiRequest,
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::{
        api_utils::{extract_response_data, serialize_params},
        models::EmptyData,
    },
    endpoints::CONTACT_V3_UNIT,
};

/// 修改单位信息请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchUnitBody {
    /// 单位名字
    ///
    /// 注意：文档中标注“必填：否”，但实际请求该字段必填。
    pub name: String,
}

/// 修改单位信息请求
pub struct PatchUnitRequest {
    config: Config,
    unit_id: String,
}

impl PatchUnitRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            unit_id: String::new(),
        }
    }

    /// 单位 ID（路径参数）
    pub fn unit_id(mut self, unit_id: impl Into<String>) -> Self {
        self.unit_id = unit_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/contact-v3/unit/patch
    pub async fn execute(self, body: PatchUnitBody) -> SDKResult<EmptyData> {
        validate_required!(self.unit_id, "unit_id 不能为空");
        validate_required!(body.name, "name 不能为空");

        // url: PATCH:/open-apis/contact/v3/unit/:unit_id
        let req: ApiRequest<EmptyData> =
            ApiRequest::patch(format!("{}/{}", CONTACT_V3_UNIT, self.unit_id))
                .body(serialize_params(&body, "修改单位信息")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "修改单位信息")
    }
}

