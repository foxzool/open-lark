//! OpenAPI 审计日志 API
//!
//! 提供API调用审计日志的查询功能。

use std::sync::Arc;

/// OpenAPI 审计日志服务
#[derive(Debug)]
pub struct OpenApiLogsService {
    config: Arc<crate::models::SecurityConfig>,
}

impl OpenApiLogsService {
    /// 创建新的审计日志服务实例
    pub fn new(config: Arc<crate::models::SecurityConfig>) -> Self {
        Self { config }
    }

    /// 获取审计日志数据
    pub fn list_data(&self) -> ListOpenApiLogsBuilder {
        ListOpenApiLogsBuilder {
            config: self.config.clone(),
            start_time: None,
            end_time: None,
            user_id_filter: None,
            api_path_filter: None,
            status_code_filter: None,
            app_id_filter: None,
            page_size: Some(20),
            page_token: None,
            sort_field: Some("request_time".to_string()),
            sort_direction: Some("desc".to_string()),
        }
    }
}

/// 获取审计日志数据构建器
#[derive(Debug)]
pub struct ListOpenApiLogsBuilder {
    config: Arc<crate::models::SecurityConfig>,
    start_time: Option<crate::models::Timestamp>,
    end_time: Option<crate::models::Timestamp>,
    user_id_filter: Option<String>,
    api_path_filter: Option<String>,
    status_code_filter: Option<i32>,
    app_id_filter: Option<String>,
    page_size: Option<i32>,
    page_token: Option<String>,
    sort_field: Option<String>,
    sort_direction: Option<String>,
}

impl ListOpenApiLogsBuilder {
    /// 设置开始时间
    pub fn start_time(mut self, start_time: crate::models::Timestamp) -> Self {
        self.start_time = Some(start_time);
        self
    }

    /// 设置结束时间
    pub fn end_time(mut self, end_time: crate::models::Timestamp) -> Self {
        self.end_time = Some(end_time);
        self
    }

    /// 设置时间范围
    pub fn time_range(
        mut self,
        start_time: crate::models::Timestamp,
        end_time: crate::models::Timestamp,
    ) -> Self {
        self.start_time = Some(start_time);
        self.end_time = Some(end_time);
        self
    }

    /// 设置最近N天的日志
    pub fn last_days(mut self, days: i64) -> Self {
        use chrono::Utc;
        let now = Utc::now().timestamp();
        let start_time = now - days * 24 * 3600;
        self.start_time = Some(start_time);
        self.end_time = Some(now);
        self
    }

    /// 设置最近N小时的日志
    pub fn last_hours(mut self, hours: i64) -> Self {
        use chrono::Utc;
        let now = Utc::now().timestamp();
        let start_time = now - hours * 3600;
        self.start_time = Some(start_time);
        self.end_time = Some(now);
        self
    }

    /// 设置用户ID过滤
    pub fn user_id_filter(mut self, user_id: impl Into<String>) -> Self {
        self.user_id_filter = Some(user_id.into());
        self
    }

    /// 设置API路径过滤
    pub fn api_path_filter(mut self, api_path: impl Into<String>) -> Self {
        self.api_path_filter = Some(api_path.into());
        self
    }

    /// 设置API路径模式过滤
    pub fn api_path_contains(mut self, pattern: impl Into<String>) -> Self {
        self.api_path_filter = Some(format!("*{}*", pattern.into()));
        self
    }

    /// 设置状态码过滤
    pub fn status_code_filter(mut self, status_code: i32) -> Self {
        self.status_code_filter = Some(status_code);
        self
    }

    /// 设置应用ID过滤
    pub fn app_id_filter(mut self, app_id: impl Into<String>) -> Self {
        self.app_id_filter = Some(app_id.into());
        self
    }

    /// 设置页面大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置排序字段
    pub fn sort_field(mut self, sort_field: impl Into<String>) -> Self {
        self.sort_field = Some(sort_field.into());
        self
    }

    /// 设置排序方向
    pub fn sort_direction(mut self, sort_direction: impl Into<String>) -> Self {
        self.sort_direction = Some(sort_direction.into());
        self
    }

    /// 设置按时间升序排列
    pub fn sort_by_time_asc(mut self) -> Self {
        self.sort_field = Some("request_time".to_string());
        self.sort_direction = Some("asc".to_string());
        self
    }

    /// 设置按时间降序排列
    pub fn sort_by_time_desc(mut self) -> Self {
        self.sort_field = Some("request_time".to_string());
        self.sort_direction = Some("desc".to_string());
        self
    }

    /// 发送请求获取审计日志数据
    pub async fn send(
        self,
    ) -> crate::SecurityResult<
        crate::models::PageResponse<crate::models::security_and_compliance::OpenApiLog>,
    > {
        let url = format!(
            "{}/open-apis/security_and_compliance/v1/openapi_logs/list_data",
            self.config.base_url
        );

        let mut request_body = serde_json::Map::new();

        if let Some(start_time) = self.start_time {
            request_body.insert(
                "start_time".to_string(),
                serde_json::Value::Number(start_time.into()),
            );
        }
        if let Some(end_time) = self.end_time {
            request_body.insert(
                "end_time".to_string(),
                serde_json::Value::Number(end_time.into()),
            );
        }
        if let Some(user_id_filter) = &self.user_id_filter {
            request_body.insert(
                "user_id_filter".to_string(),
                serde_json::Value::String(user_id_filter.clone()),
            );
        }
        if let Some(api_path_filter) = &self.api_path_filter {
            request_body.insert(
                "api_path_filter".to_string(),
                serde_json::Value::String(api_path_filter.clone()),
            );
        }
        if let Some(status_code_filter) = self.status_code_filter {
            request_body.insert(
                "status_code_filter".to_string(),
                serde_json::Value::Number(status_code_filter.into()),
            );
        }
        if let Some(app_id_filter) = &self.app_id_filter {
            request_body.insert(
                "app_id_filter".to_string(),
                serde_json::Value::String(app_id_filter.clone()),
            );
        }
        if let Some(page_size) = self.page_size {
            request_body.insert(
                "page_size".to_string(),
                serde_json::Value::Number(page_size.into()),
            );
        }
        if let Some(page_token) = &self.page_token {
            request_body.insert(
                "page_token".to_string(),
                serde_json::Value::String(page_token.clone()),
            );
        }
        if let Some(sort_field) = &self.sort_field {
            request_body.insert(
                "sort_field".to_string(),
                serde_json::Value::String(sort_field.clone()),
            );
        }
        if let Some(sort_direction) = &self.sort_direction {
            request_body.insert(
                "sort_direction".to_string(),
                serde_json::Value::String(sort_direction.clone()),
            );
        }

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
            let api_response: crate::models::ApiResponse<
                crate::models::PageResponse<crate::models::security_and_compliance::OpenApiLog>,
            > = response.json().await?;
            match api_response.data {
                Some(logs) => Ok(logs),
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
