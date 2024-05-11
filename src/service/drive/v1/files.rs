use bytes::Bytes;

/// 上传文件 请求体
#[derive(Debug, Clone)]
pub struct UploadAllReqBody {
    /// 文件名。
    ///
    /// 示例值："demo.pdf"
    pub file_name: String,
    /// 上传点类型。
    ///
    /// 示例值："explorer"
    pub parent_type: String,
    /// 文件夹token，获取方式见 概述
    ///
    /// 示例值："fldbcO1UuPz8VwnpPx5a92abcef"
    pub parent_node: String,
    /// 文件大小（以字节为单位）。
    ///
    /// 示例值：1024
    pub size: i32,
    /// 文件adler32校验和(可选)。
    pub checksum: Option<String>,
    /// 文件二进制内容。
    pub file: Bytes,
}
