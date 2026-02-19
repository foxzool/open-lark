//! Document AI 根模块
//!
//! 提供文档AI识别服务的完整功能。

pub mod document_ai;

// 重导出 document_ai 模块的主要类型
pub use document_ai::{
    create_bank_card_recognize, create_bank_card_recognize_with_options,
    create_business_license_recognize, create_business_license_recognize_with_options,
    create_id_card_recognize, create_id_card_recognize_with_options, create_resume_parse,
    create_resume_parse_with_options, create_vat_invoice_recognize,
    create_vat_invoice_recognize_with_options, v1, BankCardRecognizeBody,
    BankCardRecognizeRequest, BankCardRecognizeRequestBuilder, BankCardRecognizeResponse,
    BusinessLicenseRecognizeBody, BusinessLicenseRecognizeRequest,
    BusinessLicenseRecognizeRequestBuilder, BusinessLicenseRecognizeResponse, IdCardRecognizeBody,
    IdCardRecognizeRequest, IdCardRecognizeRequestBuilder, IdCardRecognizeResponse, recognize,
    ResumeParseBody, ResumeParseRequest, ResumeParseRequestBuilder, ResumeParseResponse,
    VatInvoiceRecognizeBody, VatInvoiceRecognizeRequest, VatInvoiceRecognizeRequestBuilder,
    VatInvoiceRecognizeResponse,
};
