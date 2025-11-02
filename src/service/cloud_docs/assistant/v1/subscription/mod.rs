use open_lark_core::{core::{config::Config, req_option::RequestOption, SDKResult}};

pub use create::{
    create_subscription, CreateSubscriptionRequest, CreateSubscriptionResponse, SubscriptionConfig,
    SubscriptionPriority,
};
pub use get::{
    get_subscription, FileType, GetSubscriptionRequest, GetSubscriptionResponse,
    SubscriptionDetail, SubscriptionStatus,
};
pub use patch::{patch_subscription, PatchSubscriptionRequest, PatchSubscriptionResponse};

pub mod create;
pub mod get;
pub mod patch;

/// 订阅服务
#[derive(Debug, Clone)]
pub struct SubscriptionService {
    config: Config,
}

impl SubscriptionService {
    /// 创建新的订阅服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取订阅状态
    pub async fn get(
        &self,
        request: GetSubscriptionRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<GetSubscriptionResponse> {
        let result = get_subscription(request, &self.config, option).await?;
        result.data.ok_or_else(|| {
            open_lark_core::crate::core::error::LarkAPIError::IllegalParamError(
                "Response data is missing".to_string(),
            )
        })
    }

    /// 创建订阅
    pub async fn create(
        &self,
        request: CreateSubscriptionRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<CreateSubscriptionResponse> {
        let result = create_subscription(request, &self.config, option).await?;
        result.data.ok_or_else(|| {
            open_lark_core::crate::core::error::LarkAPIError::IllegalParamError(
                "Response data is missing".to_string(),
            )
        })
    }

    /// 更新订阅状态
    pub async fn patch(
        &self,
        request: PatchSubscriptionRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<PatchSubscriptionResponse> {
        let result = patch_subscription(request, &self.config, option).await?;
        result.data.ok_or_else(|| {
            open_lark_core::crate::core::error::LarkAPIError::IllegalParamError(
                "Response data is missing".to_string(),
            )
        })
    }

    /// 快速订阅文档（基础配置）
    pub async fn quick_subscribe_doc(
        &self,
        file_token: impl ToString,
        option: Option<RequestOption>,
    ) -> SDKResult<CreateSubscriptionResponse> {
        let request = CreateSubscriptionRequest::builder()
            .file_token(file_token.to_string())
            .as_doc()
            .basic_subscription()
            .build();
        self.create(request, option).await
    }

    /// 快速订阅表格（基础配置）
    pub async fn quick_subscribe_sheet(
        &self,
        file_token: impl ToString,
        option: Option<RequestOption>,
    ) -> SDKResult<CreateSubscriptionResponse> {
        let request = CreateSubscriptionRequest::builder()
            .file_token(file_token.to_string())
            .as_sheet()
            .basic_subscription()
            .build();
        self.create(request, option).await
    }

    /// 快速订阅幻灯片（基础配置）
    pub async fn quick_subscribe_slide(
        &self,
        file_token: impl ToString,
        option: Option<RequestOption>,
    ) -> SDKResult<CreateSubscriptionResponse> {
        let request = CreateSubscriptionRequest::builder()
            .file_token(file_token.to_string())
            .as_slide()
            .basic_subscription()
            .build();
        self.create(request, option).await
    }

    /// 快速订阅白板（基础配置）
    pub async fn quick_subscribe_whiteboard(
        &self,
        file_token: impl ToString,
        option: Option<RequestOption>,
    ) -> SDKResult<CreateSubscriptionResponse> {
        let request = CreateSubscriptionRequest::builder()
            .file_token(file_token.to_string())
            .as_whiteboard()
            .basic_subscription()
            .build();
        self.create(request, option).await
    }

    /// 快速订阅思维导图（基础配置）
    pub async fn quick_subscribe_mindnote(
        &self,
        file_token: impl ToString,
        option: Option<RequestOption>,
    ) -> SDKResult<CreateSubscriptionResponse> {
        let request = CreateSubscriptionRequest::builder()
            .file_token(file_token.to_string())
            .as_mindnote()
            .basic_subscription()
            .build();
        self.create(request, option).await
    }

    /// 高级订阅文档（高级配置）
    pub async fn premium_subscribe_doc(
        &self,
        file_token: impl ToString,
        option: Option<RequestOption>,
    ) -> SDKResult<CreateSubscriptionResponse> {
        let request = CreateSubscriptionRequest::builder()
            .file_token(file_token.to_string())
            .as_doc()
            .high_priority()
            .notification_interval(1800) // 30分钟
            .auto_renew()
            .add_tag("premium")
            .add_tag("document")
            .build();
        self.create(request, option).await
    }

    /// 紧急订阅表格（紧急配置）
    pub async fn urgent_subscribe_sheet(
        &self,
        file_token: impl ToString,
        option: Option<RequestOption>,
    ) -> SDKResult<CreateSubscriptionResponse> {
        let request = CreateSubscriptionRequest::builder()
            .file_token(file_token.to_string())
            .as_sheet()
            .urgent_priority()
            .notification_interval(300) // 5分钟
            .auto_renew()
            .add_tag("urgent")
            .add_tag("spreadsheet")
            .build();
        self.create(request, option).await
    }

    /// 激活订阅
    pub async fn activate(
        &self,
        file_token: impl ToString,
        file_type: FileType,
        option: Option<RequestOption>,
    ) -> SDKResult<PatchSubscriptionResponse> {
        let request = PatchSubscriptionRequest::builder()
            .file_token(file_token.to_string())
            .file_type(file_type)
            .activate()
            .build();
        self.patch(request, option).await
    }

    /// 暂停订阅
    pub async fn pause(
        &self,
        file_token: impl ToString,
        file_type: FileType,
        option: Option<RequestOption>,
    ) -> SDKResult<PatchSubscriptionResponse> {
        let request = PatchSubscriptionRequest::builder()
            .file_token(file_token.to_string())
            .file_type(file_type)
            .pause()
            .build();
        self.patch(request, option).await
    }

    /// 取消订阅
    pub async fn cancel(
        &self,
        file_token: impl ToString,
        file_type: FileType,
        option: Option<RequestOption>,
    ) -> SDKResult<PatchSubscriptionResponse> {
        let request = PatchSubscriptionRequest::builder()
            .file_token(file_token.to_string())
            .file_type(file_type)
            .cancel()
            .build();
        self.patch(request, option).await
    }

    /// 快速激活订阅（高频模式）
    pub async fn quick_activate(
        &self,
        file_token: impl ToString,
        file_type: FileType,
        option: Option<RequestOption>,
    ) -> SDKResult<PatchSubscriptionResponse> {
        let request = PatchSubscriptionRequest::builder()
            .file_token(file_token.to_string())
            .file_type(file_type)
            .quick_activate()
            .build();
        self.patch(request, option).await
    }

    /// 节能模式激活订阅（低频模式）
    pub async fn eco_activate(
        &self,
        file_token: impl ToString,
        file_type: FileType,
        option: Option<RequestOption>,
    ) -> SDKResult<PatchSubscriptionResponse> {
        let request = PatchSubscriptionRequest::builder()
            .file_token(file_token.to_string())
            .file_type(file_type)
            .eco_activate()
            .build();
        self.patch(request, option).await
    }

    /// 安全暂停订阅（附加系统标签）
    pub async fn safe_pause(
        &self,
        file_token: impl ToString,
        file_type: FileType,
        option: Option<RequestOption>,
    ) -> SDKResult<PatchSubscriptionResponse> {
        let request = PatchSubscriptionRequest::builder()
            .file_token(file_token.to_string())
            .file_type(file_type)
            .safe_pause()
            .build();
        self.patch(request, option).await
    }

    /// 检查订阅状态并返回是否已订阅
    pub async fn is_subscribed(
        &self,
        file_token: impl ToString,
        file_type: FileType,
        option: Option<RequestOption>,
    ) -> SDKResult<bool> {
        let request = GetSubscriptionRequest::builder()
            .file_token(file_token.to_string())
            .file_type(file_type)
            .build();
        let response = self.get(request, option).await?;
        Ok(response.subscription.is_subscribed())
    }

    /// 批量管理订阅 - 订阅多个文档
    pub async fn batch_subscribe(
        &self,
        files: Vec<(String, FileType)>,
        option: Option<RequestOption>,
    ) -> Vec<SDKResult<CreateSubscriptionResponse>> {
        let mut results = Vec::new();
        for (file_token, file_type) in files {
            let request = CreateSubscriptionRequest::builder()
                .file_token(file_token)
                .file_type(file_type.to_string().as_str().into())
                .basic_subscription()
                .build();
            let result = self.create(request, option.clone()).await;
            results.push(result);
        }
        results
    }

    /// 批量激活订阅
    pub async fn batch_activate(
        &self,
        files: Vec<(String, FileType)>,
        option: Option<RequestOption>,
    ) -> Vec<SDKResult<PatchSubscriptionResponse>> {
        let mut results = Vec::new();
        for (file_token, file_type) in files {
            let result = self.activate(file_token, file_type, option.clone()).await;
            results.push(result);
        }
        results
    }

    /// 批量暂停订阅
    pub async fn batch_pause(
        &self,
        files: Vec<(String, FileType)>,
        option: Option<RequestOption>,
    ) -> Vec<SDKResult<PatchSubscriptionResponse>> {
        let mut results = Vec::new();
        for (file_token, file_type) in files {
            let result = self.pause(file_token, file_type, option.clone()).await;
            results.push(result);
        }
        results
    }
}

#[cfg(test)]
mod tests;