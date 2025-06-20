//! 用户考勤任务查询服务
//!
//! 提供查询用户考勤记录的功能，包括打卡结果、异常信息等。

use crate::core::{
    api_req::ApiRequest, api_resp::BaseResponse, config::Config, constants::AccessTokenType,
    http::Transport, req_option::RequestOption, SDKResult,
};
use reqwest::Method;

use super::models::{UserTaskQueryRequest, UserTaskQueryResponse, UserTaskRecord};

/// 用户考勤任务服务
pub struct UserTaskService {
    config: Config,
}

impl UserTaskService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 查询用户考勤记录
    ///
    /// # 权限要求
    /// - `attendance:readonly` - 基础考勤数据读取权限
    /// - `attendance:user.read` - 用户考勤数据访问权限
    ///
    /// # 参数
    /// - `req` - 查询请求参数
    /// - `option` - 请求选项
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use chrono::NaiveDate;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = LarkClient::builder("app_id", "app_secret").build();
    ///     let req = UserTaskQueryRequest::builder()
    ///         .user_ids(vec!["user_id_1".to_string()])
    ///         .check_date_from(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap())
    ///         .check_date_to(NaiveDate::from_ymd_opt(2025, 1, 31).unwrap())
    ///         .need_absent_info(true)
    ///         .build();
    ///
    ///     let response = client.attendance.v1.user_task.query(req, None).await?;
    ///     for record in response.records {
    ///         println!("用户 {} 在 {} 的考勤记录", record.user_id, record.date);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn query(
        &self,
        req: UserTaskQueryRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<UserTaskQueryResponse> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: "/open-apis/attendance/v1/user_daily_shifts/query".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&req)?,
            ..Default::default()
        };

        let api_resp: BaseResponse<UserTaskQueryResponse> =
            Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp.data.unwrap_or_default())
    }

    /// 查询用户考勤记录（迭代器方式）
    ///
    /// 返回一个迭代器，自动处理分页查询。
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use chrono::NaiveDate;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = LarkClient::builder("app_id", "app_secret").build();
    ///     let req = UserTaskQueryRequest::builder()
    ///         .user_ids(vec!["user_id_1".to_string()])
    ///         .check_date_from(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap())
    ///         .check_date_to(NaiveDate::from_ymd_opt(2025, 1, 31).unwrap())
    ///         .build();
    ///
    ///     let mut iter = client.attendance.v1.user_task.query_iter(req);
    ///     while let Some(records) = iter.next_page().await? {
    ///         for record in records {
    ///             println!("考勤记录: {:?}", record);
    ///         }
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn query_iter(&self, req: UserTaskQueryRequest) -> UserTaskIterator {
        UserTaskIterator::new(self.config.clone(), req)
    }
}

/// 用户考勤任务迭代器
pub struct UserTaskIterator {
    config: Config,
    request: UserTaskQueryRequest,
    finished: bool,
}

impl UserTaskIterator {
    pub fn new(config: Config, request: UserTaskQueryRequest) -> Self {
        Self {
            config,
            request,
            finished: false,
        }
    }

    /// 获取下一页数据
    pub async fn next_page(&mut self) -> SDKResult<Option<Vec<UserTaskRecord>>> {
        if self.finished {
            return Ok(None);
        }

        let service = UserTaskService::new(self.config.clone());
        let response = service.query(self.request.clone(), None).await?;

        // 更新分页信息
        if let Some(page_token) = response.page_token {
            self.request.page_token = Some(page_token);
        }
        self.finished = !response.has_more;

        Ok(Some(response.records))
    }

    /// 检查是否还有更多数据
    pub fn has_more(&self) -> bool {
        !self.finished
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_user_task_iterator_creation() {
        let config = Config::default();
        let req = UserTaskQueryRequest::builder()
            .user_ids(vec!["test_user".to_string()])
            .check_date_from(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap())
            .check_date_to(NaiveDate::from_ymd_opt(2025, 1, 31).unwrap())
            .build();

        let iter = UserTaskIterator::new(config, req);
        assert!(iter.has_more());
        assert!(!iter.finished);
    }

    #[test]
    fn test_user_task_service_creation() {
        let config = Config::default();
        let service = UserTaskService::new(config);

        // 验证服务创建成功
        assert_eq!(
            std::mem::size_of_val(&service),
            std::mem::size_of::<Config>()
        );
    }
}
