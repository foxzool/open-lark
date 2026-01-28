//! 根据简历附件创建招聘官网投递任务
//!
//! docPath: https://open.feishu.cn/document/server-docs/hire-v1/website.delivery/create_by_attachment

use openlark_core::{
    api::{ApiResponseTrait, ResponseFormat},
    config::Config,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 根据简历附件创建招聘官网投递任务请求
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CreateByAttachmentRequest {
    /// 配置信息
    config: Config,
    // TODO: 添加请求字段
}

impl CreateByAttachmentRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            // TODO: 初始化字段
        }
    }

    // TODO: 添加字段 setter 方法

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateByAttachmentResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        _option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateByAttachmentResponse> {
        // TODO: 实现 API 调用逻辑
        todo!("实现 根据简历附件创建招聘官网投递任务 API 调用")
    }
}

/// 根据简历附件创建招聘官网投递任务响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateByAttachmentResponse {
    /// 响应数据
    ///
    /// TODO: 根据官方文档添加具体字段
    pub data: Value,
}

impl ApiResponseTrait for CreateByAttachmentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
