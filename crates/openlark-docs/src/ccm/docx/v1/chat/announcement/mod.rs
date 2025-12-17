#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]

/// 聊天公告模块
///
/// 提供聊天公告的管理功能，包括公告获取、区块管理等。
use openlark_core::{
    api::Response, config::Config, error::validation_error, req_option::RequestOption, SDKResult,
};

// 重新导出所有模块类型
pub use block::*;
pub use get::*;

// 子模块
mod block;
mod get; // 群公告区块管理

/// 聊天公告服务
///
/// 提供聊天公告的完整管理功能，包括公告获取、区块管理等。
/// 支持聊天场景下的公告功能。
#[derive(Clone)]
pub struct AnnouncementService {
    config: Config,
}

impl AnnouncementService {
    /// 创建新的聊天公告服务实例
    ///
    /// # 参数
    /// * `config` - SDK配置信息
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取聊天公告
    ///
    /// 获取指定聊天公告的详细信息，包括公告内容、结构等。
    /// 支持获取公告的完整内容和元数据。
    ///
    /// # 参数
    /// * `request` - 获取公告请求
    /// * `option` - 可选请求配置
    ///
    /// # 返回
    /// 成功返回公告详细信息，失败返回错误信息
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::service::cloud_docs::docx::v1::chat::announcement::{AnnouncementService, GetChatAnnouncementRequest};
    ///
    /// let service = AnnouncementService::new(config);
    /// let request = GetChatAnnouncementRequest::new("announcement_id");
    ///
    /// let response = service.get(request, None).await?;
    /// println!("公告标题: {}", response.title);
    /// ```
    pub async fn get(
        &self,
        request: GetChatAnnouncementRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<AnnouncementData> {
        let response = get_chat_announcement(request, &self.config, option).await?;
        let resp_data = response
            .data
            .ok_or_else(|| validation_error("response_data", "Response data is missing"))?;
        resp_data
            .data
            .ok_or_else(|| validation_error("data", "Announcement data is missing"))
    }
}

impl openlark_core::trait_system::service::Service for AnnouncementService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "announcement"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::trait_system::service::Service;

    fn create_test_service() -> AnnouncementService {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        AnnouncementService::new(config)
    }

    #[test]
    fn test_announcement_service_creation() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = AnnouncementService::new(config);

        assert_eq!(service.config().app_id(), "test_app_id");
        assert_eq!(service.config().app_secret(), "test_app_secret");
    }

    #[test]
    fn test_announcement_service_clone() {
        let service = create_test_service();
        let cloned_service = service.clone();

        assert_eq!(service.config().app_id(), cloned_service.config().app_id());
        assert_eq!(
            service.config().app_secret(),
            cloned_service.config().app_secret()
        );
    }

    #[test]
    fn test_service_trait_implementation() {
        let service = create_test_service();

        // 测试Service trait的实现
        let config_ref = service.config();
        assert_eq!(config_ref.app_id(), "test_app_id");
    }

    #[test]
    fn test_module_structure() {
        // 这个测试验证模块结构的完整性
        let service = create_test_service();

        // 验证可以访问所有服务方法
        let _get_request = GetChatAnnouncementRequest::new("announcement_id");

        // 如果编译通过，说明模块结构正确
        assert!(true);
    }
}
