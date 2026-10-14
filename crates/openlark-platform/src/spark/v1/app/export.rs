//! 导出妙搭应用代码包
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/spark-v1/app/export>
//!
//! 权限点 `spark:app:read`。Authorization 为 tenant 或 user（默认即可）。
//! 成功响应是 zip 二进制流（`Content-Type: application/octet-stream`），不是 JSON envelope。

use crate::common::api_endpoints::SparkApiV1;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::validation_error,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 导出妙搭应用代码包请求体。
///
/// `app_id` 与 `meta_token` 必须且只能传一个。`checkpoint_id` 与 `release_id` 不能同时传入。
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportSparkAppBody {
    /// 妙搭应用 ID（`app_` 前缀）。与 `meta_token` 互斥。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// 创意应用分享链接 token。与 `app_id` 互斥。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta_token: Option<String>,
    /// 检查点 ID（大于等于 0 的整数字符串）。与 `release_id` 互斥。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpoint_id: Option<String>,
    /// 发布记录 ID（大于 0 的整数字符串）。与 `checkpoint_id` 互斥。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_id: Option<String>,
}

impl ExportSparkAppBody {
    /// 校验互斥字段、长度与整数字段。
    pub fn validate(&self) -> SDKResult<()> {
        let app_id = present(&self.app_id);
        let meta_token = present(&self.meta_token);
        if app_id.is_some() == meta_token.is_some() {
            return Err(validation_error(
                "app_id",
                "app_id 与 meta_token 必须且只能传一个",
            ));
        }
        if let Some(value) = app_id {
            check_max(value, 100, "app_id")?;
        }
        if let Some(value) = meta_token {
            check_max(value, 200, "meta_token")?;
        }

        let checkpoint_id = present(&self.checkpoint_id);
        let release_id = present(&self.release_id);
        if checkpoint_id.is_some() && release_id.is_some() {
            return Err(validation_error(
                "checkpoint_id",
                "checkpoint_id 与 release_id 不能同时传入",
            ));
        }
        if let Some(value) = checkpoint_id {
            match value.parse::<u64>() {
                Ok(_) => {}
                Err(_) => {
                    return Err(validation_error(
                        "checkpoint_id",
                        "checkpoint_id 必须是大于等于 0 的整数",
                    ));
                }
            }
        }
        if let Some(value) = release_id {
            match value.parse::<u64>() {
                Ok(parsed) if parsed > 0 => {}
                _ => {
                    return Err(validation_error(
                        "release_id",
                        "release_id 必须是大于 0 的整数",
                    ));
                }
            }
        }
        Ok(())
    }
}

/// 导出得到的 zip 代码包。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportSparkAppPackage {
    /// `Content-Disposition` 中的文件名；响应头缺失时为空字符串。
    pub file_name: String,
    /// zip 二进制内容。
    pub content: Vec<u8>,
}

impl ApiResponseTrait for ExportSparkAppPackage {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Binary
    }

    fn from_binary(file_name: String, body: Vec<u8>) -> Option<Self> {
        Some(Self {
            file_name,
            content: body,
        })
    }
}

/// 导出妙搭应用代码包请求。
#[derive(Debug, Clone)]
pub struct ExportSparkAppRequest {
    config: Arc<Config>,
}

impl ExportSparkAppRequest {
    /// 创建请求构建器。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行请求。
    pub async fn execute(self, body: ExportSparkAppBody) -> SDKResult<ExportSparkAppPackage> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: ExportSparkAppBody,
        option: RequestOption,
    ) -> SDKResult<ExportSparkAppPackage> {
        body.validate()?;
        let req: ApiRequest<ExportSparkAppPackage> =
            ApiRequest::post(SparkApiV1::AppsExport.to_url()).body(
                crate::common::api_utils::serialize_params(&body, "导出妙搭应用代码包")?,
            );
        Transport::request_typed(req, &self.config, Some(option), "导出妙搭应用代码包").await
    }
}

fn present(value: &Option<String>) -> Option<&str> {
    value
        .as_deref()
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

    #[test]
    fn export_body_omits_absent_fields() {
        let body = ExportSparkAppBody {
            app_id: Some("app_x".into()),
            meta_token: None,
            checkpoint_id: None,
            release_id: Some("7687553819701775556".into()),
        };
        assert_eq!(
            serde_json::to_value(&body).unwrap(),
            json!({
                "app_id": "app_x",
                "release_id": "7687553819701775556"
            })
        );
    }

    #[test]
    fn export_body_rejects_identity_and_locator_conflicts() {
        let both_ids = ExportSparkAppBody {
            app_id: Some("app_x".into()),
            meta_token: Some("token".into()),
            ..ExportSparkAppBody::default()
        };
        assert!(
            both_ids
                .validate()
                .unwrap_err()
                .to_string()
                .contains("app_id")
        );

        let neither = ExportSparkAppBody::default();
        assert!(neither.validate().is_err());

        let both_locators = ExportSparkAppBody {
            app_id: Some("app_x".into()),
            checkpoint_id: Some("0".into()),
            release_id: Some("1".into()),
            ..ExportSparkAppBody::default()
        };
        assert!(
            both_locators
                .validate()
                .unwrap_err()
                .to_string()
                .contains("checkpoint_id")
        );

        let zero_release = ExportSparkAppBody {
            app_id: Some("app_x".into()),
            release_id: Some("0".into()),
            ..ExportSparkAppBody::default()
        };
        assert!(zero_release.validate().is_err());
    }

    #[tokio::test]
    async fn export_posts_zip_package() {
        let server = MockServer::start().await;
        let zip = b"PK\x03\x04spark-export".to_vec();
        Mock::given(method("POST"))
            .and(path("/open-apis/spark/v1/apps/export"))
            .and(header("authorization", "Bearer tenant-token"))
            .and(body_json(json!({
                "app_id": "app_x",
                "release_id": "7687553819701775556"
            })))
            .respond_with(
                ResponseTemplate::new(200)
                    .insert_header("content-disposition", "attachment; filename=\"app_x.zip\"")
                    .set_body_bytes(zip.clone()),
            )
            .mount(&server)
            .await;

        let package = ExportSparkAppRequest::new(test_config(server.uri()))
            .execute_with_options(
                ExportSparkAppBody {
                    app_id: Some("app_x".into()),
                    release_id: Some("7687553819701775556".into()),
                    ..ExportSparkAppBody::default()
                },
                tenant_option(),
            )
            .await
            .expect("导出代码包应成功");
        assert_eq!(package.file_name, "app_x.zip");
        assert_eq!(package.content, zip);
    }

    #[tokio::test]
    async fn export_validation_does_not_call_server() {
        let server = MockServer::start().await;
        let result = ExportSparkAppRequest::new(test_config(server.uri()))
            .execute(ExportSparkAppBody::default())
            .await;
        assert!(result.is_err());
        assert!(
            server
                .received_requests()
                .await
                .unwrap_or_default()
                .is_empty()
        );
    }
}
