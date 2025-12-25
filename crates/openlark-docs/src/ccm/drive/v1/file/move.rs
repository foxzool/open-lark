use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 移动文件或文件夹
///
/// 将文件或者文件夹移动到用户云空间的其他位置。
/// docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/move
/// doc: https://open.feishu.cn/document/server-docs/docs/drive-v1/file/move
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 移动文件请求
#[derive(Debug, Clone, Serialize)]
pub struct MoveFileRequest {
    #[serde(skip)]
    config: Config,
    /// 需要移动的文件或文件夹 token
    pub file_token: String,
    /// 文件类型（必填）
    #[serde(rename = "type")]
    pub r#type: String,
    /// 目标文件夹的 token
    pub folder_token: String,
}

impl MoveFileRequest {
    /// 创建移动文件请求
    ///
    /// # 参数
    /// * `config` - 配置
    /// * `file_token` - 需要移动的文件或文件夹 token
    /// * `file_type` - 文件类型（file/docx/bitable/doc/sheet/mindnote/folder/slides）
    /// * `folder_token` - 目标文件夹 token
    pub fn new(
        config: Config,
        file_token: impl Into<String>,
        file_type: impl Into<String>,
        folder_token: impl Into<String>,
    ) -> Self {
        Self {
            config,
            file_token: file_token.into(),
            r#type: file_type.into(),
            folder_token: folder_token.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<MoveFileResponse> {
        if self.file_token.is_empty() {
            return Err(openlark_core::error::validation_error(
                "file_token",
                "file_token 不能为空",
            ));
        }
        if self.r#type.is_empty() {
            return Err(openlark_core::error::validation_error(
                "type",
                "type 不能为空",
            ));
        }
        if self.folder_token.is_empty() {
            return Err(openlark_core::error::validation_error(
                "folder_token",
                "folder_token 不能为空",
            ));
        }

        let api_endpoint = DriveApi::MoveFile(self.file_token.clone());
        #[derive(Serialize)]
        struct MoveFileBody {
            #[serde(rename = "type")]
            r#type: String,
            folder_token: String,
        }
        let request = ApiRequest::<MoveFileResponse>::post(&api_endpoint.to_url()).body(serialize_params(
            &MoveFileBody {
                r#type: self.r#type,
                folder_token: self.folder_token,
            },
            "移动文件或文件夹",
        )?);

        let response = Transport::request(request, &self.config, None).await?;
        extract_response_data(response, "移动文件或文件夹")
    }
}

/// 移动文件响应（data）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveFileResponse {
    /// 异步任务 ID，移动文件夹时返回
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

impl ApiResponseTrait for MoveFileResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_file_request_builder() {
        let config = Config::default();
        let request = MoveFileRequest::new(config, "file_token", "file", "folder_token");

        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.r#type, "file");
        assert_eq!(request.folder_token, "folder_token");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(MoveFileResponse::data_format(), ResponseFormat::Data);
    }
}
