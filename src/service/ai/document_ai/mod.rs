use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::Endpoints,
        http::Transport,
        req_option::RequestOption,
        trait_system::Service,
        SDKResult,
    },
    service::ai::models::{
        BankCardInfo, BusinessCardInfo, BusinessLicenseInfo, ContractInfo, DrivingLicenseInfo,
        FileRecognizeRequest, IdCardInfo, RecognizeResponse, ResumeInfo, VatInvoiceInfo,
    },
};

/// 智能文档处理服务
pub struct DocumentAiService {
    pub config: Config,
}

/// 简历解析响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ResumeParseResponse {
    /// 简历信息
    #[serde(flatten)]
    pub resume: RecognizeResponse<ResumeInfo>,
}

impl ApiResponseTrait for ResumeParseResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 身份证识别响应
#[derive(Debug, Serialize, Deserialize)]
pub struct IdCardRecognizeResponse {
    /// 身份证信息
    #[serde(flatten)]
    pub id_card: RecognizeResponse<IdCardInfo>,
}

impl ApiResponseTrait for IdCardRecognizeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 驾驶证识别响应
#[derive(Debug, Serialize, Deserialize)]
pub struct DrivingLicenseRecognizeResponse {
    /// 驾驶证信息
    #[serde(flatten)]
    pub driving_license: RecognizeResponse<DrivingLicenseInfo>,
}

impl ApiResponseTrait for DrivingLicenseRecognizeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 银行卡识别响应
#[derive(Debug, Serialize, Deserialize)]
pub struct BankCardRecognizeResponse {
    /// 银行卡信息
    #[serde(flatten)]
    pub bank_card: RecognizeResponse<BankCardInfo>,
}

impl ApiResponseTrait for BankCardRecognizeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 营业执照识别响应
#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessLicenseRecognizeResponse {
    /// 营业执照信息
    #[serde(flatten)]
    pub business_license: RecognizeResponse<BusinessLicenseInfo>,
}

impl ApiResponseTrait for BusinessLicenseRecognizeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 增值税发票识别响应
#[derive(Debug, Serialize, Deserialize)]
pub struct VatInvoiceRecognizeResponse {
    /// 增值税发票信息
    #[serde(flatten)]
    pub vat_invoice: RecognizeResponse<VatInvoiceInfo>,
}

impl ApiResponseTrait for VatInvoiceRecognizeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 合同字段提取响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ContractFieldExtractionResponse {
    /// 合同信息
    #[serde(flatten)]
    pub contract: RecognizeResponse<ContractInfo>,
}

impl ApiResponseTrait for ContractFieldExtractionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 名片识别响应
#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessCardRecognizeResponse {
    /// 名片信息
    #[serde(flatten)]
    pub business_card: RecognizeResponse<BusinessCardInfo>,
}

impl ApiResponseTrait for BusinessCardRecognizeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 通用证件识别响应
#[derive(Debug, Serialize, Deserialize)]
pub struct GenericDocumentResponse {
    /// 识别结果
    pub data: serde_json::Value,
    /// 置信度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
}

