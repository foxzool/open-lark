#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use crate::core::{config::Config, req_option::RequestOption, SDKResult};

pub use batch_query::{
    batch_query_comments, BatchQueryCommentsRequest, BatchQueryCommentsResponse,
};
pub use create::{
    create_comment, ContentBuilder, CreateCommentRequest, CreateCommentResponse, CreatedComment,
};
pub use delete_reply::{
    delete_reply, DeleteReplyRequest, DeleteReplyResponse, DeletedReply,
};
pub use get::{get_comment, GetCommentRequest, GetCommentResponse};
pub use list::{
    list_comments, Comment, ContentElement, ListCommentsRequest, ListCommentsResponse, Reply,
    ReplyContent, TextRun,
};
pub use list_replies::{list_replies, ListRepliesRequest, ListRepliesResponse};
pub use patch::{patch_comment, PatchCommentRequest, PatchCommentResponse};
pub use update_reply::{
    update_reply, UpdateReplyRequest, UpdateReplyResponse, UpdatedReply,
};
pub mod batch_query;
pub mod create;
pub mod delete_reply;
pub mod get;
pub mod list;
pub mod list_replies;
pub mod patch;
pub mod update_reply;
/// 评论服务
///
/// 提供云文档评论相关功能的统一接口，包括：
/// - 创建全文评论
/// - 获取评论列表和单个评论
/// - 评论回复功能
/// - 评论更新和删除
/// - 批量查询操作
#[derive(Debug, Clone)]
pub struct CommentsService {
    config: Config,
}

impl CommentsService {
    /// 创建新的评论服务实例
    ///
    /// # 参数
    /// - `config`: SDK配置信息
    ///
    /// # 示例
    /// ```
    /// use open_lark_core::core::config::Config;
    /// use open_lark::service::cloud_docs::comments::CommentsService;
    ///
    /// let config = Config::new("app_id", "app_secret");
    /// let comments_service = CommentsService::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self { config }
    }
  /// 获取云文档所有评论
    ///
    /// # 参数
    /// - `request`: 获取评论列表请求
    /// - `option`: 可选请求参数
    ///
    /// # 返回
    /// 包含评论列表信息的响应
    pub async fn list(
        &self,
        request: ListCommentsRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<ListCommentsResponse>> {
        list_comments(request, &self.config, option).await
    }

    /// 批量获取评论
    ///
    /// # 参数
    /// - `request`: 批量查询评论请求
    /// - `option`: 可选请求参数
    ///
    /// # 返回
    /// 包含批量评论信息的响应
    pub async fn batch_query(
        &self,
        request: BatchQueryCommentsRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<BatchQueryCommentsResponse>> {
        batch_query_comments(request, &self.config, option).await
    }

    /// 解决/恢复评论
    ///
    /// # 参数
    /// - `request`: 修改评论状态请求
    /// - `option`: 可选请求参数
    ///
    /// # 返回
    /// 包含修改结果信息的响应
    pub async fn patch(
        &self,
        request: PatchCommentRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<PatchCommentResponse>> {
        patch_comment(request, &self.config, option).await
    }

    /// 添加全文评论
    ///
    /// # 参数
    /// - `request`: 创建评论请求
    /// - `option`: 可选请求参数
    ///
    /// # 返回
    /// 包含创建的评论信息响应
    pub async fn create(
        &self,
        request: CreateCommentRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<CreateCommentResponse>> {
        create_comment(request, &self.config, option).await
    }

    /// 获取全文评论
    ///
    /// # 参数
    /// - `request`: 获取单个评论请求
    /// - `option`: 可选请求参数
    ///
    /// # 返回
    /// 包含评论详细信息响应
    pub async fn get(
        &self,
        request: GetCommentRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<GetCommentResponse>> {
        get_comment(request, &self.config, option).await
    }

    /// 获取回复信息
    ///
    /// # 参数
    /// - `request`: 获取回复列表请求
    /// - `option`: 可选请求参数
    ///
    /// # 返回
    /// 包含回复列表信息响应
    pub async fn list_replies(
        &self,
        request: ListRepliesRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<ListRepliesResponse>> {
        list_replies(request, &self.config, option).await
    }

    /// 更新回复的内容
    ///
    /// # 参数
    /// - `request`: 更新回复请求
    /// - `option`: 可选请求参数
    ///
    /// # 返回
    /// 包含更新结果信息响应
    pub async fn update_reply(
        &self,
        request: UpdateReplyRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<UpdateReplyResponse>> {
        update_reply(request, &self.config, option).await
    }

    /// 删除回复
    ///
    /// # 参数
    /// - `request`: 删除回复请求
    /// - `option`: 可选请求参数
    ///
    /// # 返回
    /// 包含删除结果信息响应
    pub async fn delete_reply(
        &self,
        request: DeleteReplyRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<DeleteReplyResponse>> {
        delete_reply(request, &self.config, option).await
    }
}