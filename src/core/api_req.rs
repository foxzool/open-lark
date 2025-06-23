use std::collections::HashMap;

use reqwest::Method;

use crate::core::constants::AccessTokenType;

/// API请求的核心数据结构 - 命令模式的体现
///
/// `ApiRequest` 是整个SDK架构的核心，采用命令模式（Command Pattern）设计。
/// 它封装了发起一次飞书API调用所需的所有信息，充当服务层（Service）与传输层（Transport）之间的桥梁。
///
/// # 设计理念
///
/// - **解耦性**：服务层只负责构建请求，不关心HTTP细节
/// - **统一性**：所有API调用都通过这个统一的结构体表示
/// - **灵活性**：支持普通请求和multipart/form-data请求
///
/// # 使用流程
///
/// 1. 服务层方法创建并配置 `ApiRequest` 实例
/// 2. 设置HTTP方法、路径、认证需求等基本信息
/// 3. 根据请求类型填充 `body` 和/或 `file` 字段
/// 4. 将配置好的请求传递给 `Transport::request` 进行处理
///
/// # 示例
///
/// ```rust
/// // 普通JSON请求
/// let mut api_req = ApiRequest::default();
/// api_req.http_method = Method::POST;
/// api_req.api_path = "/open-apis/drive/v1/files".to_string();
/// api_req.body = serde_json::to_vec(&request_data)?;
///
/// // 文件上传请求（multipart）
/// let mut api_req = ApiRequest::default();
/// api_req.http_method = Method::POST;
/// api_req.api_path = "/open-apis/drive/v1/files/upload".to_string();
/// api_req.body = serde_json::to_vec(&metadata)?;  // JSON元数据
/// api_req.file = file_bytes;  // 文件内容
/// ```
#[derive(Debug, Clone, Default)]
pub struct ApiRequest {
    /// HTTP请求方法（GET、POST、PUT、DELETE等）
    ///
    /// 由服务层根据具体API的要求设置。
    /// 使用 `pub(crate)` 限制只能在crate内部修改，保证一致性。
    pub(crate) http_method: Method,

    /// API的相对路径
    ///
    /// 例如：`/open-apis/drive/v1/files/{file_id}`
    ///
    /// 路径中的动态参数（如 `{file_id}`）通常通过 `format!` 宏直接嵌入，
    /// 而不是使用 `path_params` 字段。
    pub api_path: String,

    /// 请求体数据（序列化后的字节数组）
    ///
    /// # 在不同请求类型中的用途：
    ///
    /// - **普通请求**：包含完整的请求体，通常是JSON序列化后的数据
    /// - **文件上传（multipart）**：仅包含JSON元数据部分，文件内容存储在 `file` 字段
    /// - **无请求体的请求**：保持为空 `Vec`
    ///
    /// # 注意事项
    ///
    /// 服务层通常使用 `serde_json::to_vec()` 将请求结构体序列化到这个字段。
    pub body: Vec<u8>,

    /// URL查询参数
    ///
    /// 存储将被附加到URL末尾的查询参数。
    /// 例如：`?page_size=10&page_token=xxx`
    ///
    /// # 示例
    ///
    /// ```rust
    /// api_req.query_params.insert("page_size".to_string(), "10".to_string());
    /// api_req.query_params.insert("page_token".to_string(), token);
    /// ```
    pub query_params: HashMap<String, String>,

    /// URL路径参数（当前未使用）
    ///
    /// 原设计意图可能是用于路径模板替换，如将 `/files/{id}` 中的 `{id}` 替换。
    /// 但当前实现中，路径参数都是通过 `format!` 宏直接嵌入到 `api_path` 中。
    ///
    /// # TODO
    ///
    /// 考虑移除此字段或实现路径模板功能以保持设计一致性。
    pub path_params: HashMap<String, Vec<String>>,

    /// 支持的访问令牌类型
    ///
    /// 指定此API端点接受哪些类型的访问令牌：
    /// - `User`：用户访问令牌
    /// - `Tenant`：租户访问令牌  
    /// - `App`：应用访问令牌
    ///
    /// Transport层会根据这个列表和当前配置选择合适的令牌类型。
    /// 使用 `pub(crate)` 确保只能由服务层设置。
    pub(crate) supported_access_token_types: Vec<AccessTokenType>,

    /// 文件内容（用于multipart/form-data请求）
    ///
    /// # 在不同请求类型中的用途：
    ///
    /// - **普通请求**：保持为空 `Vec`
    /// - **文件上传（multipart）**：包含要上传的文件的二进制内容
    ///
    /// # 工作原理
    ///
    /// 当 `file` 字段非空时，Transport层会自动识别这是一个multipart请求：
    /// 1. `body` 字段的内容作为multipart的JSON元数据部分
    /// 2. `file` 字段的内容作为文件部分
    /// 3. Content-Type自动设置为 `multipart/form-data`
    ///
    /// # 示例
    ///
    /// ```rust
    /// // 文件上传请求
    /// api_req.body = serde_json::to_vec(&FileMetadata {
    ///     name: "document.pdf",
    ///     parent_id: "folder123",
    /// })?;
    /// api_req.file = std::fs::read("path/to/document.pdf")?;
    /// ```
    pub file: Vec<u8>,
}
