/// 文件操作模块
///
/// 提供云盘文件管理功能，包括上传、下载、复制、移动、删除、评论等。
pub mod comment;
pub mod copy;
pub mod create_folder;
pub mod create_shortcut;
pub mod delete;
pub mod delete_subscribe;
pub mod download;
pub mod get_subscribe;
pub mod list;
pub mod r#move;
pub mod statistics;
pub mod subscribe;
pub mod subscription;
pub mod task_check;
pub mod upload_all;
pub mod upload_finish;
pub mod upload_part;
pub mod upload_prepare;
pub mod version;
pub mod view_record;

// 显式导出 - 避免使用 glob reexport
pub use comment::{
    BatchQueryCommentRequest, BatchQueryCommentResponse,
    Comment, CommentContent, CommentElement, CreateCommentReply, CreateCommentReplyList,
    CreateCommentRequest,
    GetCommentRequest, GetCommentResponse,
    ListCommentsRequest, ListCommentsResponse,
    PatchCommentRequest, PatchCommentResponse,
    DeleteCommentReplyRequest, DeleteCommentReplyResponse,
    ListCommentReplyRequest, ListCommentReplyResponse,
};

pub use copy::{CopyFileRequest, CopyFileResponse};

pub use create_folder::{CreateFolderRequest, CreateFolderResponse};

pub use create_shortcut::{CreateFileShortcutRequest, CreateFileShortcutResponse, ShortcutNode, CreateShortcutInfo};

pub use delete::{DeleteFileRequest, DeleteFileResponse};

pub use delete_subscribe::{DeleteSubscribeRequest, DeleteSubscribeResponse};

pub use download::{DownloadFileRequest};

pub use get_subscribe::{GetSubscribeRequest, GetSubscribeResponse};

pub use list::{ListFilesRequest, ListFilesResponse, FileInfo, ShortcutInfo};

pub use r#move::{MoveFileRequest, MoveFileResponse};

pub use statistics::{GetFileStatisticsRequest, GetFileStatisticsResponse};

pub use subscribe::{SubscribeFileRequest, SubscribeFileResponse};

pub use task_check::{CheckTaskStatusRequest, CheckTaskStatusResponse};

pub use upload_all::{UploadAllRequest, UploadAllResponse};

pub use upload_finish::{UploadFinishRequest, UploadFinishResponse};

pub use upload_part::{UploadPartRequest, UploadPartResponse};

pub use upload_prepare::{UploadPrepareRequest, UploadPrepareResponse};

pub use view_record::GetFileViewRecordsRequest;
