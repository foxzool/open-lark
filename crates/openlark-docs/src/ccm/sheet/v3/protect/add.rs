/// 添加工作表保护
///
/// 为工作表或指定范围添加保护，限制编辑权限。
/// docPath: https://open.feishu.cn/open-apis/sheets/v3/spreadsheets/:spreadsheetToken/sheetProtection
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::common::{api_endpoints::CcmSheetApiV3, api_utils::*};

/// 添加工作表保护请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddSheetProtectionRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 工作表ID
    pub sheet_id: String,
    /// 保护范围
    pub range: Option<String>,
    /// 保护类型
    pub protection_type: String,
    /// 密码
    pub password: Option<String>,
    /// 提示
    pub hint: Option<String>,
}

impl AddSheetProtectionRequest {
    /// 创建添加工作表保护请求
    ///
    /// # 参数
    /// * `spreadsheet_token` - 电子表格token
    /// * `sheet_id` - 工作表ID
    /// * `protection_type` - 保护类型
    pub fn new(
        spreadsheet_token: impl Into<String>,
        sheet_id: impl Into<String>,
        protection_type: impl Into<String>,
    ) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            sheet_id: sheet_id.into(),
            range: None,
            protection_type: protection_type.into(),
            password: None,
            hint: None,
        }
    }

    /// 设置保护范围
    pub fn range(mut self, range: impl Into<String>) -> Self {
        self.range = Some(range.into());
        self
    }

    /// 设置密码
    pub fn password(mut self, password: impl Into<String>) -> Self {
        self.password = Some(password.into());
        self
    }

    /// 设置提示
    pub fn hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }
}

/// 添加工作表保护响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddSheetProtectionResponse {
    /// 保护信息
    pub data: Option<SheetProtectionInfo>,
}

impl ApiResponseTrait for AddSheetProtectionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 工作表保护信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheetProtectionInfo {
    /// 保护ID
    pub protection_id: String,
    /// 创建时间
    pub create_time: String,
    /// 创建者信息
    pub creator: Option<CreatorInfo>,
}

/// 创建者信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatorInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名
    pub name: String,
}

/// 添加工作表保护
///
/// 为工作表或指定范围添加保护，限制编辑权限。
/// docPath: https://open.feishu.cn/open-apis/sheets/v3/spreadsheets/:spreadsheetToken/sheetProtection
pub async fn add_sheet_protection(
    request: AddSheetProtectionRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<AddSheetProtectionResponse>> {
    // 构建请求体
    let mut body = json!({
        "sheetId": request.sheet_id,
        "protectionType": request.protection_type
    });

    if let Some(range) = &request.range {
        body["range"] = json!(range);
    }
    if let Some(password) = &request.password {
        body["password"] = json!(password);
    }
    if let Some(hint) = &request.hint {
        body["hint"] = json!(hint);
    }

    // 创建API请求
    let mut api_request: ApiRequest<AddSheetProtectionResponse> =
        ApiRequest::post(&format!("{}/spreadsheets/{}/sheetProtection", CcmSheetApiV3, request.spreadsheet_token))
            .body(body);

    // 如果有请求选项，应用它们
    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    // 发送请求
    Transport::request(api_request, config, None).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_sheet_protection_request_builder() {
        let request = AddSheetProtectionRequest::new(
            "spreadsheet_token",
            "sheet_id",
            "EDIT"
        );

        assert_eq!(request.spreadsheet_token, "spreadsheet_token");
        assert_eq!(request.sheet_id, "sheet_id");
        assert_eq!(request.protection_type, "EDIT");
        assert!(request.range.is_none());
        assert!(request.password.is_none());
    }

    #[test]
    fn test_add_sheet_protection_request_builder_chain() {
        let request = AddSheetProtectionRequest::new(
            "spreadsheet_token",
            "sheet_id",
            "EDIT"
        )
        .range("A1:Z100")
        .password("protect123")
        .hint("请输入保护密码");

        assert_eq!(request.range, Some("A1:Z100".to_string()));
        assert_eq!(request.password, Some("protect123".to_string()));
        assert_eq!(request.hint, Some("请输入保护密码".to_string()));
    }

    #[test]
    fn test_sheet_protection_info_structure() {
        let creator = CreatorInfo {
            user_id: "user_123".to_string(),
            name: "张三".to_string(),
        };

        let protection_info = SheetProtectionInfo {
            protection_id: "protection_456".to_string(),
            create_time: "2023-01-01T00:00:00Z".to_string(),
            creator: Some(creator.clone()),
        };

        assert_eq!(protection_info.protection_id, "protection_456");
        assert_eq!(protection_info.creator.as_ref().unwrap().name, "张三");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(AddSheetProtectionResponse::data_format(), ResponseFormat::Data);
    }
}