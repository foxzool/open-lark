use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 上传文件（一次性上传）
///
/// 上传指定文件到指定目录中，支持单次上传文件。
/// docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/upload_all
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 上传文件请求
#[derive(Debug)]
pub struct UploadAllRequest {
    config: Config,
    /// 文件名
    pub file_name: String,
    /// 父文件夹token
    pub parent_node: String,
    /// 父节点类型: explorer (文件夹)
    pub parent_type: String,
    /// 文件大小
    pub size: usize,
    /// 文件的 Adler-32 校验和
    pub checksum: Option<String>,
    /// 文件内容
    pub file: Vec<u8>,
}

/// 上传文件响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadAllResponse {
    /// 文件token
    pub file_token: String,
}

impl ApiResponseTrait for UploadAllResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl UploadAllRequest {
    /// 创建上传请求
    pub fn new(
        config: Config,
        file_name: impl Into<String>,
        parent_node: impl Into<String>,
        parent_type: impl Into<String>,
        size: usize,
        file: Vec<u8>,
    ) -> Self {
        Self {
            config,
            file_name: file_name.into(),
            parent_node: parent_node.into(),
            parent_type: parent_type.into(),
            size,
            checksum: None,
            file,
        }
    }

    /// 设置文件校验和（Adler-32）
    pub fn checksum(mut self, checksum: impl Into<String>) -> Self {
        self.checksum = Some(checksum.into());
        self
    }

    pub async fn execute(self) -> SDKResult<UploadAllResponse> {
        let file_name_len = self.file_name.chars().count();
        if file_name_len == 0 || file_name_len > 250 {
            return Err(openlark_core::error::validation_error(
                "file_name",
                "file_name 长度必须在 1~250 字符之间",
            ));
        }
        if self.parent_type != "explorer" {
            return Err(openlark_core::error::validation_error(
                "parent_type",
                "parent_type 仅支持固定值 explorer",
            ));
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

        let api_endpoint = DriveApi::UploadFile;

        // 构建 multipart 所需的元数据
        #[derive(Serialize)]
        struct UploadMeta {
            file_name: String,
            parent_type: String,
            parent_node: String,
            size: usize,
            #[serde(skip_serializing_if = "Option::is_none")]
            checksum: Option<String>,
        }

        let meta = UploadMeta {
            file_name: self.file_name,
            parent_type: self.parent_type,
            parent_node: self.parent_node,
            size: self.size,
            checksum: self.checksum,
        };

        // 使用 json_body 传递元数据，使用 file_content 传递文件
        // UnifiedRequestBuilder 会将其组合成 multipart 请求
        let request = ApiRequest::<UploadAllResponse>::post(&api_endpoint.to_url())
            .json_body(&meta)
            .file_content(self.file);

        let response = Transport::request(request, &self.config, None).await?;
        extract_response_data(response, "上传文件")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upload_all_request() {
        let config = Config::default();
        let request =
            UploadAllRequest::new(config, "test.txt", "folder_token", "explorer", 100, vec![]);
        assert_eq!(request.file_name, "test.txt");
        assert_eq!(request.parent_node, "folder_token");
        assert_eq!(request.parent_type, "explorer");
    }
}
