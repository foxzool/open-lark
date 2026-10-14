//! 回写发布节点状态
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/spark-v1/app-release/update>
//!
//! 权限点 `spark:app:publish`。Authorization **仅** `tenant_access_token`。

use crate::common::api_endpoints::SparkApiV1;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    error::validation_error,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 发布节点裁决动作。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SparkReleaseNodeAction {
    /// 通过，流水线继续执行。
    Approve,
    /// 拒绝，终止本次发布。
    Reject,
    /// 撤回，终止本次发布。
    Cancel,
}

/// 回写到发布节点的审核结果。
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SparkReleaseNodeResult {
    /// 审核单详情页链接（HTTP/HTTPS，最长 2048 字符）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_url: Option<String>,
    /// 操作人标识（最长 128 字符）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    /// 审批意见（最长 1024 字符）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// 回写发布节点状态请求体。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSparkAppReleaseBody {
    /// 目标节点任务 ID（必填）。
    pub node_task_id: String,
    /// 裁决动作。不传则不改变节点状态。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<SparkReleaseNodeAction>,
    /// 回写结果。可与 `action` 同时传入。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<SparkReleaseNodeResult>,
}

impl UpdateSparkAppReleaseBody {
    /// 校验节点任务 ID、结果字段长度，以及 action / approval_url 至少其一。
    pub fn validate(&self) -> SDKResult<()> {
        validate_required!(self.node_task_id, "node_task_id 不能为空");
        let mut has_approval_url = false;
        if let Some(result) = &self.result {
            if let Some(url) = present(result.approval_url.as_ref()) {
                has_approval_url = true;
                check_max(url, 2048, "result.approval_url")?;
                let lower = url.to_ascii_lowercase();
                if !lower.starts_with("http://") && !lower.starts_with("https://") {
                    return Err(validation_error(
                        "result.approval_url",
                        "result.approval_url 必须是 HTTP 或 HTTPS URL",
                    ));
                }
            }
            if let Some(operator) = present(result.operator.as_ref()) {
                check_max(operator, 128, "result.operator")?;
            }
            if let Some(message) = present(result.message.as_ref()) {
                check_max(message, 1024, "result.message")?;
            }
        }
        if self.action.is_none() && !has_approval_url {
            return Err(validation_error(
                "action",
                "action 与 result.approval_url 至少传一个",
            ));
        }
        Ok(())
    }
}

/// 回写发布节点状态响应 `data`。
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateSparkAppReleaseResponse {
    /// 妙搭应用 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// 发布记录 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_id: Option<String>,
    /// 被操作的节点任务 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_task_id: Option<String>,
    /// 节点名，如 `build` / `deploy`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<String>,
    /// 节点当前状态。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_status: Option<String>,
    /// 已保存的回写结果。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<SparkReleaseNodeResult>,
}

impl ApiResponseTrait for UpdateSparkAppReleaseResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 回写发布节点状态请求。
#[derive(Debug, Clone)]
pub struct UpdateSparkAppReleaseRequest {
    config: Arc<Config>,
    app_id: String,
    release_id: String,
}

impl UpdateSparkAppReleaseRequest {
    /// 创建请求构建器。
    pub fn new(
        config: Arc<Config>,
        app_id: impl Into<String>,
        release_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            app_id: app_id.into(),
            release_id: release_id.into(),
        }
    }

    /// 执行请求。
    pub async fn execute(
        self,
        body: UpdateSparkAppReleaseBody,
    ) -> SDKResult<UpdateSparkAppReleaseResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: UpdateSparkAppReleaseBody,
        option: RequestOption,
    ) -> SDKResult<UpdateSparkAppReleaseResponse> {
        validate_required!(self.app_id, "app_id 不能为空");
        validate_required!(self.release_id, "release_id 不能为空");
        body.validate()?;
        let req: ApiRequest<UpdateSparkAppReleaseResponse> = ApiRequest::put(
            SparkApiV1::AppRelease(self.app_id.clone(), self.release_id.clone()).to_url(),
        )
        .body(crate::common::api_utils::serialize_params(
            &body,
            "回写发布节点状态",
        )?)
        .with_supported_access_token_types(vec![AccessTokenType::Tenant]);
        Transport::request_typed(req, &self.config, Some(option), "回写发布节点状态").await
    }
}

