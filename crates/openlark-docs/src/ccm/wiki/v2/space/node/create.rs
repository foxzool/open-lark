//! 创建知识空间节点
//!
//! 此接口用于在知识节点里创建节点到指定位置。
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-node/create

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::WikiApiV2, api_utils::*};
use crate::wiki::v2::models::WikiSpaceNode;

/// 创建知识空间节点请求
pub struct CreateWikiSpaceNodeRequest {
    space_id: String,
    config: Config,
}

/// 创建知识空间节点请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWikiSpaceNodeParams {
    /// 实际文档类型（必填）
    ///
    /// 支持的文档类型：
    /// - `doc`: 文本文档
    /// - `docx`: Word 文档（推荐）
    /// - `sheet`: 电子表格
    /// - `slide`: 幻灯片
    /// - `mindnote`: 思维笔记
    /// - `bitable`: 多维表格
    /// - `file`: 普通文件
    ///
    /// # 示例
    /// ```rust,no_run
    /// use openlark_docs::ccm::wiki::v2::space::node::CreateWikiSpaceNodeParams;
    ///
    /// // 创建 Word 文档节点
    /// let params = CreateWikiSpaceNodeParams {
    ///     obj_type: "docx".to_string(),
    ///     parent_node_token: Some("parent_token".to_string()),
    ///     node_type: Some("origin".to_string()),
    ///     title: Some("项目文档".to_string()),
    ///     obj_token: None,
    /// };
    ///
    /// // 创建电子表格节点
    /// let sheet_params = CreateWikiSpaceNodeParams {
    ///     obj_type: "sheet".to_string(),
    ///     parent_node_token: Some("parent_token".to_string()),
    ///     title: Some("项目进度表".to_string()),
    ///     obj_token: None,
    ///     node_type: None,
    /// };
    /// ```
    pub obj_type: String,
    /// 实际文档 token（部分场景可不传）
    ///
    /// # 何时需要传此参数
    /// - 当需要在知识空间中创建已存在文档的引用节点时
    /// - 当从云空间移动文档到知识空间时
    ///
    /// # 何时不需要传此参数
    /// - 创建全新的空白文档节点时
    pub obj_token: Option<String>,
    /// 父节点Token
    ///
    /// # 使用场景
    /// - 指定新节点在知识空间树中的位置
    /// - 为空时，节点将创建在知识空间根目录
    ///
    /// # 如何获取父节点Token
    /// - 调用 `list_space_nodes` API 获取节点列表
    /// - 使用已有节点的 `node_token` 作为父节点
    pub parent_node_token: Option<String>,
    /// 节点类型
    ///
    /// # 常用值
    /// - `origin`: 原始节点（默认值）
    /// - `shortcut`: 快捷方式节点
    /// - 其他扩展类型（根据实际需求）
    pub node_type: Option<String>,
    /// 节点标题（可选）
    ///
    /// # 注意事项
    /// - 如果不传，将使用文档的原始标题
    /// - 对于新创建的文档，建议设置标题
    /// - 标题长度限制：通常不超过 200 个字符
    pub title: Option<String>,
}

/// 创建知识空间节点响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWikiSpaceNodeResponse {
    /// 节点信息
    pub node: Option<WikiSpaceNode>,
}

impl ApiResponseTrait for CreateWikiSpaceNodeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CreateWikiSpaceNodeRequest {
    /// 创建创建知识空间节点请求
    pub fn new(config: Config) -> Self {
        Self {
            space_id: String::new(),
            config,
        }
    }

    /// 设置知识空间ID
    pub fn space_id(mut self, space_id: impl Into<String>) -> Self {
        self.space_id = space_id.into();
        self
    }

    /// 执行请求
    pub async fn execute(
        self,
        params: CreateWikiSpaceNodeParams,
    ) -> SDKResult<CreateWikiSpaceNodeResponse> {
        // 验证必填字段
        validate_required!(self.space_id, "知识空间ID不能为空");

        let valid_types = [
            "doc", "docx", "sheet", "slide", "mindnote", "bitable", "file",
        ];
        if !valid_types.contains(&params.obj_type.as_str()) {
            return Err(openlark_core::error::CoreError::validation_msg(
                &format!(
                    "obj_type 参数无效: '{}'. 支持的类型为: doc, docx, sheet, slide, mindnote, bitable, file",
                    params.obj_type
                )
            ));
        }

        validate_required!(params.obj_type, "obj_type不能为空");

        // 使用新的enum+builder系统生成API端点
        let api_endpoint = WikiApiV2::SpaceNodeCreate(self.space_id.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<CreateWikiSpaceNodeResponse> =
            ApiRequest::post(&api_endpoint.to_url())
                .body(serialize_params(&params, "创建知识空间节点")?);

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "创建知识空间节点")
    }
}
