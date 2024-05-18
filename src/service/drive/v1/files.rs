use crate::core::config::Config;
use bytes::Bytes;

/// 上传文件 请求体
#[derive(Debug, Clone, Default)]
pub struct UploadAllRequest {
    /// 文件名。
    ///
    /// 示例值："demo.pdf"
    file_name: String,
    /// 上传点类型。
    ///
    /// 示例值："explorer"
    parent_type: String,
    /// 文件夹token，获取方式见 概述
    ///
    /// 示例值："fldbcO1UuPz8VwnpPx5a92abcef"
    parent_node: String,
    /// 文件大小（以字节为单位）。
    ///
    /// 示例值：1024
    size: i32,
    /// 文件adler32校验和(可选)。
    checksum: Option<String>,
    /// 文件二进制内容。
    file: Bytes,
}

impl UploadAllRequest {
    pub fn file_name(mut self, file_name: impl ToString) -> Self {
        self.file_name = file_name.to_string();
        self
    }

    pub fn parent_type(mut self, parent_type: impl ToString) -> Self {
        self.parent_type = parent_type.to_string();
        self
    }

    pub fn parent_node(mut self, parent_node: impl ToString) -> Self {
        self.parent_node = parent_node.to_string();
        self
    }

    pub fn size(mut self, size: i32) -> Self {
        self.size = size;
        self
    }

    pub fn checksum(mut self, checksum: Option<impl ToString>) -> Self {
        self.checksum = checksum.map(|x| x.to_string());
        self
    }

    pub fn file(mut self, file: Bytes) -> Self {
        self.file = file;
        self
    }
}

pub struct FilesService {
    config: Config,
}

impl FilesService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 上传文件
    pub async fn upload_all(
        &self,
        req_body: UploadAllRequest,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