fn present(value: Option<&String>) -> Option<&str> {
    value
        .map(String::as_str)
        .map(str::trim)
        .filter(|text| !text.is_empty())
}

fn check_max(value: &str, max: usize, name: &str) -> SDKResult<()> {
    if value.chars().count() > max {
        return Err(validation_error(
            name,
            format!("{name} 不能超过 {max} 个字符"),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{body_json, header, method, path},
    };

    fn test_config(base_url: impl Into<String>) -> Arc<Config> {
        Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .base_url(base_url)
                .enable_token_cache(false)
                .build(),
        )
    }

    fn tenant_option() -> RequestOption {
        RequestOption::builder()
            .tenant_access_token("tenant-token")
            .build()
    }

    fn user_option() -> RequestOption {
        RequestOption::builder()
            .user_access_token("user-token")
            .build()
    }

    fn approve_body() -> UpdateSparkAppReleaseBody {
        UpdateSparkAppReleaseBody {
            node_task_id: "evt_1".into(),
            action: Some(SparkReleaseNodeAction::Approve),
            result: Some(SparkReleaseNodeResult {
                approval_url: Some("https://audit.example.com/tickets/10086".into()),
                operator: Some("ou_operator".into()),
                message: Some("安全扫描通过，同意发布".into()),
            }),
        }
    }

    #[test]
    fn release_action_serializes_lowercase() {
        assert_eq!(
            serde_json::to_value(SparkReleaseNodeAction::Approve).unwrap(),
            json!("approve")
        );
        assert_eq!(
            serde_json::to_value(SparkReleaseNodeAction::Reject).unwrap(),
            json!("reject")
        );
        assert_eq!(
            serde_json::to_value(SparkReleaseNodeAction::Cancel).unwrap(),
            json!("cancel")
        );
    }

    #[test]
    fn release_body_requires_action_or_approval_url() {
        let missing = UpdateSparkAppReleaseBody {
            node_task_id: "evt_1".into(),
            action: None,
            result: None,
        };
        assert!(
            missing
                .validate()
                .unwrap_err()
                .to_string()
                .contains("action")
        );

        let bad_url = UpdateSparkAppReleaseBody {
            node_task_id: "evt_1".into(),
            action: Some(SparkReleaseNodeAction::Approve),
            result: Some(SparkReleaseNodeResult {
                approval_url: Some("ftp://audit.example.com".into()),
                ..SparkReleaseNodeResult::default()
            }),
        };
        assert!(bad_url.validate().is_err());
    }

    #[tokio::test]
    async fn release_rejects_user_token() {
        let server = MockServer::start().await;
        let result = UpdateSparkAppReleaseRequest::new(test_config(server.uri()), "app_1", "rel_1")
            .execute_with_options(approve_body(), user_option())
            .await;
        assert!(result.unwrap_err().to_string().contains("tenant token"));
        assert!(
            server
                .received_requests()
                .await
                .unwrap_or_default()
                .is_empty()
        );
    }

    #[tokio::test]
    async fn release_puts_node_status() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/open-apis/spark/v1/apps/app_1/releases/rel_1"))
            .and(header("authorization", "Bearer tenant-token"))
            .and(body_json(json!({
                "node_task_id": "evt_1",
                "action": "approve",
                "result": {
                    "approval_url": "https://audit.example.com/tickets/10086",
                    "operator": "ou_operator",
                    "message": "安全扫描通过，同意发布"
                }
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "app_id": "app_1",
                    "release_id": "rel_1",
                    "node_task_id": "evt_1",
                    "node": "build",
                    "node_status": "APPROVED",
                    "result": {
                        "approval_url": "https://audit.example.com/tickets/10086",
                        "operator": "ou_operator",
                        "message": "安全扫描通过，同意发布"
                    }
                }
            })))
            .mount(&server)
            .await;

        let resp = UpdateSparkAppReleaseRequest::new(test_config(server.uri()), "app_1", "rel_1")
            .execute_with_options(approve_body(), tenant_option())
            .await
            .expect("回写发布节点应成功");
        assert_eq!(resp.app_id.as_deref(), Some("app_1"));
        assert_eq!(resp.node_status.as_deref(), Some("APPROVED"));
        assert_eq!(resp.node.as_deref(), Some("build"));
    }
}
