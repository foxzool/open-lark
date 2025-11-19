use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api::ApiRequest,
        api::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::ehr::models::{AttachmentDownloadResponse, EmployeeAttachmentRequest},
};

/// 人员附件服务
pub struct AttachmentService {
    pub config: Config,
}

/// 下载人员附件响应包装器
#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeAttachmentDownloadResponse {
    /// 附件下载信息
    #[serde(flatten)]
    pub attachment: AttachmentDownloadResponse,
}

impl ApiResponseTrait for EmployeeAttachmentDownloadResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl AttachmentService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 下载人员的附件
    ///
    /// 该接口用于下载指定员工的附件文件，如身份证扫描件、学历证明、
    /// 合同文件等。支持多种文件格式，返回base64编码的文件内容。
    ///
    /// # 参数
    ///
    /// - `request`: 附件下载请求参数，包括：
    ///   - `employee_id`: 员工ID
    ///   - `attachment_id`: 附件ID
    ///   - `user_id_type`: 用户ID类型（可选）
    /// - `option`: 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回附件下载响应，包括：
    /// - `file_content`: 文件内容（base64编码）
    /// - `file_name`: 文件名
    /// - `content_type`: 文件MIME类型
    /// - `file_size`: 文件大小（字节）
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use open_lark::service::ehr::models::EmployeeAttachmentRequest;
    ///
    /// let request = EmployeeAttachmentRequest {
    ///     employee_id: "emp_123456".to_string(),
    ///     attachment_id: "attach_789".to_string(),
    ///     user_id_type: Some("open_id".to_string()),
    /// };
    ///
    /// let response = client.ehr.attachment.download_attachment(request, None).await?;
    /// if let Some(data) = &response.data {
    ///     if let Some(file_content) = &data.attachment.file_content {
    ///         // 解码base64文件内容
    ///         let file_bytes = base64::decode(file_content)?;
    ///         // 保存或处理文件
    ///     }
    /// }
    /// ```
    ///
    /// # 安全注意事项
    ///
    /// - 确保有权限访问指定员工的附件
    /// - 妥善处理敏感的人事文件内容
    /// - 建议对下载的文件进行病毒扫描
    /// - 注意文件大小限制，避免内存溢出
    pub async fn download_attachment(
        &self,
        request: EmployeeAttachmentRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<EmployeeAttachmentDownloadResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: format!(
                "/open-apis/ehr/v1/employees/{}/attachments/{}",
                request.employee_id, request.attachment_id
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        if let Some(user_id_type) = request.user_id_type {
            api_req.query_params.insert("user_id_type", user_id_type);
        }

        Transport::request(api_req, &self.config, option).await
    }
}
