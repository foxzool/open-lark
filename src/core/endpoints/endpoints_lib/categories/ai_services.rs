//! AI服务服务端点

/// AI服务服务端点
pub struct AiServices;

impl AiServices {
    /// 草稿管理
    pub const LINGO_CLASSIFICATION_LIST: &'static str = "/open-apis/lingo/v1/classifications";
    /// / 更新草稿 (需要使用 EndpointBuilder::replace_param 替换 {draft_id})
    pub const LINGO_DRAFT_CREATE: &'static str = "/open-apis/lingo/v1/drafts";
    /// 词条管理
    pub const LINGO_DRAFT_UPDATE: &'static str = "/open-apis/lingo/v1/drafts/{draft_id}";
    /// / 获取词条详情 (需要使用 EndpointBuilder::replace_param 替换 {entity_id})
    pub const LINGO_ENTITY_CREATE: &'static str = "/open-apis/lingo/v1/entities";
    /// / 更新词条 (需要使用 EndpointBuilder::replace_param 替换 {entity_id})
    pub const LINGO_ENTITY_GET: &'static str = "/open-apis/lingo/v1/entities/{entity_id}";
    /// / 搜索词条
    pub const LINGO_ENTITY_UPDATE: &'static str = "/open-apis/lingo/v1/entities/{entity_id}";
    /// / 词条匹配
    pub const LINGO_ENTITY_SEARCH: &'static str = "/open-apis/lingo/v1/entities/search";
    /// / 提取可能的词条
    pub const LINGO_ENTITY_MATCH: &'static str = "/open-apis/lingo/v1/entities/match";
    /// 文件管理
    pub const LINGO_ENTITY_HIGHLIGHT: &'static str = "/open-apis/lingo/v1/entities/highlight";
    /// ==================== 租户管理服务端点 ====================
    pub const LINGO_REPO_LIST: &'static str = "/open-apis/lingo/v1/repos";
    /// / 身份证识别
    pub const DOCUMENT_AI_RESUME_PARSE: &'static str = "/open-apis/document_ai/v1/resume/parse";
    /// / 驾驶证识别
    pub const DOCUMENT_AI_ID_CARD_RECOGNIZE: &'static str = "/open-apis/document_ai/v1/id_card/recognize";
    /// / 银行卡识别
    pub const DOCUMENT_AI_DRIVING_LICENSE_RECOGNIZE: &'static str = "/open-apis/document_ai/v1/driving_license/recognize";
    /// / 营业执照识别
    pub const DOCUMENT_AI_BANK_CARD_RECOGNIZE: &'static str = "/open-apis/document_ai/v1/bank_card/recognize";
    /// / 增值税发票识别
    pub const DOCUMENT_AI_BUSINESS_LICENSE_RECOGNIZE: &'static str = "/open-apis/document_ai/v1/business_license/recognize";
    /// / 合同字段提取
    pub const DOCUMENT_AI_VAT_INVOICE_RECOGNIZE: &'static str = "/open-apis/document_ai/v1/vat_invoice/recognize";
    /// / 名片识别
    pub const DOCUMENT_AI_CONTRACT_FIELD_EXTRACTION: &'static str = "/open-apis/document_ai/v1/contract/field_extraction";
    /// / 机动车发票识别
    pub const DOCUMENT_AI_BUSINESS_CARD_RECOGNIZE: &'static str = "/open-apis/document_ai/v1/business_card/recognize";
    /// / 健康证识别
    pub const DOCUMENT_AI_VEHICLE_INVOICE_RECOGNIZE: &'static str = "/open-apis/document_ai/v1/vehicle_invoice/recognize";
    /// / 港澳居民来往内地通行证识别
    pub const DOCUMENT_AI_HEALTH_CERTIFICATE_RECOGNIZE: &'static str = "/open-apis/document_ai/v1/health_certificate/recognize";
    /// / 台湾居民来往大陆通行证识别
    pub const DOCUMENT_AI_HKM_MAINLAND_TRAVEL_PERMIT_RECOGNIZE: &'static str = "/open-apis/document_ai/v1/hkm_mainland_travel_permit/recognize";
    /// / 中国护照识别
    pub const DOCUMENT_AI_TW_MAINLAND_TRAVEL_PERMIT_RECOGNIZE: &'static str = "/open-apis/document_ai/v1/tw_mainland_travel_permit/recognize";
    /// / 行驶证识别
    pub const DOCUMENT_AI_CHINESE_PASSPORT_RECOGNIZE: &'static str = "/open-apis/document_ai/v1/chinese_passport/recognize";
    /// / 火车票识别
    pub const DOCUMENT_AI_VEHICLE_LICENSE_RECOGNIZE: &'static str = "/open-apis/document_ai/v1/vehicle_license/recognize";
    /// / 出租车发票识别
    pub const DOCUMENT_AI_TRAIN_INVOICE_RECOGNIZE: &'static str = "/open-apis/document_ai/v1/train_invoice/recognize";
    /// / 食品生产许可证识别
    pub const DOCUMENT_AI_TAXI_INVOICE_RECOGNIZE: &'static str = "/open-apis/document_ai/v1/taxi_invoice/recognize";
    /// / 食品经营许可证识别
    pub const DOCUMENT_AI_FOOD_PRODUCE_LICENSE_RECOGNIZE: &'static str = "/open-apis/document_ai/v1/food_produce_license/recognize";
    /// ==================== AI Services AI服务相关端点 ====================
    pub const DOCUMENT_AI_FOOD_MANAGE_LICENSE_RECOGNIZE: &'static str = "/open-apis/document_ai/v1/food_manage_license/recognize";
}