impl ApiResponseTrait for GenericDocumentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl DocumentAiService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 识别文件中的简历信息
    ///
    /// 该接口用于解析简历文件，提取关键信息如姓名、联系方式、教育经历、工作经历等。
    ///
    /// # 参数
    ///
    /// - `request`: 文件识别请求参数
    /// - `option`: 可选的请求配置
    pub async fn parse_resume(
        &self,
        request: FileRecognizeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ResumeParseResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: Endpoints::DOCUMENT_AI_RESUME_PARSE.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 识别文件中的身份证
    ///
    /// 该接口用于识别身份证图片，提取身份证上的关键信息。
    ///
    /// # 参数
    ///
    /// - `request`: 文件识别请求参数
    /// - `option`: 可选的请求配置
    pub async fn recognize_id_card(
        &self,
        request: FileRecognizeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<IdCardRecognizeResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: Endpoints::DOCUMENT_AI_ID_CARD_RECOGNIZE.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 识别文件中的驾驶证
    ///
    /// 该接口用于识别驾驶证图片，提取驾驶证上的关键信息。
    ///
    /// # 参数
    ///
    /// - `request`: 文件识别请求参数
    /// - `option`: 可选的请求配置
    pub async fn recognize_driving_license(
        &self,
        request: FileRecognizeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<DrivingLicenseRecognizeResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: Endpoints::DOCUMENT_AI_DRIVING_LICENSE_RECOGNIZE.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 识别文件中的银行卡
    ///
    /// 该接口用于识别银行卡图片，提取银行卡上的关键信息。
    ///
    /// # 参数
    ///
    /// - `request`: 文件识别请求参数
    /// - `option`: 可选的请求配置
    pub async fn recognize_bank_card(
        &self,
        request: FileRecognizeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<BankCardRecognizeResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: Endpoints::DOCUMENT_AI_BANK_CARD_RECOGNIZE.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 识别文件中的营业执照
    ///
    /// 该接口用于识别营业执照图片，提取营业执照上的关键信息。
    ///
    /// # 参数
    ///
    /// - `request`: 文件识别请求参数
    /// - `option`: 可选的请求配置
    pub async fn recognize_business_license(
        &self,
        request: FileRecognizeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<BusinessLicenseRecognizeResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: Endpoints::DOCUMENT_AI_BUSINESS_LICENSE_RECOGNIZE.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 识别文件中的增值税发票
    ///
    /// 该接口用于识别增值税发票图片，提取发票上的关键信息。
    ///
    /// # 参数
    ///
    /// - `request`: 文件识别请求参数
    /// - `option`: 可选的请求配置
    pub async fn recognize_vat_invoice(
        &self,
        request: FileRecognizeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<VatInvoiceRecognizeResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: Endpoints::DOCUMENT_AI_VAT_INVOICE_RECOGNIZE.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 提取文件中的合同字段
    ///
    /// 该接口用于识别合同文档，提取合同中的关键字段信息。
    ///
    /// # 参数
    ///
    /// - `request`: 文件识别请求参数
    /// - `option`: 可选的请求配置
    pub async fn extract_contract_fields(
        &self,
        request: FileRecognizeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ContractFieldExtractionResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: Endpoints::DOCUMENT_AI_CONTRACT_FIELD_EXTRACTION.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 识别文件中的名片
    ///
    /// 该接口用于识别名片图片，提取名片上的关键信息。
    ///
    /// # 参数
    ///
    /// - `request`: 文件识别请求参数
    /// - `option`: 可选的请求配置
    pub async fn recognize_business_card(
        &self,
        request: FileRecognizeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<BusinessCardRecognizeResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: Endpoints::DOCUMENT_AI_BUSINESS_CARD_RECOGNIZE.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 识别文件中的机动车发票
    ///
    /// 该接口用于识别机动车发票图片。
    ///
    /// # 参数
    ///
    /// - `request`: 文件识别请求参数
    /// - `option`: 可选的请求配置
    pub async fn recognize_vehicle_invoice(
        &self,
        request: FileRecognizeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GenericDocumentResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: Endpoints::DOCUMENT_AI_VEHICLE_INVOICE_RECOGNIZE.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 识别文件中的健康证
    ///
    /// 该接口用于识别健康证图片。
    ///
    /// # 参数
    ///
    /// - `request`: 文件识别请求参数
    /// - `option`: 可选的请求配置
    pub async fn recognize_health_certificate(
        &self,
        request: FileRecognizeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GenericDocumentResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: Endpoints::DOCUMENT_AI_HEALTH_CERTIFICATE_RECOGNIZE.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 识别文件中的港澳居民来往内地通行证
    ///
    /// 该接口用于识别港澳居民来往内地通行证图片。
    ///
    /// # 参数
    ///
    /// - `request`: 文件识别请求参数
    /// - `option`: 可选的请求配置
    pub async fn recognize_hkm_mainland_travel_permit(
        &self,
        request: FileRecognizeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GenericDocumentResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: Endpoints::DOCUMENT_AI_HKM_MAINLAND_TRAVEL_PERMIT_RECOGNIZE.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 识别文件中的台湾居民来往大陆通行证
    ///
    /// 该接口用于识别台湾居民来往大陆通行证图片。
    ///
    /// # 参数
    ///
    /// - `request`: 文件识别请求参数
    /// - `option`: 可选的请求配置
    pub async fn recognize_tw_mainland_travel_permit(
        &self,
        request: FileRecognizeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GenericDocumentResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: Endpoints::DOCUMENT_AI_TW_MAINLAND_TRAVEL_PERMIT_RECOGNIZE.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 识别文件中的中国护照
    ///
    /// 该接口用于识别中国护照图片。
    ///
    /// # 参数
    ///
    /// - `request`: 文件识别请求参数
    /// - `option`: 可选的请求配置
    pub async fn recognize_chinese_passport(
        &self,
        request: FileRecognizeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GenericDocumentResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: Endpoints::DOCUMENT_AI_CHINESE_PASSPORT_RECOGNIZE.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 识别文件中的行驶证
    ///
    /// 该接口用于识别行驶证图片。
    ///
    /// # 参数
    ///
    /// - `request`: 文件识别请求参数
    /// - `option`: 可选的请求配置
    pub async fn recognize_vehicle_license(
        &self,
        request: FileRecognizeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GenericDocumentResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: Endpoints::DOCUMENT_AI_VEHICLE_LICENSE_RECOGNIZE.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 识别文件中的火车票
    ///
    /// 该接口用于识别火车票图片。
    ///
    /// # 参数
    ///
    /// - `request`: 文件识别请求参数
    /// - `option`: 可选的请求配置
    pub async fn recognize_train_invoice(
        &self,
        request: FileRecognizeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GenericDocumentResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: Endpoints::DOCUMENT_AI_TRAIN_INVOICE_RECOGNIZE.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 识别文件中的出租车发票
    ///
    /// 该接口用于识别出租车发票图片。
    ///
    /// # 参数
    ///
    /// - `request`: 文件识别请求参数
    /// - `option`: 可选的请求配置
    pub async fn recognize_taxi_invoice(
        &self,
        request: FileRecognizeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GenericDocumentResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: Endpoints::DOCUMENT_AI_TAXI_INVOICE_RECOGNIZE.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 识别文件中的食品生产许可证
    ///
    /// 该接口用于识别食品生产许可证图片。
    ///
    /// # 参数
    ///
    /// - `request`: 文件识别请求参数
    /// - `option`: 可选的请求配置
    pub async fn recognize_food_produce_license(
        &self,
        request: FileRecognizeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GenericDocumentResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: Endpoints::DOCUMENT_AI_FOOD_PRODUCE_LICENSE_RECOGNIZE.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 识别文件中的食品经营许可证
    ///
    /// 该接口用于识别食品经营许可证图片。
    ///
    /// # 参数
    ///
    /// - `request`: 文件识别请求参数
    /// - `option`: 可选的请求配置
    pub async fn recognize_food_manage_license(
        &self,
        request: FileRecognizeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GenericDocumentResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: Endpoints::DOCUMENT_AI_FOOD_MANAGE_LICENSE_RECOGNIZE.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

impl Service for DocumentAiService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "document_ai"
    }

    fn service_version() -> &'static str {
        "v1"
    }
}
