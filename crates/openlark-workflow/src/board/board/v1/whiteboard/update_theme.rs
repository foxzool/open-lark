//! 更新白板主题（v1）

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdateWhiteboardThemeBodyV1 {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub background_color: String,
    #[serde(default)]
    pub theme_config: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateWhiteboardThemeResponseV1 {
    pub theme_id: String,
    pub name: String,
    pub background_color: String,
    pub update_time: i64,
}

#[derive(Debug, Clone)]
pub struct UpdateWhiteboardThemeRequestV1 {
    config: Arc<Config>,
    board_id: String,
    body: UpdateWhiteboardThemeBodyV1,
}

impl UpdateWhiteboardThemeRequestV1 {
    pub fn new(config: Arc<Config>, board_id: impl Into<String>) -> Self {
        Self {
            config,
            board_id: board_id.into(),
            body: UpdateWhiteboardThemeBodyV1::default(),
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.body.name = name.into();
        self
    }

    pub fn background_color(mut self, background_color: impl Into<String>) -> Self {
        self.body.background_color = background_color.into();
        self
    }

    pub fn theme_config(mut self, theme_config: serde_json::Value) -> Self {
        self.body.theme_config = theme_config;
        self
    }

    pub async fn execute(self) -> SDKResult<UpdateWhiteboardThemeResponseV1> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UpdateWhiteboardThemeResponseV1> {
        validate_required!(self.board_id.trim(), "白板 ID 不能为空");

        let api_endpoint =
            crate::common::api_endpoints::BoardApiV1::WhiteboardUpdateTheme(self.board_id);
        let mut request = ApiRequest::<UpdateWhiteboardThemeResponseV1>::put(api_endpoint.to_url());

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

impl ApiResponseTrait for UpdateWhiteboardThemeResponseV1 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_whiteboard_update_theme_v1_url() {
        let endpoint = crate::common::api_endpoints::BoardApiV1::WhiteboardUpdateTheme(
            "test_board_id".to_string(),
        );
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/board/v1/whiteboards/test_board_id/theme"
        );
    }
}
