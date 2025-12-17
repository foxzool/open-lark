//! 导入表格
//!
//! docPath: https://open.feishu.cn/document/server-docs/historic-version/docs/sheets/sheet-operation/import-spreadsheet

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::common::api_endpoints::CcmSheetApiOld;

pub mod result;
pub use result::*;

/// 导入表格请求体（multipart 的表单字段）
///
/// 注意：`openlark-core` 的 multipart 构建器会读取该 JSON 对象中的 `file_name` 字段，
/// 并将二进制文件作为 `file` part 上传。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ImportReq {
    /// 文件名（必填）
    pub file_name: String,

    /// 其它字段：不同历史版本/场景下字段可能存在差异，允许透传扩展字段。
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// 导入表格响应（返回导入 ticket，后续可用 `import/result` 查询导入结果）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportResp {
    pub ticket: String,
}

impl ApiResponseTrait for ImportResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 导入表格请求
pub struct ImportRequest {
    config: Config,
    file: Vec<u8>,
    req: ImportReq,
}

impl ImportRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            file: Vec::new(),
            req: ImportReq::default(),
        }
    }

    /// 上传文件内容（必填）
    pub fn file(mut self, file: Vec<u8>) -> Self {
        self.file = file;
        self
    }

    /// 文件名（必填）
    pub fn file_name(mut self, file_name: impl Into<String>) -> Self {
        self.req.file_name = file_name.into();
        self
    }

    /// 透传额外字段（不同历史版本可能存在差异）
    pub fn field(mut self, key: impl Into<String>, value: impl Into<serde_json::Value>) -> Self {
        self.req.extra.insert(key.into(), value.into());
        self
    }

    pub async fn send(self) -> SDKResult<ImportResp> {
        // multipart: body 提供 file_name + 其它字段，file_content 提供真实文件 bytes
        let api_request: ApiRequest<ImportResp> =
            ApiRequest::post(&CcmSheetApiOld::Import.to_url())
                .body(serde_json::to_value(&self.req)?)
                .file_content(self.file);

        let response: Response<ImportResp> =
            Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}
