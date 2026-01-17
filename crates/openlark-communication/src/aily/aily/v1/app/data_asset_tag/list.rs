//! 获取数据知识分类列表
//!
//! docPath: https://open.feishu.cn/document/aily-v1/data-knowledge/data-knowledge-management/list-2

use std::collections::HashMap;

use crate::{common::api_utils::extract_response_data, endpoints::AILY_V1_DATA_ASSET_TAGS};
use openlark_core::validate_required;
use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

/// 获取数据知识分类列表请求
pub struct ListDataAssetTagsRequest {
    config: Config,
    app_id: String,
    query: HashMap<String, String>,
}

impl ListDataAssetTagsRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_id: String::new(),
            query: HashMap::new(),
        }
    }

    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = app_id.into();
        self
    }

    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query.insert(key.into(), value.into());
        self
    }

    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        validate_required!(self.app_id, "app_id 不能为空");

        let url = AILY_V1_DATA_ASSET_TAGS.replace("{app_id}", &self.app_id);
        let mut req: ApiRequest<serde_json::Value> = ApiRequest::get(&url);
        for (k, v) in self.query {
            req = req.query(k, v);
        }

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "获取数据知识分类列表")
    }
}
