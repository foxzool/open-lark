/// 下载妙记音视频文件
///
/// 下载妙记的音视频文件。
/// docPath: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/minutes-v1/minute-media/get
/// 文档参考：https://open.feishu.cn/document/minutes-v1/minute-media/get
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::minutes::v1::minute::models::MinuteMediaInfo;

/// 下载妙记音视频文件请求
pub struct GetMinuteMediaRequest {
    minute_token: String,
    config: Config,
}

/// 下载妙记音视频文件响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMinuteMediaResponse {
    /// 音视频文件信息
    pub media_info: MinuteMediaInfo,
}

impl ApiResponseTrait for GetMinuteMediaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetMinuteMediaRequest {
    /// 创建下载妙记音视频文件请求
    pub fn new(config: Config) -> Self {
        Self {
            minute_token: String::new(),
            config,
        }
    }

    /// 设置妙记Token
    pub fn minute_token(mut self, minute_token: impl Into<String>) -> Self {
        self.minute_token = minute_token.into();
        self
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/minutes-v1/minute-media/get
    pub async fn execute(self) -> SDKResult<GetMinuteMediaResponse> {
        // 验证必填字段
        validate_required!(self.minute_token, "妙记Token不能为空");

        // 🚀 使用新的enum+builder系统生成API端点
        // 替代传统的字符串拼接方式，提供类型安全和IDE自动补全
        use crate::common::api_endpoints::MinutesApiV1;
        let api_endpoint = MinutesApiV1::MediaGet(self.minute_token.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<GetMinuteMediaResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}
