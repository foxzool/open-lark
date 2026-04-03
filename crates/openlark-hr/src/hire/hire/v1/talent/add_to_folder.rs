//! 将人才加入指定文件夹
//!
//! docPath: https://open.feishu.cn/document/server-docs/hire-v1/talent/add_to_folder

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 将人才加入指定文件夹请求
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AddToFolderRequest {
    /// 配置信息
    config: Config,
    request_body: Option<Value>,
}

impl AddToFolderRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request_body: None,
        }
    }

    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = Some(request_body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<AddToFolderResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<AddToFolderResponse> {
        let mut request =
            ApiRequest::<AddToFolderResponse>::post("/open-apis/hire/v1/talents/add_to_folder");
        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        let response = Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "将人才加入指定文件夹响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 将人才加入指定文件夹响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AddToFolderResponse {
    /// 响应数据
    ///
    /// TODO: 根据官方文档添加具体字段
    pub data: Value,
}

impl ApiResponseTrait for AddToFolderResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use openlark_core::testing::prelude::TestConfigBuilder;

    #[test]
    fn test_add_to_folder_request_builder_new() {
        let request = AddToFolderRequest::new(TestConfigBuilder::new().build());
        let _ = request;
    }
}
