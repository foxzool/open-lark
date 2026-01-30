//! 下载白板为图片（v1）

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Default)]
pub struct DownloadWhiteboardAsImageBodyV1 {
    #[serde(rename = "format")]
    pub format: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub quality: String,
    #[serde(default)]
    pub include_thumbnail: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DownloadWhiteboardAsImageResponseV1 {
    pub image_url: String,
    pub width: u32,
    pub height: u32,
    #[serde(default)]
    pub file_size: u64,
    #[serde(default)]
    pub thumbnail_url: String,
}

#[derive(Debug, Clone)]
pub struct DownloadWhiteboardAsImageRequestV1 {
    config: Arc<Config>,
    board_id: String,
    body: DownloadWhiteboardAsImageBodyV1,
}

impl DownloadWhiteboardAsImageRequestV1 {
    pub fn new(config: Arc<Config>, board_id: impl Into<String>) -> Self {
        Self {
            config,
            board_id: board_id.into(),
            body: DownloadWhiteboardAsImageBodyV1::default(),
        }
    }

    pub fn format(mut self, format: impl Into<String>) -> Self {
        self.body.format = format.into();
        self
    }

    pub fn quality(mut self, quality: impl Into<String>) -> Self {
        self.body.quality = quality.into();
        self
    }

    pub fn include_thumbnail(mut self, include_thumbnail: bool) -> Self {
        self.body.include_thumbnail = include_thumbnail;
        self
    }

    pub async fn execute(self) -> SDKResult<DownloadWhiteboardAsImageResponseV1> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DownloadWhiteboardAsImageResponseV1> {
        validate_required!(self.board_id.trim(), "白板 ID 不能为空");
        validate_required!(self.body.format.trim(), "图片格式不能为空");

        let api_endpoint =
            crate::common::api_endpoints::BoardApiV1::WhiteboardDownloadAsImage(self.board_id);
        let mut request =
            ApiRequest::<DownloadWhiteboardAsImageResponseV1>::post(api_endpoint.to_url());

        let body_json = serde_json::to_value(&self.body).map_err(|e| {
            openlark_core::error::validation_error("序列化请求体失败", e.to_string().as_str())
        })?;

        request = request.body(body_json);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

impl ApiResponseTrait for DownloadWhiteboardAsImageResponseV1 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    

    #[test]
    fn test_whiteboard_download_as_image_v1_url() {
        let endpoint = crate::common::api_endpoints::BoardApiV1::WhiteboardDownloadAsImage(
            "test_board_id".to_string(),
        );
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/board/v1/whiteboards/test_board_id/image"
        );
    }
}
