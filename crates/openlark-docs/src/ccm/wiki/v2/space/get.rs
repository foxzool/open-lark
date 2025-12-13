/// 获取知识空间信息
///
/// 此接口用于获取知识空间的详细信息。
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space/get

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat, Response},
    config::Config,
    http::Transport,
    SDKResult,
};
