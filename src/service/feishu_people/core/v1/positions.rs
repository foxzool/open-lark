//! Positions v1 - 职位管理API
//!
//! 提供完整的职位生命周期管理功能，包括：
//! - 职位信息的增删改查操作
//! - 批量职位数据处理和同步
//! - 职位搜索和高级筛选功能
//! - 职位序列和级别体系管理
//! - 职位统计分析和报表
//! - 职位人员配置和管理
//!
//! # 示例
//!
//! ```rust,no_run
//! use open_lark::prelude::*;
//! use open_lark::service::feishu_people::core::v1::positions::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = LarkClient::builder()
//!         .app_id("your_app_id")
//!         .app_secret("your_app_secret")
//!         .build()?;
//!
//!     // 获取职位详情
//!     let response = client.feishu_people.core.v1.positions
//!         .get_position_builder("position_001")
//!         .user_id_type("open_id")
//!         .execute(&client.feishu_people.core.v1.positions)
//!         .await?;
//!
//!     println!("职位名称: {}", response.data.name);
//!
//!     // 创建新职位
//!     let position = PositionCreateData {
//!         name: "高级工程师".to_string(),
//!         en_name: Some("Senior Engineer".to_string()),
//!         department_id: Some("dept_001".to_string()),
//!         headcount: Some(5),
//!         ..Default::default()
//!     };
//!
//!     let create_response = client.feishu_people.core.v1.positions
//!         .create_position_builder()
//!         .position_data(position)
//!         .user_id_type("open_id")
//!         .execute(&client.feishu_people.core.v1.positions)
//!         .await?;
//!
//!     println!("职位创建成功，ID: {}", create_response.data.position_id);
//!
//!     // 搜索职位
//!     let search_response = client.feishu_people.core.v1.positions
//!         .search_positions_builder()
//!         .query("工程师")
//!         .page_size(20)
//!         .user_id_type("open_id")
//!         .execute(&client.feishu_people.core.v1.positions)
//!         .await?;
//!
//!     println!("搜索到 {} 个职位", search_response.data.items.len());
//!
//!     Ok(())
//! }
//! ```

use crate::core::{
use crate::core::{SDKResult, api_resp::{ApiResponseTrait, BaseResponse}},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
};

// Use open_lark_core's error type for compatibility with async traits
use open_lark_core::core::LarkAPIError;
pub type SDKResult<T> = Result<T, LarkAPIError>;
use open_lark_core::core::api_req::ApiRequest; // trait_system::ExecutableBuilder temporarily disabled
use serde::{Deserialize, Serialize};

/// 职位管理服务
///
/// 提供完整的职位生命周期管理功能，包括职位的创建、更新、删除、查询等操作。
/// 支持企业级的职位管理需求，包括批量操作、高级搜索和统计分析功能。
///
/// # 核心功能
///
/// - **职位CRUD操作**: 创建、查询、更新、删除职位信息
/// - **批量处理**: 支持批量获取和操作职位数据
/// - **高级搜索**: 基于多种条件的职位搜索和筛选
/// - **统计分析**: 职位配置、空缺率等统计信息
/// - **序列管理**: 职位序列和级别的体系化管理
/// - **人员配置**: 职位人员配置和历史记录
///
/// # 使用示例
///
/// ```rust
/// use open_lark::prelude::*;
/// use open_lark::service::feishu_people::core::v1::positions::PositionsService;
///
/// let config = Config::new("app_id", "app_secret");
/// let service = PositionsService::new(config);
/// ```
#[derive(Debug, Clone)]
pub struct PositionsService {
    pub config: Config,
}

