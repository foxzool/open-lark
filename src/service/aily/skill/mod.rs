use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::{EndpointBuilder, Endpoints},
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::aily::models::{
        PageResponse, Skill, SkillExecution, SkillGetRequest, SkillListRequest, SkillStartRequest,
    },
};

/// 技能管理服务
pub struct SkillService {
    pub config: Config,
}

/// 技能调用响应
#[derive(Debug, Serialize, Deserialize)]
pub struct SkillStartResponse {
    /// 技能执行信息
    #[serde(flatten)]
    pub execution: SkillExecution,
}

impl ApiResponseTrait for SkillStartResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 技能信息查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct SkillGetResponse {
    /// 技能信息
    #[serde(flatten)]
    pub skill: Skill,
}

impl ApiResponseTrait for SkillGetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 技能列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct SkillListResponse {
    /// 分页响应数据
    #[serde(flatten)]
    pub page_response: PageResponse<Skill>,
}

impl ApiResponseTrait for SkillListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl SkillService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 调用技能
    ///
    /// 该接口用于调用指定的智能伙伴技能。
    ///
    /// # 参数
    ///
    /// - `request`: 技能调用请求参数
    /// - `option`: 可选的请求配置
    pub async fn start_skill(
        &self,
        request: SkillStartRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<SkillStartResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(
                Endpoints::AILY_V1_SKILL_START,
                "skill_id",
                &request.skill_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&serde_json::json!({
                "app_id": request.app_id,
                "input": request.input,
                "session_id": request.session_id
            }))?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取技能信息
    ///
    /// 该接口用于获取指定的技能详细信息。
    ///
    /// # 参数
    ///
    /// - `request`: 技能信息查询请求参数
    /// - `option`: 可选的请求配置
    pub async fn get_skill(
        &self,
        request: SkillGetRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<SkillGetResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                Endpoints::AILY_V1_SKILL_GET,
                "skill_id",
                &request.skill_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        api_req.query_params.insert("app_id", request.app_id);

        Transport::request(api_req, &self.config, option).await
    }

    /// 查询技能列表
    ///
    /// 该接口用于查询智能伙伴的技能列表。
    ///
    /// # 参数
    ///
    /// - `request`: 技能列表查询请求参数
    /// - `option`: 可选的请求配置
    pub async fn list_skills(
        &self,
        request: SkillListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<SkillListResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: Endpoints::AILY_V1_SKILLS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        api_req.query_params.insert("app_id", request.app_id);

        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }
        if let Some(page_token) = request.page_token {
            api_req.query_params.insert("page_token", page_token);
        }

        Transport::request(api_req, &self.config, option).await
    }
}
