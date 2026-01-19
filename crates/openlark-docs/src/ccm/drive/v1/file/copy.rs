//! 复制文件
//!
//! 将文件复制到用户云空间的其他文件夹中。不支持复制文件夹。
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/file/copy

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 复制文件请求
#[derive(Debug, Clone, Serialize)]
pub struct CopyFileRequest {
    #[serde(skip)]
    config: Config,
    /// 被复制的源文件的 token
    pub file_token: String,
    /// 用户 ID 类型（默认 open_id）
    #[serde(skip)]
    pub user_id_type: Option<String>,
    /// 复制的新文件的名称（最大 256 字节）
    pub name: String,
    /// 目标文件夹的 token
    pub folder_token: String,
    /// 被复制的源文件的类型。必须与 file_token 对应的源文件实际类型一致
    #[serde(rename = "type")]
    pub r#type: String,
    /// 自定义请求附加参数，用于实现特殊的复制语义
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<Vec<Property>>,
}

impl CopyFileRequest {
    /// 创建复制文件请求
    ///
    /// # 参数
    /// * `config` - 配置
    /// * `file_token` - 被复制的源文件 token
    /// * `name` - 复制的新文件名称
    /// * `file_type` - 被复制的源文件类型
    /// * `folder_token` - 目标文件夹 token
    pub fn new(
        config: Config,
        file_token: impl Into<String>,
        name: impl Into<String>,
        file_type: impl Into<String>,
        folder_token: impl Into<String>,
    ) -> Self {
        Self {
            config,
            file_token: file_token.into(),
            user_id_type: None,
            name: name.into(),
            folder_token: folder_token.into(),
            r#type: file_type.into(),
            extra: None,
        }
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置自定义附加参数
    pub fn extra(mut self, extra: Vec<Property>) -> Self {
        self.extra = Some(extra);
        self
    }

    pub async fn execute(self) -> SDKResult<CopyFileResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CopyFileResponse> {
        if self.file_token.is_empty() {
            return Err(openlark_core::error::validation_error(
                "file_token",
                "file_token 不能为空",
            ));
        }
        // 文档说明：该参数为必填（请忽略必填列的"否"），为空会导致接口失败。
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

        match self.r#type.as_str() {
            "file" | "doc" | "sheet" | "bitable" | "docx" | "mindnote" | "slides" => {}
            _ => {
                return Err(openlark_core::error::validation_error(
                    "type",
                    "type 仅支持 file/doc/sheet/bitable/docx/mindnote/slides",
                ));
            }
        }

        if let Some(user_id_type) = &self.user_id_type {
            match user_id_type.as_str() {
                "open_id" | "union_id" | "user_id" => {}
                _ => {
                    return Err(openlark_core::error::validation_error(
                        "user_id_type",
                        "user_id_type 仅支持 open_id/union_id/user_id",
                    ));
                }
            }
        }

        let name_len = self.name.len();
        if name_len > 256 || name_len == 0 {
            return Err(openlark_core::error::validation_error(
                "name",
                "name 长度必须在 1~256 字节之间",
            ));
        }

        if let Some(extra) = &self.extra {
            for (idx, prop) in extra.iter().enumerate() {
                if prop.key.trim().is_empty() {
                    return Err(openlark_core::error::validation_error(
                        &format!("extra[{}].key", idx),
                        "key 不能为空",
                    ));
                }
                if prop.value.trim().is_empty() {
                    return Err(openlark_core::error::validation_error(
                        &format!("extra[{}].value", idx),
                        "value 不能为空",
                    ));
                }
            }
        }

        let api_endpoint = DriveApi::CopyFile(self.file_token.clone());
        let mut request = ApiRequest::<CopyFileResponse>::post(&api_endpoint.to_url());

        if let Some(user_id_type) = &self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        #[derive(Serialize)]
        struct CopyFileBody {
            name: String,
            #[serde(rename = "type")]
            r#type: String,
            folder_token: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            extra: Option<Vec<Property>>,
        }

        request = request.body(serialize_params(
            &CopyFileBody {
                name: self.name,
                r#type: self.r#type,
                folder_token: self.folder_token,
                extra: self.extra,
            },
            "复制文件",
        )?);

        let response = Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "复制文件")
    }
}

/// 自定义属性键值对
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
    /// 自定义属性键对象
    pub key: String,
    /// 自定义属性值对象
    pub value: String,
}

/// 复制文件响应（data）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyFileResponse {
    /// 复制的新文件信息
    pub file: CopiedFile,
}

/// 快捷方式信息（该参数不会返回）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyShortcutInfo {
    /// 快捷方式指向的源文件类型
    pub target_type: String,
    /// 快捷方式指向的源文件 token
    pub target_token: String,
}

/// 复制的新文件信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopiedFile {
    /// 复制的新文件 token
    pub token: String,
    /// 新文件的名称
    pub name: String,
    /// 新文件的类型
    #[serde(rename = "type")]
    pub r#type: String,
    /// 新文件的父文件夹 token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_token: Option<String>,
    /// 文件在浏览器中的 URL 链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 快捷方式文件信息（该参数不会返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shortcut_info: Option<CopyShortcutInfo>,
    /// 文件创建时间（该参数不会返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// 文件最近修改时间（该参数不会返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_time: Option<String>,
    /// 文件所有者（该参数不会返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
}

impl ApiResponseTrait for CopyFileResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy_file_request_builder() {
        let config = Config::default();
        let request =
            CopyFileRequest::new(config, "file_token", "Demo copy", "docx", "folder_token");

        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.name, "Demo copy");
        assert_eq!(request.r#type, "docx");
        assert_eq!(request.folder_token, "folder_token");
        assert!(request.extra.is_none());
    }

    #[test]
    fn test_copy_file_request_with_extra() {
        let config = Config::default();
        let request =
            CopyFileRequest::new(config, "file_token", "Demo copy", "docx", "folder_token").extra(
                vec![Property {
                    key: "target_type".to_string(),
                    value: "docx".to_string(),
                }],
            );

        assert_eq!(
            request
                .extra
                .expect("extra should be set when .extra() is called")
                .len(),
            1
        );
    }

    #[test]
    fn test_copied_file_structure() {
        let file_data = CopiedFile {
            token: "doxcnUkUOWtOelpFcha2Zabcef".to_string(),
            name: "Demo copy".to_string(),
            r#type: "docx".to_string(),
            parent_token: Some("fldbcO1UuPz8VwnpPx5a92abcef".to_string()),
            url: Some("https://feishu.cn/docx/doxcnUkUOWtOelpFcha2Zabcef".to_string()),
            shortcut_info: None,
            created_time: None,
            modified_time: None,
            owner_id: None,
        };

        assert_eq!(file_data.token, "doxcnUkUOWtOelpFcha2Zabcef");
        assert_eq!(file_data.name, "Demo copy");
        assert_eq!(file_data.r#type, "docx");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(CopyFileResponse::data_format(), ResponseFormat::Data);
    }
}
