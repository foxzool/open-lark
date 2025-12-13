/// 获取知识空间列表
///
/// 此接口用于获取有权限访问的知识空间列表。
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space/list

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat, Response},
    config::Config,
    http::Transport,
    SDKResult,
};