impl PositionsService {
    /// 创建职位服务实例
    ///
    /// # 参数
    /// - `config`: SDK配置信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::positions::PositionsService;
    ///
    /// let config = Config::new("app_id", "app_secret");
    /// let service = PositionsService::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取职位详情
    ///
    /// 根据职位ID获取职位的详细信息，包括基本信息、职责要求、编制情况等。
    /// 支持使用不同的用户ID类型进行查询，返回完整的职位配置信息。
    ///
    /// # API文档
    ///
    /// 根据职位ID获取职位的详细信息。
    /// 返回职位的基本信息、职责描述、要求条件、编制情况等完整资料。
    ///
    /// # 参数
    ///
    /// * `position_id` - 职位ID
    /// * `request` - 获取职位的请求参数，包含ID类型配置
    ///
    /// # 返回值
    ///
    /// 返回职位的详细信息，包含职责、要求、编制等完整数据
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::positions::*;
    ///
    /// let request = GetPositionRequest {
    ///     user_id_type: Some("open_id".to_string()),
    /// };
    ///
    /// let response = client.feishu_people.core.v1.positions
    ///     .get("position_001", &request).await?;
    /// println!("职位名称: {}", response.data.name);
    /// ```
    pub async fn get(
        &self,
        position_id: &str,
        request: &GetPositionRequest,
    ) -> SDKResult<BaseResponse<GetPositionResponse>> {
        // 构建查询参数
        let mut query_params = std::collections::HashMap::new();
        if let Some(user_id_type) = &request.user_id_type {
            query_params.insert("user_id_type", user_id_type.clone());
        }

        // 构建API请求
        let api_req = ApiRequest {
            http_http_http_method: reqwest::Method::GET,
            api_path: format!("/open-apis/feishu_people/core/v1/positions/{}", position_id),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            query_params,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 批量获取职位信息
    ///
    /// 根据职位ID列表批量获取多个职位的详细信息。
    /// 适用于需要同时查询多个职位信息的场景，提高查询效率。
    ///
    /// # API文档
    ///
    /// 根据职位ID列表批量获取职位信息。
    /// 最多支持50个职位ID的批量查询，返回职位的完整信息。
    ///
    /// # 参数
    ///
    /// * `request` - 批量获取职位的请求参数，包含职位ID列表和配置
    ///
    /// # 返回值
    ///
    /// 返回批量职位的详细信息列表和分页信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::positions::*;
    ///
    /// let request = BatchGetPositionsRequest {
    ///     position_ids: vec!["pos_001".to_string(), "pos_002".to_string()],
    ///     user_id_type: Some("open_id".to_string()),
    /// };
    ///
    /// let response = client.feishu_people.core.v1.positions
    ///     .batch_get(&request).await?;
    /// println!("获取到 {} 个职位", response.data.items.len());
    /// ```
    pub async fn batch_get(
        &self,
        request: &BatchGetPositionsRequest,
    ) -> SDKResult<BaseResponse<BatchGetPositionsResponse>> {
        // 构建API请求
        let api_req = ApiRequest {
            http_http_http_method: reqwest::Method::POST,
            api_path: "/open-apis/feishu_people/core/v1/positions/batch_get".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(request)?,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 根据部门获取职位列表
    ///
    /// 获取指定部门下的职位列表，支持分页查询。
    /// 可以用于了解部门的职位配置和编制情况。
    ///
    /// # API文档
    ///
    /// 根据部门ID获取该部门下的职位列表。
    /// 支持分页查询，返回部门内所有职位的基本信息。
    ///
    /// # 参数
    ///
    /// * `request` - 获取部门职位列表的请求参数
    ///
    /// # 返回值
    ///
    /// 返回部门职位列表和分页信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::positions::*;
    ///
    /// let request = GetPositionsByDepartmentRequest {
    ///     department_id: "dept_001".to_string(),
    ///     page_size: Some(20),
    ///     page_token: None,
    ///     user_id_type: Some("open_id".to_string()),
    /// };
    ///
    /// let response = client.feishu_people.core.v1.positions
    ///     .get_by_department(&request).await?;
    /// println!("部门职位数量: {}", response.data.items.len());
    /// ```
    pub async fn get_by_department(
        &self,
        request: &GetPositionsByDepartmentRequest,
    ) -> SDKResult<BaseResponse<GetPositionsByDepartmentResponse>> {
        // 构建查询参数
        let mut query_params = std::collections::HashMap::new();
        query_params.insert("department_id", request.department_id.clone());
        if let Some(page_size) = request.page_size {
            query_params.insert("page_size", page_size.to_string());
        }
        if let Some(page_token) = &request.page_token {
            query_params.insert("page_token", page_token.clone());
        }
        if let Some(user_id_type) = &request.user_id_type {
            query_params.insert("user_id_type", user_id_type.clone());
        }

        // 构建API请求
        let api_req = ApiRequest {
            http_http_http_method: reqwest::Method::GET,
            api_path: "/open-apis/feishu_people/core/v1/positions/list_by_department".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            query_params,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 搜索职位
    ///
    /// 根据关键词搜索职位，支持按职位名称、描述、职责等字段进行搜索。
    /// 返回匹配的职位列表，支持分页查询和相关性排序。
    ///
    /// # API文档
    ///
    /// 根据关键词搜索职位信息。
    /// 支持按职位名称、英文职位、职责描述等多种字段进行模糊搜索。
    ///
    /// # 参数
    ///
    /// * `request` - 搜索职位的请求参数，包含搜索关键词和过滤条件
    ///
    /// # 返回值
    ///
    /// 返回搜索结果和分页信息，包含匹配度评分
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::positions::*;
    ///
    /// let request = SearchPositionsRequest {
    ///     query: "工程师".to_string(),
    ///     page_size: Some(20),
    ///     page_token: None,
    ///     user_id_type: Some("open_id".to_string()),
    /// };
    ///
    /// let response = client.feishu_people.core.v1.positions
    ///     .search(&request).await?;
    /// println!("搜索到 {} 个职位", response.data.items.len());
    /// ```
    pub async fn search(
        &self,
        request: &SearchPositionsRequest,
    ) -> SDKResult<BaseResponse<SearchPositionsResponse>> {
        // 构建API请求
        let api_req = ApiRequest {
            http_http_http_method: reqwest::Method::POST,
            api_path: "/open-apis/feishu_people/core/v1/positions/search".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(request)?,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 创建职位
    ///
    /// 创建新的职位，设置职位的基本信息、职责要求、编制等。
    /// 创建成功后，职位将可用于人员配置和组织管理。
    ///
    /// # API文档
    ///
    /// 创建新的职位信息，系统会自动分配职位ID。
    /// 支持设置职位的名称、职责、要求、编制等完整信息。
    ///
    /// # 参数
    ///
    /// * `request` - 创建职位的请求参数，包含职位信息和配置
    ///
    /// # 返回值
    ///
    /// 返回创建成功的职位信息，包含系统分配的职位ID
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::positions::*;
    ///
    /// let position_data = PositionCreateData {
    ///     name: "高级工程师".to_string(),
    ///     en_name: Some("Senior Engineer".to_string()),
    ///     department_id: Some("dept_001".to_string()),
    ///     headcount: Some(5),
    ///     responsibilities: Some(vec![
    ///         "技术架构设计".to_string(),
    ///         "代码审查".to_string(),
    ///     ]),
    ///     ..Default::default()
    /// };
    ///
    /// let request = CreatePositionRequest {
    ///     position_data,
    ///     user_id_type: Some("open_id".to_string()),
    /// };
    ///
    /// let response = client.feishu_people.core.v1.positions
    ///     .create(&request).await?;
    /// println!("职位创建成功，ID: {}", response.data.position_id);
    /// ```
    pub async fn create(
        &self,
        request: &CreatePositionRequest,
    ) -> SDKResult<BaseResponse<CreatePositionResponse>> {
        // 构建API请求
        let api_req = ApiRequest {
            http_http_http_method: reqwest::Method::POST,
            api_path: "/open-apis/feishu_people/core/v1/positions".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(request)?,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 更新职位信息
    ///
    /// 更新职位的信息，支持修改职位的基本属性、职责要求等。
    /// 只更新传入的字段，未传入的字段保持不变。
    ///
    /// # API文档
    ///
    /// 修改职位的部分信息，只更新提供的字段。
    /// 支持修改职位的名称、职责、要求、编制等信息。
    ///
    /// # 参数
    ///
    /// * `position_id` - 职位ID
    /// * `request` - 修改职位的请求参数
    ///
    /// # 返回值
    ///
    /// 返回修改后的职位信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::positions::*;
    ///
    /// let update_data = PositionUpdateData {
    ///     name: Some("高级工程师(更新)".to_string()),
    ///     headcount: Some(6),
    ///     ..Default::default()
    /// };
    ///
    /// let request = UpdatePositionRequest {
    ///     position_data: update_data,
    ///     user_id_type: Some("open_id".to_string()),
    /// };
    ///
    /// let response = client.feishu_people.core.v1.positions
    ///     .update("position_001", &request).await?;
    /// println!("职位信息更新成功");
    /// ```
    pub async fn update(
        &self,
        position_id: &str,
        request: &UpdatePositionRequest,
    ) -> SDKResult<BaseResponse<UpdatePositionResponse>> {
        // 构建API请求
        let api_req = ApiRequest {
            http_http_http_method: reqwest::Method::PUT,
            api_path: format!("/open-apis/feishu_people/core/v1/positions/{}", position_id),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(request)?,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 删除职位
    ///
    /// 删除指定的职位，删除后职位将不再可用。
    /// 删除操作不可逆，请谨慎使用。建议先停用职位再删除。
    ///
    /// # API文档
    ///
    /// 删除职位信息，操作不可逆。
    /// 删除前请确保职位下没有人员配置，相关数据将被清理。
    ///
    /// # 参数
    ///
    /// * `position_id` - 要删除的职位ID
    /// * `request` - 删除职位的请求参数
    ///
    /// # 返回值
    ///
    /// 返回删除操作的结果
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::positions::*;
    ///
    /// let request = DeletePositionRequest {
    ///     user_id_type: Some("open_id".to_string()),
    /// };
    ///
    /// let response = client.feishu_people.core.v1.positions
    ///     .delete("position_001", &request).await?;
    /// println!("职位删除成功");
    /// ```
    pub async fn delete(
        &self,
        position_id: &str,
        request: &DeletePositionRequest,
    ) -> SDKResult<BaseResponse<DeletePositionResponse>> {
        // 构建查询参数
        let mut query_params = std::collections::HashMap::new();
        if let Some(user_id_type) = &request.user_id_type {
            query_params.insert("user_id_type", user_id_type.clone());
        }

        // 构建API请求
        let api_req = ApiRequest {
            http_http_http_method: reqwest::Method::DELETE,
            api_path: format!("/open-apis/feishu_people/core/v1/positions/{}", position_id),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            query_params,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 获取职位序列列表
    ///
    /// 获取职位序列的列表，包括序列的基本信息和包含的级别。
    /// 职位序列是组织中对职位进行分类管理的重要体系。
    ///
    /// # API文档
    ///
    /// 获取职位序列列表，支持分页查询。
    /// 返回职位序列的基本信息和包含的职位级别。
    ///
    /// # 参数
    ///
    /// * `request` - 获取职位序列列表的请求参数
    ///
    /// # 返回值
    ///
    /// 返回职位序列列表和分页信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::positions::*;
    ///
    /// let request = GetPositionSequencesRequest {
    ///     page_size: Some(20),
    ///     page_token: None,
    /// };
    ///
    /// let response = client.feishu_people.core.v1.positions
    ///     .get_position_sequences(&request).await?;
    /// println!("获取到 {} 个职位序列", response.data.items.len());
    /// ```
    pub async fn get_position_sequences(
        &self,
        request: &GetPositionSequencesRequest,
    ) -> SDKResult<BaseResponse<GetPositionSequencesResponse>> {
        // 构建查询参数
        let mut query_params = std::collections::HashMap::new();
        if let Some(page_size) = request.page_size {
            query_params.insert("page_size", page_size.to_string());
        }
        if let Some(page_token) = &request.page_token {
            query_params.insert("page_token", page_token.clone());
        }

        // 构建API请求
        let api_req = ApiRequest {
            http_http_http_method: reqwest::Method::GET,
            api_path: "/open-apis/feishu_people/core/v1/position_sequences".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            query_params,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 获取职位统计信息
    ///
    /// 获取职位的统计分析数据，包括职位总数、空缺情况、分布统计等。
    /// 支持按部门过滤，用于组织架构分析和人力资源规划。
    ///
    /// # API文档
    ///
    /// 获取职位统计信息，支持按部门过滤。
    /// 返回职位配置、空缺率、分布情况等统计数据。
    ///
    /// # 参数
    ///
    /// * `request` - 获取职位统计信息的请求参数
    ///
    /// # 返回值
    ///
    /// 返回职位的详细统计信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::positions::*;
    ///
    /// let request = GetPositionStatisticsRequest {
    ///     department_id: Some("dept_001".to_string()),
    /// };
    ///
    /// let response = client.feishu_people.core.v1.positions
    ///     .get_statistics(&request).await?;
    /// println!("总职位数: {}", response.data.total_positions);
    /// println!("空缺率: {:.2}%", response.data.vacancy_rate * 100.0);
    /// ```
    pub async fn get_statistics(
        &self,
        request: &GetPositionStatisticsRequest,
    ) -> SDKResult<BaseResponse<GetPositionStatisticsResponse>> {
        // 构建查询参数
        let mut query_params = std::collections::HashMap::new();
        if let Some(department_id) = &request.department_id {
            query_params.insert("department_id", department_id.clone());
        }

        // 构建API请求
        let api_req = ApiRequest {
            http_http_http_method: reqwest::Method::GET,
            api_path: "/open-apis/feishu_people/core/v1/positions/statistics".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            query_params,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 获取职位人员列表
    ///
    /// 获取指定职位下的人员列表，包括历史和当前的人员配置情况。
    /// 支持分页查询，用于了解职位的人员配置历史。
    ///
    /// # API文档
    ///
    /// 获取职位下的人员配置列表，支持分页查询。
    /// 返回职位的人员信息、配置时间、状态等详细数据。
    ///
    /// # 参数
    ///
    /// * `position_id` - 职位ID
    /// * `request` - 获取职位人员列表的请求参数
    ///
    /// # 返回值
    ///
    /// 返回职位人员列表和分页信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::positions::*;
    ///
    /// let request = GetPositionHoldersRequest {
    ///     page_size: Some(20),
    ///     page_token: None,
    ///     user_id_type: Some("open_id".to_string()),
    /// };
    ///
    /// let response = client.feishu_people.core.v1.positions
    ///     .get_position_holders("position_001", &request).await?;
    /// println!("职位人员数量: {}", response.data.items.len());
    /// ```
    pub async fn get_position_holders(
        &self,
        position_id: &str,
        request: &GetPositionHoldersRequest,
    ) -> SDKResult<BaseResponse<GetPositionHoldersResponse>> {
        // 构建查询参数
        let mut query_params = std::collections::HashMap::new();
        if let Some(page_size) = request.page_size {
            query_params.insert("page_size", page_size.to_string());
        }
        if let Some(page_token) = &request.page_token {
            query_params.insert("page_token", page_token.clone());
        }
        if let Some(user_id_type) = &request.user_id_type {
            query_params.insert("user_id_type", user_id_type.clone());
        }

        // 构建API请求
        let api_req = ApiRequest {
            http_http_http_method: reqwest::Method::GET,
            api_path: format!(
                "/open-apis/feishu_people/core/v1/positions/{}/holders",
                position_id
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            query_params,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    // ==================== Builder模式实现 ====================

    /// 获取职位详情构建器
    ///
    /// 提供流式API来构建获取职位详情的请求参数。
    /// 支持链式调用，方便配置查询参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::positions::*;
    ///
    /// let builder = client.feishu_people.core.v1.positions
    ///     .get_position_builder("position_001")
    ///     .user_id_type("open_id");
    ///
    /// let response = builder.execute(&client.feishu_people.core.v1.positions).await?;
    /// ```
    pub fn get_position_builder(&self, position_id: &str) -> GetPositionBuilder {
        GetPositionBuilder::new(position_id)
    }

    /// 批量获取职位构建器
    ///
    /// 提供流式API来构建批量获取职位的请求参数。
    /// 支持链式调用，方便配置批量查询参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::positions::*;
    ///
    /// let builder = client.feishu_people.core.v1.positions
    ///     .batch_get_positions_builder()
    ///     .position_ids(vec!["pos_001".to_string(), "pos_002".to_string()])
    ///     .user_id_type("open_id");
    ///
    /// let response = builder.execute(&client.feishu_people.core.v1.positions).await?;
    /// ```
    pub fn batch_get_positions_builder(&self) -> BatchGetPositionsBuilder {
        BatchGetPositionsBuilder::new()
    }

    /// 根据部门获取职位列表构建器
    ///
    /// 提供流式API来构建获取部门职位列表的请求参数。
    /// 支持链式调用，方便配置分页和过滤参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::positions::*;
    ///
    /// let builder = client.feishu_people.core.v1.positions
    ///     .get_positions_by_department_builder("dept_001")
    ///     .page_size(20)
    ///     .user_id_type("open_id");
    ///
    /// let response = builder.execute(&client.feishu_people.core.v1.positions).await?;
    /// ```
    pub fn get_positions_by_department_builder(
        &self,
        department_id: &str,
    ) -> GetPositionsByDepartmentBuilder {
        GetPositionsByDepartmentBuilder::new(department_id)
    }

    /// 搜索职位构建器
    ///
    /// 提供流式API来构建搜索职位的请求参数。
    /// 支持链式调用，方便配置搜索条件和分页参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::positions::*;
    ///
    /// let builder = client.feishu_people.core.v1.positions
    ///     .search_positions_builder()
    ///     .query("工程师".to_string())
    ///     .page_size(20)
    ///     .user_id_type("open_id");
    ///
    /// let response = builder.execute(&client.feishu_people.core.v1.positions).await?;
    /// ```
    pub fn search_positions_builder(&self) -> SearchPositionsBuilder {
        SearchPositionsBuilder::new()
    }

    /// 创建职位构建器
    ///
    /// 提供流式API来构建创建职位的请求参数。
    /// 支持链式调用，方便构建复杂的职位创建请求。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::positions::*;
    ///
    /// let position_data = PositionCreateData {
    ///     name: "高级工程师".to_string(),
    ///     department_id: Some("dept_001".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// let builder = client.feishu_people.core.v1.positions
    ///     .create_position_builder()
    ///     .position_data(position_data)
    ///     .user_id_type("open_id");
    ///
    /// let response = builder.execute(&client.feishu_people.core.v1.positions).await?;
    /// ```
    pub fn create_position_builder(&self) -> CreatePositionBuilder {
        CreatePositionBuilder::new()
    }

    /// 更新职位构建器
    ///
    /// 提供流式API来构建更新职位的请求参数。
    /// 支持链式调用，方便构建职位更新请求。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::positions::*;
    ///
    /// let update_data = PositionUpdateData {
    ///     name: Some("高级工程师(更新)".to_string()),
    ///     headcount: Some(6),
    ///     ..Default::default()
    /// };
    ///
    /// let builder = client.feishu_people.core.v1.positions
    ///     .update_position_builder("position_001")
    ///     .position_data(update_data)
    ///     .user_id_type("open_id");
    ///
    /// let response = builder.execute(&client.feishu_people.core.v1.positions).await?;
    /// ```
    pub fn update_position_builder(&self, position_id: &str) -> UpdatePositionBuilder {
        UpdatePositionBuilder::new(position_id)
    }

    /// 删除职位构建器
    ///
    /// 提供流式API来构建删除职位的请求参数。
    /// 支持链式调用，方便配置删除参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::positions::*;
    ///
    /// let builder = client.feishu_people.core.v1.positions
    ///     .delete_position_builder("position_001")
    ///     .user_id_type("open_id");
    ///
    /// let response = builder.execute(&client.feishu_people.core.v1.positions).await?;
    /// ```
    pub fn delete_position_builder(&self, position_id: &str) -> DeletePositionBuilder {
        DeletePositionBuilder::new(position_id)
    }

    /// 获取职位序列构建器
    ///
    /// 提供流式API来构建获取职位序列的请求参数。
    /// 支持链式调用，方便配置分页参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::positions::*;
    ///
    /// let builder = client.feishu_people.core.v1.positions
    ///     .get_position_sequences_builder()
    ///     .page_size(20);
    ///
    /// let response = builder.execute(&client.feishu_people.core.v1.positions).await?;
    /// ```
    pub fn get_position_sequences_builder(&self) -> GetPositionSequencesBuilder {
        GetPositionSequencesBuilder::new()
    }

    /// 获取职位统计信息构建器
    ///
    /// 提供流式API来构建获取职位统计信息的请求参数。
    /// 支持链式调用，方便配置过滤参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::positions::*;
    ///
    /// let builder = client.feishu_people.core.v1.positions
    ///     .get_position_statistics_builder()
    ///     .department_id("dept_001");
    ///
    /// let response = builder.execute(&client.feishu_people.core.v1.positions).await?;
    /// ```
    pub fn get_position_statistics_builder(&self) -> GetPositionStatisticsBuilder {
        GetPositionStatisticsBuilder::new()
    }

    /// 获取职位人员列表构建器
    ///
    /// 提供流式API来构建获取职位人员列表的请求参数。
    /// 支持链式调用，方便配置分页和查询参数。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::feishu_people::core::v1::positions::*;
    ///
    /// let builder = client.feishu_people.core.v1.positions
    ///     .get_position_holders_builder("position_001")
    ///     .page_size(20)
    ///     .user_id_type("open_id");
    ///
    /// let response = builder.execute(&client.feishu_people.core.v1.positions).await?;
    /// ```
    pub fn get_position_holders_builder(&self, position_id: &str) -> GetPositionHoldersBuilder {
        GetPositionHoldersBuilder::new(position_id)
    }
}

// ==================== Builder结构体实现 ====================

/// 获取职位详情构建器
#[derive(Debug, Clone)]
pub struct GetPositionBuilder {
    position_id: String,
    request: GetPositionRequest,
}

impl GetPositionBuilder {
    /// 创建新的Builder实例
    pub fn new(position_id: &str) -> Self {
        Self {
            position_id: position_id.to_string(),
            request: GetPositionRequest::default(),
        }
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: &str) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> (String, GetPositionRequest) {
        (self.position_id, self.request)
    }
}

impl Default for GetPositionBuilder {
    fn default() -> Self {
        Self::new("")
    }
}

// 应用ExecutableBuilder trait - 使用自定义实现
#[async_trait::async_trait]
impl
    open_lark_core::core::trait_system::ExecutableBuilder<
        PositionsService,
        (String, GetPositionRequest),
        BaseResponse<GetPositionResponse>,
    > for GetPositionBuilder
{
    async fn execute(
        &self,
        service: &PositionsService,
        _option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetPositionResponse>> {
        service.get_with_tuple(self.clone().build()).await
    }
}

/// 批量获取职位构建器
#[derive(Debug, Clone)]
pub struct BatchGetPositionsBuilder {
    request: BatchGetPositionsRequest,
}

impl BatchGetPositionsBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: BatchGetPositionsRequest::default(),
        }
    }

    /// 设置职位ID列表
    pub fn position_ids(mut self, position_ids: Vec<String>) -> Self {
        self.request.position_ids = position_ids;
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: &str) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> BatchGetPositionsRequest {
        self.request
    }
}

impl Default for BatchGetPositionsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// 应用ExecutableBuilder trait - 标准实现
// crate::impl_executable_builder!(
//    BatchGetPositionsBuilder,
//    PositionsService,
//    BatchGetPositionsRequest,
//    BaseResponse<BatchGetPositionsResponse>,
//    batch_get
//);

/// 根据部门获取职位列表构建器
#[derive(Debug, Clone)]
pub struct GetPositionsByDepartmentBuilder {
    request: GetPositionsByDepartmentRequest,
}

impl GetPositionsByDepartmentBuilder {
    /// 创建新的Builder实例
    pub fn new(department_id: &str) -> Self {
        Self {
            request: GetPositionsByDepartmentRequest {
                department_id: department_id.to_string(),
                page_size: None,
                page_token: None,
                user_id_type: None,
            },
        }
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: &str) -> Self {
        self.request.page_token = Some(page_token.to_string());
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: &str) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> GetPositionsByDepartmentRequest {
        self.request
    }
}

impl Default for GetPositionsByDepartmentBuilder {
    fn default() -> Self {
        Self::new("")
    }
}

// 应用ExecutableBuilder trait
// crate::impl_executable_builder!(
//    GetPositionsByDepartmentBuilder,
//    PositionsService,
//    GetPositionsByDepartmentRequest,
//    BaseResponse<GetPositionsByDepartmentResponse>,
//    get_by_department
//);

/// 搜索职位构建器
#[derive(Debug, Clone)]
pub struct SearchPositionsBuilder {
    request: SearchPositionsRequest,
}

impl SearchPositionsBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: SearchPositionsRequest::default(),
        }
    }

    /// 设置搜索关键词
    pub fn query(mut self, query: String) -> Self {
        self.request.query = query;
        self
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: &str) -> Self {
        self.request.page_token = Some(page_token.to_string());
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: &str) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> SearchPositionsRequest {
        self.request
    }
}

impl Default for SearchPositionsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// 应用ExecutableBuilder trait
// crate::impl_executable_builder!(
//    SearchPositionsBuilder,
//    PositionsService,
//    SearchPositionsRequest,
//    BaseResponse<SearchPositionsResponse>,
//    search
//);

/// 创建职位构建器
#[derive(Debug, Clone)]
pub struct CreatePositionBuilder {
    request: CreatePositionRequest,
}

impl CreatePositionBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: CreatePositionRequest::default(),
        }
    }

    /// 设置职位数据
    pub fn position_data(mut self, position_data: PositionCreateData) -> Self {
        self.request.position_data = position_data;
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: &str) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> CreatePositionRequest {
        self.request
    }
}

impl Default for CreatePositionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// 应用ExecutableBuilder trait
// crate::impl_executable_builder!(
//    CreatePositionBuilder,
//    PositionsService,
//    CreatePositionRequest,
//    BaseResponse<CreatePositionResponse>,
//    create
//);

/// 更新职位构建器
#[derive(Debug, Clone)]
pub struct UpdatePositionBuilder {
    position_id: String,
    request: UpdatePositionRequest,
}

impl UpdatePositionBuilder {
    /// 创建新的Builder实例
    pub fn new(position_id: &str) -> Self {
        Self {
            position_id: position_id.to_string(),
            request: UpdatePositionRequest::default(),
        }
    }

    /// 设置职位数据
    pub fn position_data(mut self, position_data: PositionUpdateData) -> Self {
        self.request.position_data = position_data;
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: &str) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> (String, UpdatePositionRequest) {
        (self.position_id, self.request)
    }
}

impl Default for UpdatePositionBuilder {
    fn default() -> Self {
        Self::new("")
    }
}

// 应用ExecutableBuilder trait - 使用自定义实现
#[async_trait::async_trait]
impl
    open_lark_core::core::trait_system::ExecutableBuilder<
        PositionsService,
        (String, UpdatePositionRequest),
        BaseResponse<UpdatePositionResponse>,
    > for UpdatePositionBuilder
{
    async fn execute(
        &self,
        service: &PositionsService,
        _option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UpdatePositionResponse>> {
        service.update_with_tuple(self.clone().build()).await
    }
}

/// 删除职位构建器
#[derive(Debug, Clone)]
pub struct DeletePositionBuilder {
    position_id: String,
    request: DeletePositionRequest,
}

impl DeletePositionBuilder {
    /// 创建新的Builder实例
    pub fn new(position_id: &str) -> Self {
        Self {
            position_id: position_id.to_string(),
            request: DeletePositionRequest::default(),
        }
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: &str) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> (String, DeletePositionRequest) {
        (self.position_id, self.request)
    }
}

impl Default for DeletePositionBuilder {
    fn default() -> Self {
        Self::new("")
    }
}

// 应用ExecutableBuilder trait - 使用自定义实现
#[async_trait::async_trait]
impl
    open_lark_core::core::trait_system::ExecutableBuilder<
        PositionsService,
        (String, DeletePositionRequest),
        BaseResponse<DeletePositionResponse>,
    > for DeletePositionBuilder
{
    async fn execute(
        &self,
        service: &PositionsService,
        _option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<DeletePositionResponse>> {
        service.delete_with_tuple(self.clone().build()).await
    }
}

/// 获取职位序列构建器
#[derive(Debug, Clone)]
pub struct GetPositionSequencesBuilder {
    request: GetPositionSequencesRequest,
}

impl GetPositionSequencesBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: GetPositionSequencesRequest::default(),
        }
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: &str) -> Self {
        self.request.page_token = Some(page_token.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> GetPositionSequencesRequest {
        self.request
    }
}

impl Default for GetPositionSequencesBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// 应用ExecutableBuilder trait
// crate::impl_executable_builder!(
//    GetPositionSequencesBuilder,
//    PositionsService,
//    GetPositionSequencesRequest,
//    BaseResponse<GetPositionSequencesResponse>,
//    get_position_sequences
//);

/// 获取职位统计信息构建器
#[derive(Debug, Clone)]
pub struct GetPositionStatisticsBuilder {
    request: GetPositionStatisticsRequest,
}

impl GetPositionStatisticsBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: GetPositionStatisticsRequest::default(),
        }
    }

    /// 设置部门ID
    pub fn department_id(mut self, department_id: &str) -> Self {
        self.request.department_id = Some(department_id.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> GetPositionStatisticsRequest {
        self.request
    }
}

impl Default for GetPositionStatisticsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// 应用ExecutableBuilder trait
// crate::impl_executable_builder!(
//    GetPositionStatisticsBuilder,
//    PositionsService,
//    GetPositionStatisticsRequest,
//    BaseResponse<GetPositionStatisticsResponse>,
//    get_statistics
//);

/// 获取职位人员列表构建器
#[derive(Debug, Clone)]
pub struct GetPositionHoldersBuilder {
    position_id: String,
    request: GetPositionHoldersRequest,
}

impl GetPositionHoldersBuilder {
    /// 创建新的Builder实例
    pub fn new(position_id: &str) -> Self {
        Self {
            position_id: position_id.to_string(),
            request: GetPositionHoldersRequest::default(),
        }
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: &str) -> Self {
        self.request.page_token = Some(page_token.to_string());
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: &str) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> (String, GetPositionHoldersRequest) {
        (self.position_id, self.request)
    }
}

impl Default for GetPositionHoldersBuilder {
    fn default() -> Self {
        Self::new("")
    }
}

// 应用ExecutableBuilder trait - 使用自定义实现
#[async_trait::async_trait]
impl
    open_lark_core::core::trait_system::ExecutableBuilder<
        PositionsService,
        (String, GetPositionHoldersRequest),
        BaseResponse<GetPositionHoldersResponse>,
    > for GetPositionHoldersBuilder
{
    async fn execute(
        &self,
        service: &PositionsService,
        _option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetPositionHoldersResponse>> {
        service
            .get_position_holders_with_tuple(self.clone().build())
            .await
    }
}

// 为PositionsService实现辅助方法，处理Builder的参数
impl PositionsService {
    async fn get_with_tuple(
        &self,
        params: (String, GetPositionRequest),
    ) -> SDKResult<BaseResponse<GetPositionResponse>> {
        self.get(&params.0, &params.1).await
    }

    async fn update_with_tuple(
        &self,
        params: (String, UpdatePositionRequest),
    ) -> SDKResult<BaseResponse<UpdatePositionResponse>> {
        self.update(&params.0, &params.1).await
    }

    async fn delete_with_tuple(
        &self,
        params: (String, DeletePositionRequest),
    ) -> SDKResult<BaseResponse<DeletePositionResponse>> {
        self.delete(&params.0, &params.1).await
    }

    async fn get_position_holders_with_tuple(
        &self,
        params: (String, GetPositionHoldersRequest),
    ) -> SDKResult<BaseResponse<GetPositionHoldersResponse>> {
        self.get_position_holders(&params.0, &params.1).await
    }
}

// ==================== 数据模型 ====================

/// 获取职位请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPositionRequest {
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl Default for GetPositionRequest {
    fn default() -> Self {
        Self { user_id_type: None }
    }
}

/// 获取职位响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPositionResponse {
    /// 职位信息
    pub position: Position,
}

impl ApiResponseTrait for GetPositionResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

/// 批量获取职位请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGetPositionsRequest {
    /// 职位ID列表
    pub position_ids: Vec<String>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl Default for BatchGetPositionsRequest {
    fn default() -> Self {
        Self {
            position_ids: vec![],
            user_id_type: None,
        }
    }
}

/// 批量获取职位响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGetPositionsResponse {
    /// 职位列表
    pub items: Vec<Position>,
    /// 是否还有更多数据
    pub has_more: Option<bool>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 总数
    pub total: Option<i32>,
}

impl ApiResponseTrait for BatchGetPositionsResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

/// 根据部门获取职位列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPositionsByDepartmentRequest {
    /// 部门ID
    pub department_id: String,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

/// 根据部门获取职位列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPositionsByDepartmentResponse {
    /// 职位列表
    pub items: Vec<Position>,
    /// 是否还有更多数据
    pub has_more: Option<bool>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 总数
    pub total: Option<i32>,
}

impl ApiResponseTrait for GetPositionsByDepartmentResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

/// 搜索职位请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchPositionsRequest {
    /// 搜索关键词
    pub query: String,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl Default for SearchPositionsRequest {
    fn default() -> Self {
        Self {
            query: String::new(),
            page_size: None,
            page_token: None,
            user_id_type: None,
        }
    }
}

/// 搜索职位响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchPositionsResponse {
    /// 搜索结果项
    pub items: Vec<PositionSearchResult>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 是否还有更多数据
    pub has_more: Option<bool>,
    /// 总数
    pub total: Option<i32>,
}

/// 创建职位请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePositionRequest {
    /// 职位数据
    pub position_data: PositionCreateData,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl Default for CreatePositionRequest {
    fn default() -> Self {
        Self {
            position_data: PositionCreateData::default(),
            user_id_type: None,
        }
    }
}

/// 创建职位响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePositionResponse {
    /// 职位ID
    pub position_id: String,
    /// 职位信息
    pub position: Position,
}

/// 更新职位请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePositionRequest {
    /// 职位数据
    pub position_data: PositionUpdateData,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl Default for UpdatePositionRequest {
    fn default() -> Self {
        Self {
            position_data: PositionUpdateData::default(),
            user_id_type: None,
        }
    }
}

/// 更新职位响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePositionResponse {
    /// 职位信息
    pub position: Position,
}

/// 删除职位请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePositionRequest {
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl Default for DeletePositionRequest {
    fn default() -> Self {
        Self { user_id_type: None }
    }
}

/// 删除职位响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeletePositionResponse {}

/// 获取职位序列请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPositionSequencesRequest {
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl Default for GetPositionSequencesRequest {
    fn default() -> Self {
        Self {
            page_size: None,
            page_token: None,
        }
    }
}

/// 获取职位序列响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPositionSequencesResponse {
    /// 职位序列列表
    pub items: Vec<PositionSequence>,
    /// 是否还有更多数据
    pub has_more: Option<bool>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 总数
    pub total: Option<i32>,
}

/// 获取职位统计信息请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPositionStatisticsRequest {
    /// 部门ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
}

impl Default for GetPositionStatisticsRequest {
    fn default() -> Self {
        Self {
            department_id: None,
        }
    }
}

/// 获取职位统计信息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPositionStatisticsResponse {
    /// 统计信息
    pub statistics: PositionStatistics,
}

/// 获取职位人员列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPositionHoldersRequest {
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl Default for GetPositionHoldersRequest {
    fn default() -> Self {
        Self {
            page_size: None,
            page_token: None,
            user_id_type: None,
        }
    }
}

/// 获取职位人员列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPositionHoldersResponse {
    /// 人员列表
    pub items: Vec<Person>,
    /// 是否还有更多数据
    pub has_more: Option<bool>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 总数
    pub total: Option<i32>,
}

/// 职位搜索结果
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PositionSearchResult {
    /// 职位ID
    pub position_id: String,
    /// 职位名称
    pub name: String,
    /// 英文名称
    pub en_name: Option<String>,
    /// 部门名称
    pub department_name: Option<String>,
    /// 职位级别名称
    pub position_level_name: Option<String>,
    /// 职位序列名称
    pub position_sequence_name: Option<String>,
    /// 当前人数
    pub current_count: i32,
    /// 编制人数
    pub headcount: i32,
    /// 匹配分数
    pub match_score: f64,
}

/// 职位统计信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PositionStatistics {
    /// 总职位数
    pub total_positions: i32,
    /// 活跃职位数
    pub active_positions: i32,
    /// 非活跃职位数
    pub inactive_positions: i32,
    /// 总编制数
    pub total_headcount: i32,
    /// 当前员工数
    pub current_employees: i32,
    /// 空缺数
    pub vacancy_count: i32,
    /// 空缺率
    pub vacancy_rate: f64,
    /// 部门分布
    pub department_distribution: Option<serde_json::Value>,
    /// 级别分布
    pub level_distribution: Option<serde_json::Value>,
}

/// 职位信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Position {
    /// 职位ID
    pub position_id: String,
    /// 职位名称
    pub name: String,
    /// 英文名称
    pub en_name: Option<String>,
    /// 职位序列ID
    pub position_sequence_id: Option<String>,
    /// 职位级别ID
    pub position_level_id: Option<String>,
    /// 部门ID
    pub department_id: Option<String>,
    /// 职位描述
    pub description: Option<String>,
    /// 职责
    pub responsibilities: Option<Vec<String>>,
    /// 要求
    pub requirements: Option<Vec<String>>,
    /// 职位状态
    pub status: Option<String>,
    /// 编制人数
    pub headcount: Option<i32>,
    /// 当前人数
    pub current_count: Option<i32>,
    /// 创建时间
    pub created_at: Option<String>,
    /// 更新时间
    pub updated_at: Option<String>,
    /// 扩展属性
    pub extended_attributes: Option<serde_json::Value>,
}

/// 职位创建数据
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PositionCreateData {
    /// 职位名称
    pub name: String,
    /// 英文名称
    pub en_name: Option<String>,
    /// 职位序列ID
    pub position_sequence_id: Option<String>,
    /// 职位级别ID
    pub position_level_id: Option<String>,
    /// 部门ID
    pub department_id: Option<String>,
    /// 职位描述
    pub description: Option<String>,
    /// 职责
    pub responsibilities: Option<Vec<String>>,
    /// 要求
    pub requirements: Option<Vec<String>>,
    /// 编制人数
    pub headcount: Option<i32>,
    /// 扩展属性
    pub extended_attributes: Option<serde_json::Value>,
}

/// 职位更新数据
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PositionUpdateData {
    /// 职位名称
    pub name: Option<String>,
    /// 英文名称
    pub en_name: Option<String>,
    /// 职位序列ID
    pub position_sequence_id: Option<String>,
    /// 职位级别ID
    pub position_level_id: Option<String>,
    /// 部门ID
    pub department_id: Option<String>,
    /// 职位描述
    pub description: Option<String>,
    /// 职责
    pub responsibilities: Option<Vec<String>>,
    /// 要求
    pub requirements: Option<Vec<String>>,
    /// 职位状态
    pub status: Option<String>,
    /// 编制人数
    pub headcount: Option<i32>,
    /// 扩展属性
    pub extended_attributes: Option<serde_json::Value>,
}

/// 职位序列
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PositionSequence {
    /// 序列ID
    pub sequence_id: String,
    /// 序列名称
    pub name: String,
    /// 英文名称
    pub en_name: Option<String>,
    /// 描述
    pub description: Option<String>,
    /// 职位级别列表
    pub levels: Option<Vec<PositionLevel>>,
    /// 创建时间
    pub created_at: Option<String>,
    /// 更新时间
    pub updated_at: Option<String>,
}

/// 职位级别
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PositionLevel {
    /// 级别ID
    pub level_id: String,
    /// 级别名称
    pub name: String,
    /// 英文名称
    pub en_name: Option<String>,
    /// 级别
    pub level: i32,
    /// 描述
    pub description: Option<String>,
    /// 创建时间
    pub created_at: Option<String>,
    /// 更新时间
    pub updated_at: Option<String>,
}

/// 人员信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Person {
    /// 用户ID
    pub user_id: String,
    /// 姓名
    pub name: String,
    /// 英文名
    pub en_name: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// 手机号
    pub mobile: Option<String>,
    /// 员工类型
    pub employee_type: Option<String>,
    /// 状态
    pub status: Option<String>,
    /// 职位
    pub position: Option<String>,
    /// 部门
    pub department: Option<String>,
    /// 入职时间
    pub hire_time: Option<String>,
    /// 扩展属性
    pub extended_attributes: Option<serde_json::Value>,
}

// ==================== ApiResponseTrait实现 ====================

impl ApiResponseTrait for SearchPositionsResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

impl ApiResponseTrait for CreatePositionResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

impl ApiResponseTrait for UpdatePositionResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

impl ApiResponseTrait for DeletePositionResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

impl ApiResponseTrait for GetPositionSequencesResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

impl ApiResponseTrait for GetPositionStatisticsResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

impl ApiResponseTrait for GetPositionHoldersResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}
