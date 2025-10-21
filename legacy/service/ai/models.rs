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

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_file_recognize_request_serialization() {
        let request = FileRecognizeRequest {
            file: "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg==".to_string(),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: FileRecognizeRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(request.file, deserialized.file);
    }

    #[test]
    fn test_recognize_response_serialization() {
        let response = RecognizeResponse {
            data: "识别结果".to_string(),
            confidence: Some(0.95),
        };

        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: RecognizeResponse<String> = serde_json::from_str(&serialized).unwrap();

        assert_eq!(response.data, deserialized.data);
        assert_eq!(response.confidence, deserialized.confidence);
    }

    #[test]
    fn test_resume_info_serialization() {
        let resume = ResumeInfo {
            name: Some("张三".to_string()),
            phone: Some("13800138000".to_string()),
            email: Some("zhangsan@example.com".to_string()),
            gender: Some("男".to_string()),
            birth_date: Some("1990-01-01".to_string()),
            education: Some(vec![EducationInfo {
                school: Some("清华大学".to_string()),
                major: Some("计算机科学".to_string()),
                degree: Some("学士".to_string()),
                start_time: Some("2008-09".to_string()),
                end_time: Some("2012-06".to_string()),
            }]),
            work_experience: Some(vec![WorkExperienceInfo {
                company: Some("字节跳动".to_string()),
                position: Some("软件工程师".to_string()),
                start_time: Some("2012-07".to_string()),
                end_time: Some("2020-03".to_string()),
                description: Some("负责后端开发工作".to_string()),
            }]),
            skills: Some(vec![
                "Python".to_string(),
                "Java".to_string(),
                "Rust".to_string(),
            ]),
        };

        let serialized = serde_json::to_string(&resume).unwrap();
        let deserialized: ResumeInfo = serde_json::from_str(&serialized).unwrap();

        assert_eq!(resume.name, deserialized.name);
        assert_eq!(resume.phone, deserialized.phone);
        assert_eq!(resume.email, deserialized.email);
        assert_eq!(
            resume.education.as_ref().unwrap().len(),
            deserialized.education.as_ref().unwrap().len()
        );
        assert_eq!(
            resume.work_experience.as_ref().unwrap().len(),
            deserialized.work_experience.as_ref().unwrap().len()
        );
        assert_eq!(
            resume.skills.as_ref().unwrap().len(),
            deserialized.skills.as_ref().unwrap().len()
        );
    }

    #[test]
    fn test_education_info_serialization() {
        let education = EducationInfo {
            school: Some("北京大学".to_string()),
            major: Some("软件工程".to_string()),
            degree: Some("硕士".to_string()),
            start_time: Some("2012-09".to_string()),
            end_time: Some("2015-06".to_string()),
        };

        let serialized = serde_json::to_string(&education).unwrap();
        let deserialized: EducationInfo = serde_json::from_str(&serialized).unwrap();

        assert_eq!(education.school, deserialized.school);
        assert_eq!(education.major, deserialized.major);
        assert_eq!(education.degree, deserialized.degree);
        assert_eq!(education.start_time, deserialized.start_time);
        assert_eq!(education.end_time, deserialized.end_time);
    }

    #[test]
    fn test_work_experience_info_serialization() {
        let work = WorkExperienceInfo {
            company: Some("腾讯".to_string()),
            position: Some("高级软件工程师".to_string()),
            start_time: Some("2020-04".to_string()),
            end_time: Some("2023-12".to_string()),
            description: Some("负责微信后端架构设计和开发".to_string()),
        };

        let serialized = serde_json::to_string(&work).unwrap();
        let deserialized: WorkExperienceInfo = serde_json::from_str(&serialized).unwrap();

        assert_eq!(work.company, deserialized.company);
        assert_eq!(work.position, deserialized.position);
        assert_eq!(work.start_time, deserialized.start_time);
        assert_eq!(work.end_time, deserialized.end_time);
        assert_eq!(work.description, deserialized.description);
    }

    #[test]
    fn test_id_card_info_serialization() {
        let id_card = IdCardInfo {
            name: Some("李四".to_string()),
            id_number: Some("110101199001011234".to_string()),
            gender: Some("女".to_string()),
            nation: Some("汉族".to_string()),
            birth_date: Some("1990年1月1日".to_string()),
            address: Some("北京市朝阳区".to_string()),
            authority: Some("北京市公安局朝阳分局".to_string()),
            valid_period: Some("2010.01.01-2030.01.01".to_string()),
        };

        let serialized = serde_json::to_string(&id_card).unwrap();
        let deserialized: IdCardInfo = serde_json::from_str(&serialized).unwrap();

        assert_eq!(id_card.name, deserialized.name);
        assert_eq!(id_card.id_number, deserialized.id_number);
        assert_eq!(id_card.gender, deserialized.gender);
        assert_eq!(id_card.nation, deserialized.nation);
        assert_eq!(id_card.birth_date, deserialized.birth_date);
        assert_eq!(id_card.address, deserialized.address);
        assert_eq!(id_card.authority, deserialized.authority);
        assert_eq!(id_card.valid_period, deserialized.valid_period);
    }

    #[test]
    fn test_business_card_info_serialization() {
        let card = BusinessCardInfo {
            name: Some("王五".to_string()),
            company: Some("阿里巴巴".to_string()),
            position: Some("产品经理".to_string()),
            phone: Some("13900139000".to_string()),
            mobile: Some("13900139001".to_string()),
            email: Some("wangwu@alibaba.com".to_string()),
            address: Some("杭州市西湖区".to_string()),
            website: Some("https://www.alibaba.com".to_string()),
        };

        let serialized = serde_json::to_string(&card).unwrap();
        let deserialized: BusinessCardInfo = serde_json::from_str(&serialized).unwrap();

        assert_eq!(card.name, deserialized.name);
        assert_eq!(card.company, deserialized.company);
        assert_eq!(card.position, deserialized.position);
        assert_eq!(card.phone, deserialized.phone);
        assert_eq!(card.mobile, deserialized.mobile);
        assert_eq!(card.email, deserialized.email);
        assert_eq!(card.website, deserialized.website);
    }

    #[test]
    fn test_ocr_result_serialization() {
        let ocr_result = OcrResult {
            text: "识别的文字内容".to_string(),
            text_detection: Some(vec![TextDetection {
                text: "第一行文字".to_string(),
                confidence: Some(0.95),
                bounding_box: Some(BoundingBox {
                    x: 10,
                    y: 20,
                    width: 100,
                    height: 30,
                }),
            }]),
        };

        let serialized = serde_json::to_string(&ocr_result).unwrap();
        let deserialized: OcrResult = serde_json::from_str(&serialized).unwrap();

        assert_eq!(ocr_result.text, deserialized.text);
        assert_eq!(
            ocr_result.text_detection.as_ref().unwrap().len(),
            deserialized.text_detection.as_ref().unwrap().len()
        );
    }

    #[test]
    fn test_speech_recognition_result_serialization() {
        let speech_result = SpeechRecognizeResult {
            recognition_text: "这是语音识别的结果".to_string(),
            confidence: Some(0.92),
        };

        let serialized = serde_json::to_string(&speech_result).unwrap();
        let deserialized: SpeechRecognizeResult = serde_json::from_str(&serialized).unwrap();

        assert_eq!(
            speech_result.recognition_text,
            deserialized.recognition_text
        );
        assert_eq!(speech_result.confidence, deserialized.confidence);
    }

    #[test]
    fn test_translate_result_serialization() {
        let translate_result = TranslateResult {
            translated_text: "Hello World".to_string(),
            detected_language: Some("zh".to_string()),
        };

        let serialized = serde_json::to_string(&translate_result).unwrap();
        let deserialized: TranslateResult = serde_json::from_str(&serialized).unwrap();

        assert_eq!(
            translate_result.translated_text,
            deserialized.translated_text
        );
        assert_eq!(
            translate_result.detected_language,
            deserialized.detected_language
        );
    }

    #[test]
    fn test_models_with_none_values() {
        let resume = ResumeInfo {
            name: None,
            phone: None,
            email: None,
            gender: None,
            birth_date: None,
            education: None,
            work_experience: None,
            skills: None,
        };

        let serialized = serde_json::to_string(&resume).unwrap();
        let deserialized: ResumeInfo = serde_json::from_str(&serialized).unwrap();

        assert!(deserialized.name.is_none());
        assert!(deserialized.phone.is_none());
        assert!(deserialized.email.is_none());
        assert!(deserialized.education.is_none());
        assert!(deserialized.work_experience.is_none());
        assert!(deserialized.skills.is_none());
    }

    #[test]
    fn test_debug_trait_for_models() {
        let request = FileRecognizeRequest {
            file: "test_file".to_string(),
        };

        let debug_string = format!("{:?}", request);
        assert!(debug_string.contains("FileRecognizeRequest"));
        assert!(debug_string.contains("test_file"));
    }

    #[test]
    fn test_driving_license_info_serialization() {
        let license = DrivingLicenseInfo {
            name: Some("赵六".to_string()),
            gender: Some("男".to_string()),
            nationality: Some("中国".to_string()),
            birth_date: Some("1985-05-15".to_string()),
            address: Some("上海市浦东新区".to_string()),
            license_number: Some("310000123456789".to_string()),
            vehicle_type: Some("C1".to_string()),
            valid_period: Some("2020.01.01-2026.01.01".to_string()),
        };

        let serialized = serde_json::to_string(&license).unwrap();
        let deserialized: DrivingLicenseInfo = serde_json::from_str(&serialized).unwrap();

        assert_eq!(license.name, deserialized.name);
        assert_eq!(license.gender, deserialized.gender);
        assert_eq!(license.nationality, deserialized.nationality);
        assert_eq!(license.birth_date, deserialized.birth_date);
        assert_eq!(license.address, deserialized.address);
        assert_eq!(license.license_number, deserialized.license_number);
        assert_eq!(license.vehicle_type, deserialized.vehicle_type);
        assert_eq!(license.valid_period, deserialized.valid_period);
    }

    #[test]
    fn test_bank_card_info_serialization() {
        let bank_card = BankCardInfo {
            bank_name: Some("中国工商银行".to_string()),
            card_number: Some("6222021234567890".to_string()),
            card_type: Some("储蓄卡".to_string()),
            valid_period: Some("12/28".to_string()),
        };

        let serialized = serde_json::to_string(&bank_card).unwrap();
        let deserialized: BankCardInfo = serde_json::from_str(&serialized).unwrap();

        assert_eq!(bank_card.bank_name, deserialized.bank_name);
        assert_eq!(bank_card.card_number, deserialized.card_number);
        assert_eq!(bank_card.card_type, deserialized.card_type);
        assert_eq!(bank_card.valid_period, deserialized.valid_period);
    }

    #[test]
    fn test_business_license_info_serialization() {
        let license = BusinessLicenseInfo {
            credit_code: Some("91310000123456789A".to_string()),
            company_name: Some("上海科技有限公司".to_string()),
            company_type: Some("有限责任公司".to_string()),
            legal_person: Some("张三".to_string()),
            registered_capital: Some("100万元".to_string()),
            establishment_date: Some("2020年1月1日".to_string()),
            business_term: Some("长期".to_string()),
            business_scope: Some("技术开发、技术咨询".to_string()),
            address: Some("上海市浦东新区张江高科".to_string()),
        };

        let serialized = serde_json::to_string(&license).unwrap();
        let deserialized: BusinessLicenseInfo = serde_json::from_str(&serialized).unwrap();

        assert_eq!(license.credit_code, deserialized.credit_code);
        assert_eq!(license.company_name, deserialized.company_name);
        assert_eq!(license.company_type, deserialized.company_type);
        assert_eq!(license.legal_person, deserialized.legal_person);
        assert_eq!(license.registered_capital, deserialized.registered_capital);
        assert_eq!(license.establishment_date, deserialized.establishment_date);
        assert_eq!(license.business_term, deserialized.business_term);
        assert_eq!(license.business_scope, deserialized.business_scope);
        assert_eq!(license.address, deserialized.address);
    }

    #[test]
    fn test_vat_invoice_info_serialization() {
        let invoice = VatInvoiceInfo {
            invoice_code: Some("044001800226".to_string()),
            invoice_number: Some("12345678".to_string()),
            invoice_date: Some("2023年12月31日".to_string()),
            buyer_name: Some("北京科技有限公司".to_string()),
            buyer_tax_id: Some("91110000123456789B".to_string()),
            seller_name: Some("上海服务有限公司".to_string()),
            seller_tax_id: Some("91310000987654321C".to_string()),
            total_amount: Some("1000.00".to_string()),
            total_tax: Some("130.00".to_string()),
            total_amount_with_tax: Some("1130.00".to_string()),
        };

        let serialized = serde_json::to_string(&invoice).unwrap();
        let deserialized: VatInvoiceInfo = serde_json::from_str(&serialized).unwrap();

        assert_eq!(invoice.invoice_code, deserialized.invoice_code);
        assert_eq!(invoice.invoice_number, deserialized.invoice_number);
        assert_eq!(invoice.invoice_date, deserialized.invoice_date);
        assert_eq!(invoice.buyer_name, deserialized.buyer_name);
        assert_eq!(invoice.buyer_tax_id, deserialized.buyer_tax_id);
        assert_eq!(invoice.seller_name, deserialized.seller_name);
        assert_eq!(invoice.seller_tax_id, deserialized.seller_tax_id);
        assert_eq!(invoice.total_amount, deserialized.total_amount);
        assert_eq!(invoice.total_tax, deserialized.total_tax);
        assert_eq!(
            invoice.total_amount_with_tax,
            deserialized.total_amount_with_tax
        );
    }

    #[test]
    fn test_contract_info_serialization() {
        let mut other_fields = serde_json::Map::new();
        other_fields.insert(
            "备注".to_string(),
            serde_json::Value::String("重要合同".to_string()),
        );
        other_fields.insert(
            "版本号".to_string(),
            serde_json::Value::Number(serde_json::Number::from(2)),
        );

        let contract = ContractInfo {
            title: Some("软件开发合同".to_string()),
            party_a: Some("甲方公司".to_string()),
            party_b: Some("乙方公司".to_string()),
            amount: Some("50万元".to_string()),
            sign_date: Some("2023-01-01".to_string()),
            effective_date: Some("2023-01-01".to_string()),
            end_date: Some("2023-12-31".to_string()),
            other_fields: Some(serde_json::Value::Object(other_fields)),
        };

        let serialized = serde_json::to_string(&contract).unwrap();
        let deserialized: ContractInfo = serde_json::from_str(&serialized).unwrap();

        assert_eq!(contract.title, deserialized.title);
        assert_eq!(contract.party_a, deserialized.party_a);
        assert_eq!(contract.party_b, deserialized.party_b);
        assert_eq!(contract.amount, deserialized.amount);
        assert_eq!(contract.sign_date, deserialized.sign_date);
        assert_eq!(contract.effective_date, deserialized.effective_date);
        assert_eq!(contract.end_date, deserialized.end_date);
        assert_eq!(contract.other_fields, deserialized.other_fields);
    }

    #[test]
    fn test_text_detection_and_bounding_box() {
        let detection = TextDetection {
            text: "检测到的文字".to_string(),
            confidence: Some(0.98),
            bounding_box: Some(BoundingBox {
                x: 50,
                y: 100,
                width: 200,
                height: 40,
            }),
        };

        let serialized = serde_json::to_string(&detection).unwrap();
        let deserialized: TextDetection = serde_json::from_str(&serialized).unwrap();

        assert_eq!(detection.text, deserialized.text);
        assert_eq!(detection.confidence, deserialized.confidence);

        if let (Some(orig_box), Some(deser_box)) =
            (&detection.bounding_box, &deserialized.bounding_box)
        {
            assert_eq!(orig_box.x, deser_box.x);
            assert_eq!(orig_box.y, deser_box.y);
            assert_eq!(orig_box.width, deser_box.width);
            assert_eq!(orig_box.height, deser_box.height);
        }
    }

    #[test]
    fn test_speech_recognize_request_serialization() {
        let request = SpeechRecognizeRequest {
            speech: SpeechFile {
                file_token: Some("speech_token_123".to_string()),
                content: None,
            },
            format: Some("wav".to_string()),
            sample_rate: Some(16000),
            language: Some("zh-cn".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: SpeechRecognizeRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(request.format, deserialized.format);
        assert_eq!(request.sample_rate, deserialized.sample_rate);
        assert_eq!(request.language, deserialized.language);
        assert_eq!(request.speech.file_token, deserialized.speech.file_token);
        assert_eq!(request.speech.content, deserialized.speech.content);
    }

    #[test]
    fn test_speech_file_with_base64_content() {
        let speech_file = SpeechFile {
            file_token: None,
            content: Some("UklGRnoGAABXQVZFZm10IBAAAAABAAEAQB8AAEAfAAABAAgAZGF0YQoGAACBhYqFbF1fdJivrJBhNjVgodDbq2EcBj+a2/LDciUFLIHO8tiJNwgZaLvt559NEAxQp+PwtmMcBjiR1+u+fykEMm3J7+KJQQ0PVqzn77BdGAg+ltryxnkpBSl+zPLZiTYIG2m98OacTgwOUarm7bdKDAk9jNXqwnkpBSt9y/LZgzcIG2i+8OWOTAoOUKnn7bY=".to_string()),
        };

        let serialized = serde_json::to_string(&speech_file).unwrap();
        let deserialized: SpeechFile = serde_json::from_str(&serialized).unwrap();

        assert_eq!(speech_file.file_token, deserialized.file_token);
        assert_eq!(speech_file.content, deserialized.content);
        assert!(!serialized.contains("file_token"));
        assert!(serialized.contains("content"));
    }

    #[test]
    fn test_stream_speech_request_serialization() {
        let request = StreamSpeechRequest {
            stream: "audio_stream_data_chunk_1".to_string(),
            format: Some("pcm".to_string()),
            sample_rate: Some(8000),
            language: Some("en-us".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: StreamSpeechRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(request.stream, deserialized.stream);
        assert_eq!(request.format, deserialized.format);
        assert_eq!(request.sample_rate, deserialized.sample_rate);
        assert_eq!(request.language, deserialized.language);
    }

    #[test]
    fn test_language_detect_request_and_result() {
        let request = LanguageDetectRequest {
            text: "Hello, how are you today?".to_string(),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: LanguageDetectRequest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(request.text, deserialized.text);

        let result = LanguageDetectResult {
            language: "en".to_string(),
            confidence: Some(0.99),
        };

        let serialized = serde_json::to_string(&result).unwrap();
        let deserialized: LanguageDetectResult = serde_json::from_str(&serialized).unwrap();
        assert_eq!(result.language, deserialized.language);
        assert_eq!(result.confidence, deserialized.confidence);
    }

    #[test]
    fn test_translate_request_serialization() {
        let request = TranslateRequest {
            source_language: Some("zh".to_string()),
            target_language: "en".to_string(),
            text: "你好，世界！".to_string(),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: TranslateRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(request.source_language, deserialized.source_language);
        assert_eq!(request.target_language, deserialized.target_language);
        assert_eq!(request.text, deserialized.text);
    }

    #[test]
    fn test_translate_request_without_source_language() {
        let request = TranslateRequest {
            source_language: None,
            target_language: "fr".to_string(),
            text: "Auto-detect source language".to_string(),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(!serialized.contains("source_language"));

        let deserialized: TranslateRequest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(request.source_language, deserialized.source_language);
        assert_eq!(request.target_language, deserialized.target_language);
        assert_eq!(request.text, deserialized.text);
    }

    #[test]
    fn test_models_with_all_none_values() {
        let id_card = IdCardInfo {
            name: None,
            gender: None,
            nation: None,
            birth_date: None,
            address: None,
            id_number: None,
            authority: None,
            valid_period: None,
        };

        let serialized = serde_json::to_string(&id_card).unwrap();
        assert_eq!(serialized, "{}");

        let deserialized: IdCardInfo = serde_json::from_str(&serialized).unwrap();
        assert!(deserialized.name.is_none());
        assert!(deserialized.gender.is_none());
        assert!(deserialized.nation.is_none());
        assert!(deserialized.birth_date.is_none());
        assert!(deserialized.address.is_none());
        assert!(deserialized.id_number.is_none());
        assert!(deserialized.authority.is_none());
        assert!(deserialized.valid_period.is_none());

        let bank_card = BankCardInfo {
            bank_name: None,
            card_number: None,
            card_type: None,
            valid_period: None,
        };

        let serialized = serde_json::to_string(&bank_card).unwrap();
        assert_eq!(serialized, "{}");

        let deserialized: BankCardInfo = serde_json::from_str(&serialized).unwrap();
        assert!(deserialized.bank_name.is_none());
        assert!(deserialized.card_number.is_none());
        assert!(deserialized.card_type.is_none());
        assert!(deserialized.valid_period.is_none());
    }

    #[test]
    fn test_recognize_response_with_none_confidence() {
        let response: RecognizeResponse<ResumeInfo> = RecognizeResponse {
            data: ResumeInfo {
                name: Some("测试用户".to_string()),
                phone: None,
                email: None,
                gender: None,
                birth_date: None,
                education: None,
                work_experience: None,
                skills: None,
            },
            confidence: None,
        };

        let serialized = serde_json::to_string(&response).unwrap();
        assert!(!serialized.contains("confidence"));

        let deserialized: RecognizeResponse<ResumeInfo> =
            serde_json::from_str(&serialized).unwrap();
        assert_eq!(response.confidence, deserialized.confidence);
        assert_eq!(response.data.name, deserialized.data.name);
    }

    #[test]
    fn test_complex_nested_structures() {
        let complex_resume = ResumeInfo {
            name: Some("复杂测试用户".to_string()),
            phone: Some("13000000000".to_string()),
            email: Some("complex@test.com".to_string()),
            gender: Some("未知".to_string()),
            birth_date: Some("1990-12-31".to_string()),
            education: Some(vec![
                EducationInfo {
                    school: Some("第一大学".to_string()),
                    major: Some("计算机科学".to_string()),
                    degree: Some("学士".to_string()),
                    start_time: Some("2008-09".to_string()),
                    end_time: Some("2012-06".to_string()),
                },
                EducationInfo {
                    school: Some("第二大学".to_string()),
                    major: Some("软件工程".to_string()),
                    degree: Some("硕士".to_string()),
                    start_time: Some("2012-09".to_string()),
                    end_time: Some("2014-06".to_string()),
                },
            ]),
            work_experience: Some(vec![
                WorkExperienceInfo {
                    company: Some("第一家公司".to_string()),
                    position: Some("初级开发工程师".to_string()),
                    start_time: Some("2014-07".to_string()),
                    end_time: Some("2017-03".to_string()),
                    description: Some("负责前端开发".to_string()),
                },
                WorkExperienceInfo {
                    company: Some("第二家公司".to_string()),
                    position: Some("高级开发工程师".to_string()),
                    start_time: Some("2017-04".to_string()),
                    end_time: None,
                    description: None,
                },
            ]),
            skills: Some(vec![
                "Rust".to_string(),
                "Python".to_string(),
                "JavaScript".to_string(),
                "Docker".to_string(),
                "Kubernetes".to_string(),
            ]),
        };

        let serialized = serde_json::to_string(&complex_resume).unwrap();
        let deserialized: ResumeInfo = serde_json::from_str(&serialized).unwrap();

        assert_eq!(complex_resume.name, deserialized.name);
        assert_eq!(complex_resume.phone, deserialized.phone);
        assert_eq!(complex_resume.email, deserialized.email);
        assert_eq!(complex_resume.gender, deserialized.gender);
        assert_eq!(complex_resume.birth_date, deserialized.birth_date);

        // Test education array length and content
        assert_eq!(
            complex_resume.education.as_ref().unwrap().len(),
            deserialized.education.as_ref().unwrap().len()
        );
        assert_eq!(
            complex_resume.education.as_ref().unwrap()[0].school,
            deserialized.education.as_ref().unwrap()[0].school
        );
        assert_eq!(
            complex_resume.education.as_ref().unwrap()[1].degree,
            deserialized.education.as_ref().unwrap()[1].degree
        );

        // Test work experience array length and content
        assert_eq!(
            complex_resume.work_experience.as_ref().unwrap().len(),
            deserialized.work_experience.as_ref().unwrap().len()
        );
        assert_eq!(
            complex_resume.work_experience.as_ref().unwrap()[0].company,
            deserialized.work_experience.as_ref().unwrap()[0].company
        );
        assert_eq!(
            complex_resume.work_experience.as_ref().unwrap()[1].end_time,
            deserialized.work_experience.as_ref().unwrap()[1].end_time
        );

        // Test skills array length and content
        assert_eq!(
            complex_resume.skills.as_ref().unwrap().len(),
            deserialized.skills.as_ref().unwrap().len()
        );
        assert_eq!(
            complex_resume.skills.as_ref().unwrap()[0],
            deserialized.skills.as_ref().unwrap()[0]
        );
    }
}
