use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        req_option, SDKResult,
    },
    service::sheets::v2::{data_operation::SheetDataUpdates, SpreadsheetService},
};
use crate::service::sheets::v2::data_operation::{CellStyle, StyleFont};

#[derive(Debug, Serialize, Default)]
pub struct SetCellStyleRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    #[serde(skip)]
    spreadsheet_token: String,
    /// 设置单元格样式
    #[serde(rename = "appendStyle")]
    append_style: AppendStyle,
}

#[derive(Debug, Serialize, Default)]
struct AppendStyle {
    range: String,
    style: CellStyle,
}

impl SetCellStyleRequest {
    pub fn builder() -> SetCellStyleRequestBuilder {
        SetCellStyleRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct SetCellStyleRequestBuilder {
    request: SetCellStyleRequest,
}

impl SetCellStyleRequestBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    pub fn range(mut self, range: impl ToString) -> Self {
        self.request.append_style.range = range.to_string();
        self
    }

    pub fn font(mut self, font: StyleFont) -> Self {
        self.request.append_style.style.font = font;
        self
    }

    pub fn text_decoration(mut self, text_decoration: i32) -> Self {
        self.request.append_style.style.text_decoration = Some(text_decoration);
        self
    }

    pub fn formatter(mut self, formatter: impl ToString) -> Self {
        self.request.append_style.style.formatter = Some(formatter.to_string());
        self
    }

    pub fn h_align(mut self, h_align: i32) -> Self {
        self.request.append_style.style.h_align = Some(h_align);
        self
    }

    pub fn v_align(mut self, v_align: i32) -> Self {
        self.request.append_style.style.v_align = Some(v_align);
        self
    }

    pub fn fore_color(mut self, fore_color: impl ToString) -> Self {
        self.request.append_style.style.fore_color = Some(fore_color.to_string());
        self
    }

    pub fn back_color(mut self, back_color: impl ToString) -> Self {
        self.request.append_style.style.back_color = Some(back_color.to_string());
        self
    }

    pub fn border_type(mut self, border_type: impl ToString) -> Self {
        self.request.append_style.style.border_type = Some(border_type.to_string());
        self
    }

    pub fn border_color(mut self, border_color: impl ToString) -> Self {
        self.request.append_style.style.border_color = Some(border_color.to_string());
        self
    }

    pub fn clean(mut self, clean: bool) -> Self {
        self.request.append_style.style.clean = clean;
        self
    }

    pub fn build(mut self) -> SetCellStyleRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

#[derive(Deserialize, Debug)]
pub struct SetCellStyleResponse {
    pub updates: SheetDataUpdates,
}

impl ApiResponseTrait for SetCellStyleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl SpreadsheetService {
    /// 该接口用于根据 spreadsheetToken 、range
    /// 和样式信息更新单元格样式；单次写入不超过5000行，100列。建议在设置边框样式时，
    /// 每次更新的单元格数量不要超过30000个。
    pub async fn set_cell_style(
        &self,
        request: SetCellStyleRequest,
        option: Option<req_option::RequestOption>,
    ) -> SDKResult<BaseResponse<SetCellStyleResponse>> {
        let mut api_req = request.api_request;
        api_req.api_path = format!(
            "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/style",
            spreadsheet_token = request.spreadsheet_token
        );
        api_req.http_method = reqwest::Method::PUT;
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::App];

        let api_resp = crate::core::http::Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}
