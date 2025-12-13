/// 获取知识空间设置
///
/// 此接口用于获取知识空间的详细设置信息。
/// 注意：实际上知识空间的基础设置信息通过 get space info 接口获取。
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space/get

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat, Response},
    config::Config,
    http::Transport,
    SDKResult,
};
