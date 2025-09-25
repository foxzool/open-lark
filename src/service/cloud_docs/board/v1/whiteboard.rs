use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::core::{
    api_req::ApiRequest,
    api_resp::{BaseResponse, BinaryResponse},
    config::Config,
    constants::AccessTokenType,
    endpoints::cloud_docs::*,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

/// 画板服务
pub struct WhiteboardService {
    config: Config,
}

impl WhiteboardService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取画板缩略图片
    ///
    /// 该接口用于获取画板的缩略图片。
    ///
    /// <https://open.feishu.cn/document/server-docs/docs/whiteboard-v1/whiteboard/get_thumbnail>
    pub async fn get_thumbnail(
        &self,
        request: GetWhiteboardThumbnailRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<BinaryResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: BOARD_V1_WHITEBOARD_THUMBNAIL.replace("{}", &request.whiteboard_token),
            supported_access_token_types: vec![AccessTokenType::User, AccessTokenType::Tenant],
            ..Default::default()
        };

        // 添加查询参数
        if let Some(format) = request.format {
            api_req.query_params.insert("format", format);
        }
        if let Some(width) = request.width {
            api_req.query_params.insert("width", width.to_string());
        }
        if let Some(height) = request.height {
            api_req.query_params.insert("height", height.to_string());
        }

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

/// 获取画板缩略图请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWhiteboardThumbnailRequest {
    /// 画板token
    pub whiteboard_token: String,
    /// 图片格式 (png, jpeg)
    pub format: Option<String>,
    /// 图片宽度
    pub width: Option<i32>,
    /// 图片高度
    pub height: Option<i32>,
}

impl GetWhiteboardThumbnailRequest {
    pub fn new(whiteboard_token: impl Into<String>) -> Self {
        Self {
            whiteboard_token: whiteboard_token.into(),
            format: None,
            width: None,
            height: None,
        }
    }

    pub fn with_format(mut self, format: impl Into<String>) -> Self {
        self.format = Some(format.into());
        self
    }

    pub fn with_width(mut self, width: i32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn with_height(mut self, height: i32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn with_size(mut self, width: i32, height: i32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }
}
