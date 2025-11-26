//! 设备申报审批 API
//!
//! 提供设备申报的审批功能。

use std::sync::Arc;

/// 设备申报审批服务
#[derive(Debug)]
pub struct DeviceApplyRecordsService {
    config: Arc<crate::models::SecurityConfig>,
}

impl DeviceApplyRecordsService {
    /// 创建新的设备申报审批服务实例
    pub fn new(config: Arc<crate::models::SecurityConfig>) -> Self {
        Self { config }
    }

    /// 审批设备申报
    pub fn approve(&self) -> ApproveDeviceApplyRecordBuilder {
        ApproveDeviceApplyRecordBuilder {
            config: self.config.clone(),
            device_apply_record_id: String::new(),
            approved: false,
            comment: None,
            remark: None,
        }
    }
}

/// 审批设备申报构建器
#[derive(Debug)]
pub struct ApproveDeviceApplyRecordBuilder {
    config: Arc<crate::models::SecurityConfig>,
    device_apply_record_id: String,
    approved: bool,
    comment: Option<String>,
    remark: Option<String>,
}

impl ApproveDeviceApplyRecordBuilder {
    /// 设置申报记录ID
    pub fn device_apply_record_id(mut self, device_apply_record_id: impl Into<String>) -> Self {
        self.device_apply_record_id = device_apply_record_id.into();
        self
    }

    /// 设置是否批准
    pub fn approved(mut self, approved: bool) -> Self {
        self.approved = approved;
        self
    }

    /// 设置审批意见
    pub fn comment(mut self, comment: impl Into<String>) -> Self {
        self.comment = Some(comment.into());
        self
    }

    /// 设置审批备注
    pub fn remark(mut self, remark: impl Into<String>) -> Self {
        self.remark = Some(remark.into());
        self
    }

    /// 快速批准
    pub fn approve(mut self) -> Self {
        self.approved = true;
        self
    }

    /// 快速拒绝
    pub fn reject(mut self) -> Self {
        self.approved = false;
        self
    }

    /// 发送请求审批设备申报
    pub async fn send(
        self,
    ) -> crate::SecurityResult<crate::models::security_and_compliance::DeviceApplyRecord> {
        let url = format!(
            "{}/open-apis/security_and_compliance/v2/device_apply_records/{}",
            self.config.base_url, self.device_apply_record_id
        );

        let request_body =
            crate::models::security_and_compliance::DeviceApplyRecordApproveRequest {
                approved: self.approved,
                comment: self.comment,
                remark: self.remark,
            };

        let response = reqwest::Client::new()
            .put(&url)
            .header(
                "Authorization",
                format!("Bearer {}", get_app_token(&self.config).await?),
            )
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if response.status().is_success() {
            let api_response: crate::models::ApiResponse<
                crate::models::security_and_compliance::DeviceApplyRecord,
            > = response.json().await?;
            match api_response.data {
                Some(record) => Ok(record),
                None => Err(crate::SecurityError::APIError {
                    code: api_response.code,
                    message: api_response.msg,
                }),
            }
        } else {
            Err(crate::SecurityError::APIError {
                code: response.status().as_u16() as i32,
                message: format!(
                    "HTTP {}: {}",
                    response.status(),
                    response.text().await.unwrap_or_default()
                ),
            })
        }
    }
}

/// 获取应用访问令牌的辅助函数
async fn get_app_token(_config: &crate::models::SecurityConfig) -> crate::SecurityResult<String> {
    // 这里应该调用认证服务获取应用访问令牌
    // 为了简化实现，暂时返回占位符
    // TODO: 集成 openlark-auth 服务
    Ok("placeholder_app_token".to_string())
}
