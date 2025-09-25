use crate::core::{config::Config, req_option::RequestOption, SDKResult};

pub use batch_query::{
    batch_query_comments, BatchQueryCommentsRequest, BatchQueryCommentsResponse,
};
pub use create::{
    create_comment, ContentBuilder, CreateCommentRequest, CreateCommentResponse, CreatedComment,
};
pub use delete_reply::{delete_reply, DeleteReplyRequest, DeleteReplyResponse, DeletedReply};
pub use get::{get_comment, GetCommentRequest, GetCommentResponse};
pub use list::{
    list_comments, Comment, ContentElement, ListCommentsRequest, ListCommentsResponse, Reply,
    ReplyContent, TextRun,
};
pub use list_replies::{list_replies, ListRepliesRequest, ListRepliesResponse};
pub use patch::{patch_comment, PatchCommentRequest, PatchCommentResponse};
pub use update_reply::{update_reply, UpdateReplyRequest, UpdateReplyResponse, UpdatedReply};

mod batch_query;
mod create;
mod delete_reply;
mod get;
mod list;
mod list_replies;
mod patch;
mod update_reply;

/// 评论服务
pub struct CommentsService {
    config: Config,
}

impl CommentsService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 使用共享配置（实验性）
    pub fn new_from_shared(shared: std::sync::Arc<Config>) -> Self {
        Self {
            config: (*shared).clone(),
        }
    }

    /// 获取云文档所有评论
    pub async fn list(
        &self,
        request: ListCommentsRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<ListCommentsResponse>> {
        list_comments(request, &self.config, option).await
    }

    /// 批量获取评论
    pub async fn batch_query(
        &self,
        request: BatchQueryCommentsRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<BatchQueryCommentsResponse>> {
        batch_query_comments(request, &self.config, option).await
    }

    /// 解决/恢复评论
    pub async fn patch(
        &self,
        request: PatchCommentRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<PatchCommentResponse>> {
        patch_comment(request, &self.config, option).await
    }

    /// 添加全文评论
    pub async fn create(
        &self,
        request: CreateCommentRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<CreateCommentResponse>> {
        create_comment(request, &self.config, option).await
    }

    /// 获取全文评论
    pub async fn get(
        &self,
        request: GetCommentRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<GetCommentResponse>> {
        get_comment(request, &self.config, option).await
    }

    /// 获取回复信息
    pub async fn list_replies(
        &self,
        request: ListRepliesRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<ListRepliesResponse>> {
        list_replies(request, &self.config, option).await
    }

    /// 更新回复的内容
    pub async fn update_reply(
        &self,
        request: UpdateReplyRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<UpdateReplyResponse>> {
        update_reply(request, &self.config, option).await
    }

    /// 删除回复
    pub async fn delete_reply(
        &self,
        request: DeleteReplyRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<DeleteReplyResponse>> {
        delete_reply(request, &self.config, option).await
    }
}
