//! Document AI V1 模块
//!
//! 提供文档AI识别服务的 v1 版本API。

pub mod recognize;

// 重导出识别模块的主要类型
pub use recognize::{
    create_bank_card_recognize, create_bank_card_recognize_with_options,
    create_business_license_recognize, create_business_license_recognize_with_options,
    create_id_card_recognize, create_id_card_recognize_with_options, create_resume_parse,
    create_resume_parse_with_options, create_vat_invoice_recognize,
    create_vat_invoice_recognize_with_options, BankCardRecognizeBody, BankCardRecognizeRequest,
    BankCardRecognizeRequestBuilder, BankCardRecognizeResponse, BusinessLicenseRecognizeBody,
    BusinessLicenseRecognizeRequest, BusinessLicenseRecognizeRequestBuilder,
    BusinessLicenseRecognizeResponse, IdCardRecognizeBody, IdCardRecognizeRequest,
    IdCardRecognizeRequestBuilder, IdCardRecognizeResponse, ResumeParseBody, ResumeParseRequest,
    ResumeParseRequestBuilder, ResumeParseResponse, VatInvoiceRecognizeBody,
    VatInvoiceRecognizeRequest, VatInvoiceRecognizeRequestBuilder, VatInvoiceRecognizeResponse,
};
