//! Document AI 模块
//!
//! 提供文档AI识别服务。

pub mod v1;

// 重导出 v1 模块的主要类型
pub use v1::{
    create_bank_card_recognize, create_bank_card_recognize_with_options,
    create_business_license_recognize, create_business_license_recognize_with_options,
    create_id_card_recognize, create_id_card_recognize_with_options, create_resume_parse,
    create_resume_parse_with_options, create_vat_invoice_recognize,
    create_vat_invoice_recognize_with_options, recognize, BankCardRecognizeBody,
    BankCardRecognizeRequest, BankCardRecognizeRequestBuilder, BankCardRecognizeResponse,
    BusinessLicenseRecognizeBody, BusinessLicenseRecognizeRequest,
    BusinessLicenseRecognizeRequestBuilder, BusinessLicenseRecognizeResponse, IdCardRecognizeBody,
    IdCardRecognizeRequest, IdCardRecognizeRequestBuilder, IdCardRecognizeResponse,
    ResumeParseBody, ResumeParseRequest, ResumeParseRequestBuilder, ResumeParseResponse,
    VatInvoiceRecognizeBody, VatInvoiceRecognizeRequest, VatInvoiceRecognizeRequestBuilder,
    VatInvoiceRecognizeResponse,
};
