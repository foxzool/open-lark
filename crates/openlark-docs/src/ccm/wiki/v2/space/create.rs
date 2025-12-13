/// 创建知识空间
///
/// 此接口用于创建知识空间。
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space/create

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat, Response},
    config::Config,
    http::Transport,
    SDKResult,
};
