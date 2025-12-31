use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

/// 分片上传素材-预上传
///
/// 发送初始化请求，以获取上传事务 ID 和分片策略，为上传分片做准备。
/// docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/media/upload_prepare
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 分片上传素材-预上传请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadPrepareMediaRequest {
    #[serde(skip)]
    config: Config,
    /// 素材的文件名称
    pub file_name: String,
    /// 上传点的类型
    pub parent_type: String,
    /// 上传点的 token
    pub parent_node: String,
    /// 文件大小（字节）
    pub size: i64,
    /// 拓展参数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,
}

impl UploadPrepareMediaRequest {
    pub fn new(
        config: Config,
        file_name: impl Into<String>,
        parent_type: impl Into<String>,
        parent_node: impl Into<String>,
        size: i64,
    ) -> Self {
        Self {
            config,
            file_name: file_name.into(),
            parent_type: parent_type.into(),
            parent_node: parent_node.into(),
            size,
            extra: None,
        }
    }

    pub fn extra(mut self, extra: impl Into<String>) -> Self {
        self.extra = Some(extra.into());
        self
    }

    pub async fn execute(self) -> SDKResult<UploadPrepareMediaResponse> {
        let file_name_len = self.file_name.chars().count();
        if file_name_len == 0 || file_name_len > 250 {
            return Err(openlark_core::error::validation_error(
                "file_name",
                "file_name 长度必须在 1~250 字符之间",
            ));
        }
        match self.parent_type.as_str() {
            "doc_image"
            | "docx_image"
            | "sheet_image"
            | "doc_file"
            | "docx_file"
            | "sheet_file"
            | "vc_virtual_background"
            | "bitable_image"
            | "bitable_file"
            | "moments"
            | "ccm_import_open" => {}
            _ => {
                return Err(openlark_core::error::validation_error(
                    "parent_type",
                    "parent_type 不在支持的取值范围内",
                ))
            }
        }
        if self.parent_node.is_empty() {
            return Err(openlark_core::error::validation_error(
                "parent_node",
                "parent_node 不能为空",
            ));
        }
        if self.size < 0 {
            return Err(openlark_core::error::validation_error("size", "size 不能为负数"));
        }

        let api_endpoint = DriveApi::UploadMediaPrepare;
        let request = ApiRequest::<UploadPrepareMediaResponse>::post(&api_endpoint.to_url()).body(
            serialize_params(&self, "分片上传素材-预上传")?,
        );

        let response = Transport::request(request, &self.config, None).await?;
        extract_response_data(response, "分片上传素材-预上传")
    }
}

/// 分片上传素材准备响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadPrepareMediaResponse {
    /// 分片上传事务 ID
    pub upload_id: String,
    /// 分片大小策略
    pub block_size: i32,
    /// 分片数量
    pub block_num: i32,
}

impl ApiResponseTrait for UploadPrepareMediaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upload_prepare_request() {
        let config = Config::default();
        let request = UploadPrepareMediaRequest::new(
            config,
            "demo.jpeg",
            "docx_image",
            "doccnFivLCfJfblZjGZtxgabcef",
            1024,
        )
        .extra("{\"drive_route_token\":\"doxcnXgNGAtaAraIRVeCfmabcef\"}");

        assert_eq!(request.file_name, "demo.jpeg");
        assert_eq!(request.parent_type, "docx_image");
        assert_eq!(request.size, 1024);
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(UploadPrepareMediaResponse::data_format(), ResponseFormat::Data);
    }
}
