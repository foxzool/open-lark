//! 上传素材
//!
//! 将文件、图片、视频等素材文件上传到指定云文档中。
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/media/upload_all

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 上传素材请求
#[derive(Debug)]
pub struct UploadAllMediaRequest {
    config: Config,
    /// 要上传的素材的名称
    pub file_name: String,
    /// 上传点的类型
    pub parent_type: String,
    /// 上传点的 token
    pub parent_node: String,
    /// 文件大小（字节）
    pub size: usize,
    /// 文件的 Adler-32 校验和
    pub checksum: Option<String>,
    /// 拓展参数
    pub extra: Option<String>,
    /// 文件二进制内容
    pub file: Vec<u8>,
}

impl UploadAllMediaRequest {
    pub fn new(
        config: Config,
        file_name: impl Into<String>,
        parent_type: impl Into<String>,
        parent_node: impl Into<String>,
        size: usize,
        file: Vec<u8>,
    ) -> Self {
        Self {
            config,
            file_name: file_name.into(),
            parent_type: parent_type.into(),
            parent_node: parent_node.into(),
            size,
            checksum: None,
            extra: None,
            file,
        }
    }

    /// 设置文件校验和（Adler-32）
    pub fn checksum(mut self, checksum: impl Into<String>) -> Self {
        self.checksum = Some(checksum.into());
        self
    }

    /// 设置拓展参数
    pub fn extra(mut self, extra: impl Into<String>) -> Self {
        self.extra = Some(extra.into());
        self
    }

    pub async fn execute(self) -> SDKResult<UploadAllMediaResponse> {
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
        if self.size == 0 || self.size > 20 * 1024 * 1024 {
            return Err(openlark_core::error::validation_error(
                "size",
                "size 必须在 1~20971520 字节之间",
            ));
        }
        if self.file.len() != self.size {
            return Err(openlark_core::error::validation_error(
                "size",
                "size 必须与 file 的实际长度一致",
            ));
        }

        let api_endpoint = DriveApi::UploadMedia;

        #[derive(Serialize)]
        struct UploadMeta {
            file_name: String,
            parent_type: String,
            parent_node: String,
            size: usize,
            #[serde(skip_serializing_if = "Option::is_none")]
            checksum: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            extra: Option<String>,
        }

        let meta = UploadMeta {
            file_name: self.file_name,
            parent_type: self.parent_type,
            parent_node: self.parent_node,
            size: self.size,
            checksum: self.checksum,
            extra: self.extra,
        };

        let request = ApiRequest::<UploadAllMediaResponse>::post(&api_endpoint.to_url())
            .json_body(&meta)
            .file_content(self.file);

        let response = Transport::request(request, &self.config, None).await?;
        extract_response_data(response, "上传素材")
    }
}

/// 上传素材响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadAllMediaResponse {
    /// 素材文件的 token
    pub file_token: String,
}

impl ApiResponseTrait for UploadAllMediaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upload_file_request_builder() {
        let config = Config::default();
        let request = UploadAllMediaRequest::new(
            config,
            "test.png",
            "docx_image",
            "doc_token",
            100,
            vec![0; 100],
        )
        .extra("extra_info");

        assert_eq!(request.file_name, "test.png");
        assert_eq!(request.parent_type, "docx_image");
        assert_eq!(request.extra, Some("extra_info".to_string()));
    }
}
