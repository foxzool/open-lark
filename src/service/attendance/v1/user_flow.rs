//! 用户打卡流水查询服务
//!
//! 提供查询用户打卡流水记录的功能，包括打卡时间、位置、方式等详细信息。

use crate::core::{
    api_req::ApiRequest, api_resp::BaseResponse, config::Config, constants::AccessTokenType,
    http::Transport, req_option::RequestOption, SDKResult,
};
use reqwest::Method;

use super::models::{UserFlowQueryRequest, UserFlowQueryResponse, UserFlowRecord};

/// 用户打卡流水服务
pub struct UserFlowService {
    config: Config,
}

impl UserFlowService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 查询用户打卡流水记录
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
    /// use chrono::Utc;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = LarkClient::builder("app_id", "app_secret").build();
    ///     let req = UserFlowQueryRequest::builder()
    ///         .user_ids(vec!["user_id_1".to_string()])
    ///         .check_time_from(Utc::now() - chrono::Duration::days(7))
    ///         .check_time_to(Utc::now())
    ///         .page_size(100)
    ///         .build();
    ///
    ///     let response = client.attendance.v1.user_flow.query(req, None).await?;
    ///     for record in response.records {
    ///         println!("用户 {} 在 {} 打卡", record.user_id, record.check_time);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn query(
        &self,
        req: UserFlowQueryRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<UserFlowQueryResponse> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: "/open-apis/attendance/v1/user_flows/query".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&req)?,
            ..Default::default()
        };

        let api_resp: BaseResponse<UserFlowQueryResponse> =
            Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp.data.unwrap_or_default())
    }

    /// 查询用户打卡流水记录（迭代器方式）
    ///
    /// 返回一个迭代器，自动处理分页查询。
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use chrono::Utc;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = LarkClient::builder("app_id", "app_secret").build();
    ///     let req = UserFlowQueryRequest::builder()
    ///         .user_ids(vec!["user_id_1".to_string()])
    ///         .check_time_from(Utc::now() - chrono::Duration::days(30))
    ///         .check_time_to(Utc::now())
    ///         .build();
    ///
    ///     let mut iter = client.attendance.v1.user_flow.query_iter(req);
    ///     while let Some(records) = iter.next_page().await? {
    ///         for record in records {
    ///             println!("打卡记录: {:?}", record);
    ///         }
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn query_iter(&self, req: UserFlowQueryRequest) -> UserFlowIterator {
        UserFlowIterator::new(self.config.clone(), req)
    }
}

/// 用户打卡流水迭代器
pub struct UserFlowIterator {
    config: Config,
    request: UserFlowQueryRequest,
    finished: bool,
}

impl UserFlowIterator {
    pub fn new(config: Config, request: UserFlowQueryRequest) -> Self {
        Self {
            config,
            request,
            finished: false,
        }
    }

    /// 获取下一页数据
    pub async fn next_page(&mut self) -> SDKResult<Option<Vec<UserFlowRecord>>> {
        if self.finished {
            return Ok(None);
        }

        let service = UserFlowService::new(self.config.clone());
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
    use chrono::Utc;

    #[test]
    fn test_user_flow_iterator_creation() {
        let config = Config::default();
        let req = UserFlowQueryRequest::builder()
            .user_ids(vec!["test_user".to_string()])
            .check_time_from(Utc::now() - chrono::Duration::days(7))
            .check_time_to(Utc::now())
            .build();

        let iter = UserFlowIterator::new(config, req);
        assert!(iter.has_more());
        assert!(!iter.finished);
    }

    #[test]
    fn test_user_flow_service_creation() {
        let config = Config::default();
        let service = UserFlowService::new(config);

        // 验证服务创建成功
        assert_eq!(
            std::mem::size_of_val(&service),
            std::mem::size_of::<Config>()
        );
    }
}
