//! Document AI 识别模块
//!
//! 提供文档AI识别相关的API，包括简历解析、身份证识别、银行卡识别、
//! 营业执照识别和增值税发票识别等核心功能。

pub mod bank_card_recognize;
pub mod business_license_recognize;
pub mod id_card_recognize;
pub mod resume_parse;
pub mod vat_invoice_recognize;

// 重导出主要类型，方便直接使用
pub use bank_card_recognize::{
    BankCardRecognizeBody, BankCardRecognizeRequest, BankCardRecognizeRequestBuilder,
    BankCardRecognizeResponse, ParsingResult as BankCardParsingResult,
};
pub use business_license_recognize::{
    BusinessLicenseRecognizeBody, BusinessLicenseRecognizeRequest,
    BusinessLicenseRecognizeRequestBuilder, BusinessLicenseRecognizeResponse,
    ParsingResult as BusinessLicenseParsingResult,
};
pub use id_card_recognize::{
    IdCardRecognizeBody, IdCardRecognizeRequest, IdCardRecognizeRequestBuilder,
    IdCardRecognizeResponse, ParsingResult as IdCardParsingResult,
};
pub use resume_parse::{
    EducationExperience, ParsingResult as ResumeParsingResult, ProjectExperience,
    ResumeParseBody, ResumeParseRequest, ResumeParseRequestBuilder, ResumeParseResponse,
    WorkExperience,
};
pub use vat_invoice_recognize::{
    ParsingResult as VatInvoiceParsingResult, VatInvoiceRecognizeBody,
    VatInvoiceRecognizeRequest, VatInvoiceRecognizeRequestBuilder, VatInvoiceRecognizeResponse,
};

/// 创建简历解析请求构建器
pub use resume_parse::create as create_resume_parse;

/// 创建身份证识别请求构建器
pub use id_card_recognize::create as create_id_card_recognize;

/// 创建银行卡识别请求构建器
pub use bank_card_recognize::create as create_bank_card_recognize;

/// 创建营业执照识别请求构建器
pub use business_license_recognize::create as create_business_license_recognize;

/// 创建增值税发票识别请求构建器
pub use vat_invoice_recognize::create as create_vat_invoice_recognize;

/// 创建简历解析请求构建器（支持自定义选项）
pub use resume_parse::create_with_options as create_resume_parse_with_options;

/// 创建身份证识别请求构建器（支持自定义选项）
pub use id_card_recognize::create_with_options as create_id_card_recognize_with_options;

/// 创建银行卡识别请求构建器（支持自定义选项）
pub use bank_card_recognize::create_with_options as create_bank_card_recognize_with_options;

/// 创建营业执照识别请求构建器（支持自定义选项）
pub use business_license_recognize::create_with_options
    as create_business_license_recognize_with_options;

/// 创建增值税发票识别请求构建器（支持自定义选项）
pub use vat_invoice_recognize::create_with_options as create_vat_invoice_recognize_with_options;
