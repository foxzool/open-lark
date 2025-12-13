/// 删除知识空间成员
///
/// 此接口用于从知识空间中删除成员。
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space_member/delete

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat, Response},
    config::Config,
    http::Transport,
    SDKResult,
};
