use crate::service::HrService;
use openlark_core::SDKResult;
use reqwest::Method;
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct ExternalReferralReward {
    service: Arc<HrService>,
}

impl ExternalReferralReward {
    pub fn new(service: Arc<HrService>) -> Self {
        Self { service }
    }

    /// 文档参考: https://open.feishu.cn/document/hire-v1/get-candidates/import-external-system-information/import-external-referral-reward-info/create
    pub async fn post_open_apis_hire_v1_external_referral_rewards(
        &self,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path = "/open-apis/hire/v1/external_referral_rewards".to_string();
        let method = Method::POST;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }

    /// 文档参考: https://open.feishu.cn/document/hire-v1/get-candidates/import-external-system-information/import-external-referral-reward-info/delete
    pub async fn delete_open_apis_hire_v1_external_referral_rewards_by_external_referral_reward_id(
        &self,
        external_referral_reward_id: impl AsRef<str>,
        payload: Option<&Value>,
    ) -> SDKResult<Value> {
        let mut path =
            "/open-apis/hire/v1/external_referral_rewards/:external_referral_reward_id".to_string();
        path = path.replace(
            ":external_referral_reward_id",
            external_referral_reward_id.as_ref(),
        );
        let method = Method::DELETE;
        let (query, body) = match method {
            Method::GET | Method::DELETE => (payload, None),
            _ => (None, payload),
        };
        self.service.request_value(method, &path, query, body).await
    }
}
