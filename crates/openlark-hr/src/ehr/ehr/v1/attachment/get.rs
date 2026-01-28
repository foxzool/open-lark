//! 下载人员的附件
//!
//! docPath: https://open.feishu.cn/document/server-docs/ehr-v1/attachment/get

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 下载人员的附件请求
#[derive(Debug, Clone)]
pub struct GetRequest {
    /// 附件 ID（必填）
    attachment_id: String,
    /// 用户 ID（必填）
    user_id: String,
    /// 配置信息
    config: Config,
}

impl GetRequest {
    /// 创建请求
    pub fn new(config: Config, attachment_id: String, user_id: String) -> Self {
        Self {
            attachment_id,
            user_id,
            config,
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetResponse> {
        use crate::common::api_endpoints::EhrApiV1;

        // 1. 构建端点
        let api_endpoint = EhrApiV1::AttachmentGet(self.attachment_id.clone());
        let mut request = ApiRequest::<GetResponse>::get(api_endpoint.to_url());

        // 2. 添加查询参数
        request = request.query("user_id", &self.user_id);

        // 3. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 4. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "下载人员附件响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 下载人员的附件响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetResponse {
    /// 附件名称
    pub name: String,
    /// 附件类型
    pub file_type: String,
    /// 附件大小（字节）
    pub size: i64,
    /// 附件下载 URL
    pub download_url: String,
    /// 附件 Token（用于下载文件）
    pub token: String,
}

impl ApiResponseTrait for GetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
