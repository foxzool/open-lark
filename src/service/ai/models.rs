use serde::{Deserialize, Serialize};

/// 通用文件识别请求
#[derive(Debug, Serialize, Deserialize)]
pub struct FileRecognizeRequest {
    /// 文件token或base64编码
    pub file: String,
}

/// 通用识别响应
#[derive(Debug, Serialize, Deserialize)]
pub struct RecognizeResponse<T> {
    /// 识别结果
    pub data: T,
    /// 识别置信度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
}

/// 简历信息
#[derive(Debug, Serialize, Deserialize)]
pub struct ResumeInfo {
    /// 姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 电话
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// 邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 性别
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    /// 出生日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<String>,
    /// 教育经历
    #[serde(skip_serializing_if = "Option::is_none")]
    pub education: Option<Vec<EducationInfo>>,
    /// 工作经历
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_experience: Option<Vec<WorkExperienceInfo>>,
    /// 技能
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skills: Option<Vec<String>>,
}

/// 教育经历
#[derive(Debug, Serialize, Deserialize)]
pub struct EducationInfo {
    /// 学校名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub school: Option<String>,
    /// 专业
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major: Option<String>,
    /// 学历
    #[serde(skip_serializing_if = "Option::is_none")]
    pub degree: Option<String>,
    /// 开始时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 结束时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}

/// 工作经历
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkExperienceInfo {
    /// 公司名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    /// 职位
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    /// 开始时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 结束时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 工作描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// 身份证信息
#[derive(Debug, Serialize, Deserialize)]
pub struct IdCardInfo {
    /// 姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 性别
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    /// 民族
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nation: Option<String>,
    /// 出生日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<String>,
    /// 地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// 身份证号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<String>,
    /// 签发机关
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authority: Option<String>,
    /// 有效期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_period: Option<String>,
}

/// 驾驶证信息
#[derive(Debug, Serialize, Deserialize)]
pub struct DrivingLicenseInfo {
    /// 姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 性别
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    /// 国籍
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nationality: Option<String>,
    /// 出生日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<String>,
    /// 地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// 证号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_number: Option<String>,
    /// 准驾车型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vehicle_type: Option<String>,
    /// 有效期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_period: Option<String>,
}

/// 银行卡信息
#[derive(Debug, Serialize, Deserialize)]
pub struct BankCardInfo {
    /// 银行名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    /// 卡号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_number: Option<String>,
    /// 卡类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_type: Option<String>,
    /// 有效期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_period: Option<String>,
}

/// 营业执照信息
#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessLicenseInfo {
    /// 统一社会信用代码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_code: Option<String>,
    /// 企业名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    /// 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_type: Option<String>,
    /// 法定代表人
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_person: Option<String>,
    /// 注册资本
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_capital: Option<String>,
    /// 成立日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub establishment_date: Option<String>,
    /// 营业期限
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_term: Option<String>,
    /// 经营范围
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_scope: Option<String>,
    /// 住所
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

/// 增值税发票信息
#[derive(Debug, Serialize, Deserialize)]
pub struct VatInvoiceInfo {
    /// 发票代码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_code: Option<String>,
    /// 发票号码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_number: Option<String>,
    /// 开票日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_date: Option<String>,
    /// 购买方名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_name: Option<String>,
    /// 购买方税号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_tax_id: Option<String>,
    /// 销售方名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_name: Option<String>,
    /// 销售方税号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_tax_id: Option<String>,
    /// 合计金额
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<String>,
    /// 合计税额
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tax: Option<String>,
    /// 价税合计
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amount_with_tax: Option<String>,
}

/// 合同字段信息
#[derive(Debug, Serialize, Deserialize)]
pub struct ContractInfo {
    /// 合同标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 甲方
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party_a: Option<String>,
    /// 乙方
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party_b: Option<String>,
    /// 合同金额
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    /// 签订日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign_date: Option<String>,
    /// 生效日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    /// 终止日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// 其他字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_fields: Option<serde_json::Value>,
}

/// 名片信息
#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessCardInfo {
    /// 姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 职位
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    /// 公司
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    /// 电话
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// 手机
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    /// 邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 网址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    /// 地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

/// OCR文字识别结果
#[derive(Debug, Serialize, Deserialize)]
pub struct OcrResult {
    /// 识别的文本内容
    pub text: String,
    /// 文本位置信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_detection: Option<Vec<TextDetection>>,
}

/// 文本位置信息
#[derive(Debug, Serialize, Deserialize)]
pub struct TextDetection {
    /// 文本内容
    pub text: String,
    /// 边界框
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounding_box: Option<BoundingBox>,
    /// 置信度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
}

/// 边界框
#[derive(Debug, Serialize, Deserialize)]
pub struct BoundingBox {
    /// 左上角x坐标
    pub x: i32,
    /// 左上角y坐标
    pub y: i32,
    /// 宽度
    pub width: i32,
    /// 高度
    pub height: i32,
}

/// 语音识别请求
#[derive(Debug, Serialize, Deserialize)]
pub struct SpeechRecognizeRequest {
    /// 音频文件
    pub speech: SpeechFile,
    /// 语音格式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// 采样率
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<i32>,
    /// 语言
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

/// 语音文件
#[derive(Debug, Serialize, Deserialize)]
pub struct SpeechFile {
    /// 文件token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_token: Option<String>,
    /// base64编码的音频数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

/// 语音识别结果
#[derive(Debug, Serialize, Deserialize)]
pub struct SpeechRecognizeResult {
    /// 识别的文本
    pub recognition_text: String,
    /// 置信度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
}

/// 流式语音识别请求
#[derive(Debug, Serialize, Deserialize)]
pub struct StreamSpeechRequest {
    /// 音频数据流
    pub stream: String,
    /// 格式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// 采样率
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<i32>,
    /// 语言
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

/// 文本语种检测请求
#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageDetectRequest {
    /// 要检测的文本
    pub text: String,
}

/// 语种检测结果
#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageDetectResult {
    /// 检测到的语种
    pub language: String,
    /// 置信度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
}

/// 文本翻译请求
#[derive(Debug, Serialize, Deserialize)]
pub struct TranslateRequest {
    /// 源语言
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_language: Option<String>,
    /// 目标语言
    pub target_language: String,
    /// 要翻译的文本
    pub text: String,
}

/// 翻译结果
#[derive(Debug, Serialize, Deserialize)]
pub struct TranslateResult {
    /// 翻译后的文本
    pub translated_text: String,
    /// 检测到的源语言
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_language: Option<String>,
}
