use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 分片上传文件-预上传
///
/// 发送初始化请求，以获取上传事务 ID 和分片策略，为上传分片做准备。
/// docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/upload_prepare
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 分片上传预备请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadPrepareRequest {
    #[serde(skip)]
    config: Config,
    /// 文件名
    pub file_name: String,
    /// 上传点的类型：固定为 explorer（云空间）
    pub parent_type: String,
    /// 云空间中文件夹的 token
    pub parent_node: String,
    /// 文件大小（字节）
    pub size: i64,
}

impl UploadPrepareRequest {
    pub fn new(
        config: Config,
        file_name: impl Into<String>,
        parent_node: impl Into<String>,
        size: i64,
    ) -> Self {
        Self {
            config,
            file_name: file_name.into(),
            parent_type: "explorer".to_string(),
            parent_node: parent_node.into(),
            size,
        }
    }

    /// 覆盖上传点类型（文档固定为 explorer，一般无需设置）
    pub fn parent_type(mut self, parent_type: impl Into<String>) -> Self {
        self.parent_type = parent_type.into();
        self
    }

    pub async fn execute(self) -> SDKResult<UploadPrepareResponse> {
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
        if self.size < 0 {
            return Err(openlark_core::error::validation_error(
                "size",
                "size 不能为负数",
            ));
        }

        let api_endpoint = DriveApi::UploadPrepare;
        let request =
            ApiRequest::<UploadPrepareResponse>::post(&api_endpoint.to_url()).body(serialize_params(
                &self,
                "分片上传文件-预上传",
            )?);

        let response = Transport::request(request, &self.config, None).await?;
        extract_response_data(response, "分片上传文件-预上传")
    }
}

/// 分片上传预备响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadPrepareResponse {
    /// 上传会话ID
    pub upload_id: String,
    /// 分片大小（固定为 4MB）
    pub block_size: i32,
    /// 分片数量
    pub block_num: i32,
}

impl ApiResponseTrait for UploadPrepareResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upload_prepare_request_builder() {
        let config = Config::default();
        let request = UploadPrepareRequest::new(config, "test.txt", "folder_token", 1024);
        assert_eq!(request.file_name, "test.txt");
        assert_eq!(request.parent_node, "folder_token");
        assert_eq!(request.size, 1024);
    }
}
