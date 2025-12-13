/// 创建知识空间成员
///
/// 此接口用于向知识空间添加成员，支持用户类型成员和不同权限角色。
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space_member/create

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat, Response},
    config::Config,
    http::Transport,
    SDKResult,
};
