//! 访客管理 API
//!
//! 提供访客的添加和删除功能。

use std::sync::Arc;

/// 访客管理服务
#[derive(Debug)]
pub struct VisitorsService {
    config: Arc<crate::models::SecurityConfig>,
}

impl VisitorsService {
    /// 创建新的访客管理服务实例
    pub fn new(config: Arc<crate::models::SecurityConfig>) -> Self {
        Self { config }
    }

    /// 添加访客
    pub fn create(&self) -> CreateVisitorBuilder {
        CreateVisitorBuilder {
            config: self.config.clone(),
            name: String::new(),
            mobile: String::new(),
            email: None,
            visit_reason: None,
            host_info: None,
            valid_from: 0,
            valid_until: 0,
            rule_ids: Vec::new(),
        }
    }

    /// 删除访客
    pub fn delete(&self) -> DeleteVisitorBuilder {
        DeleteVisitorBuilder {
            config: self.config.clone(),
            visitor_id: String::new(),
        }
    }
}

/// 添加访客构建器
#[derive(Debug)]
pub struct CreateVisitorBuilder {
    config: Arc<crate::models::SecurityConfig>,
    name: String,
    mobile: String,
    email: Option<String>,
    visit_reason: Option<String>,
    host_info: Option<crate::models::acs::HostInfo>,
    valid_from: crate::models::Timestamp,
    valid_until: crate::models::Timestamp,
    rule_ids: Vec<String>,
}

impl CreateVisitorBuilder {
    /// 设置访客姓名
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// 设置访客手机号
    pub fn mobile(mut self, mobile: impl Into<String>) -> Self {
        self.mobile = mobile.into();
        self
    }

    /// 设置访客邮箱
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    /// 设置拜访事由
    pub fn visit_reason(mut self, visit_reason: impl Into<String>) -> Self {
        self.visit_reason = Some(visit_reason.into());
        self
    }

    /// 设置被访人信息
    pub fn host_info(mut self, host_info: crate::models::acs::HostInfo) -> Self {
        self.host_info = Some(host_info);
        self
    }

    /// 设置访问权限开始时间
    pub fn valid_from(mut self, valid_from: crate::models::Timestamp) -> Self {
        self.valid_from = valid_from;
        self
    }

    /// 设置访问权限结束时间
    pub fn valid_until(mut self, valid_until: crate::models::Timestamp) -> Self {
        self.valid_until = valid_until;
        self
    }

    /// 设置权限组ID列表
    pub fn rule_ids(mut self, rule_ids: Vec<String>) -> Self {
        self.rule_ids = rule_ids;
        self
    }

    /// 设置访问权限时间范围（小时）
    pub fn valid_duration_hours(mut self, hours: i64) -> Self {
        let now = chrono::Utc::now().timestamp();
        self.valid_from = now;
        self.valid_until = now + hours * 3600;
        self
    }

    /// 发送请求添加访客
    pub async fn send(self) -> crate::SecurityResult<crate::models::acs::VisitorInfo> {
        let url = format!("{}/open-apis/acs/v1/visitors", self.config.base_url);

        let request_body = crate::models::acs::VisitorRequest {
            name: self.name,
            mobile: self.mobile,
            email: self.email,
            visit_reason: self.visit_reason,
            host_info: self.host_info,
            valid_from: self.valid_from,
            valid_until: self.valid_until,
            rule_ids: Some(self.rule_ids),
            extension: None,
        };

        let response = reqwest::Client::new()
            .post(&url)
            .header(
                "Authorization",
                format!("Bearer {}", get_app_token(&self.config).await?),
            )
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if response.status().is_success() {
            let api_response: crate::models::ApiResponse<crate::models::acs::VisitorInfo> =
                response.json().await?;
            match api_response.data {
                Some(visitor) => Ok(visitor),
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

/// 删除访客构建器
#[derive(Debug)]
pub struct DeleteVisitorBuilder {
    config: Arc<crate::models::SecurityConfig>,
    visitor_id: String,
}

impl DeleteVisitorBuilder {
    /// 设置访客ID
    pub fn visitor_id(mut self, visitor_id: impl Into<String>) -> Self {
        self.visitor_id = visitor_id.into();
        self
    }

    /// 发送请求删除访客
    pub async fn send(self) -> crate::SecurityResult<crate::models::OperationResponse> {
        let url = format!(
            "{}/open-apis/acs/v1/visitors/{}",
            self.config.base_url, self.visitor_id
        );

        let response = reqwest::Client::new()
            .delete(&url)
            .header(
                "Authorization",
                format!("Bearer {}", get_app_token(&self.config).await?),
            )
            .header("Content-Type", "application/json")
            .send()
            .await?;

        if response.status().is_success() {
            let api_response: crate::models::ApiResponse<crate::models::OperationResponse> =
                response.json().await?;
            match api_response.data {
                Some(result) => Ok(result),
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
