use serde::{Deserialize, Serialize};

/// 证件类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IdType {
    /// 身份证
    IdCard,
    /// 护照
    Passport,
    /// 港澳通行证
    HkMacauPass,
    /// 台湾通行证
    TaiwanPass,
    /// 其他证件
    Other,
}

/// 图片类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImageType {
    /// JPEG 格式
    Jpeg,
    /// PNG 格式
    Png,
    /// BMP 格式
    Bmp,
}

/// 认证状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthenticationStatus {
    /// 等待中 - 已创建身份记录但未开始认证
    Pending,
    /// 处理中 - 正在进行人脸识别处理
    Processing,
    /// 成功 - 认证通过
    Success,
    /// 失败 - 认证失败
    Failed,
    /// 超时 - 处理超时
    Timeout,
}

/// 裁剪参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CropParameters {
    /// 裁剪起始 X 坐标
    pub x: i32,
    /// 裁剪起始 Y 坐标
    pub y: i32,
    /// 裁剪宽度
    pub width: i32,
    /// 裁剪高度
    pub height: i32,
}

/// 人脸区域信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaceRegion {
    /// 人脸左上角 X 坐标
    pub x: i32,
    /// 人脸左上角 Y 坐标
    pub y: i32,
    /// 人脸区域宽度
    pub width: i32,
    /// 人脸区域高度
    pub height: i32,
    /// 检测置信度 (0-1)
    pub confidence: f64,
}

/// 身份认证记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity {
    /// 身份记录ID
    pub identity_id: String,
    /// 真实姓名
    pub name: String,
    /// 身份证号码
    pub id_number: String,
    /// 证件类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_type: Option<IdType>,
    /// 创建时间戳
    pub created_at: i64,
    /// 更新时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
}

/// 人脸图片信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaceImage {
    /// 图片ID
    pub image_id: String,
    /// 关联的身份记录ID
    pub identity_id: String,
    /// 图片类型
    pub image_type: ImageType,
    /// 上传时间戳
    pub uploaded_at: i64,
    /// 图片大小（字节）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
    /// 图片宽度（像素）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    /// 图片高度（像素）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
}

/// 认证结果详情
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationResult {
    /// 身份记录ID
    pub identity_id: String,
    /// 认证状态
    pub status: AuthenticationStatus,
    /// 置信度分数 (0-1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence_score: Option<f64>,
    /// 认证开始时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<i64>,
    /// 认证完成时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<i64>,
    /// 错误码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// 错误信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// 处理的图片ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_images: Option<Vec<String>>,
}

/// 认证统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationStats {
    /// 总认证次数
    pub total_count: i64,
    /// 成功次数
    pub success_count: i64,
    /// 失败次数
    pub failed_count: i64,
    /// 平均置信度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_confidence: Option<f64>,
    /// 平均处理时间（毫秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_processing_time: Option<i64>,
}
