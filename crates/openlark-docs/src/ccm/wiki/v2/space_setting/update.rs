/// 更新知识空间设置
///
/// 此接口用于更新知识空间的设置，如评论权限、复制权限等。
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space_setting/update

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat, Response},
    config::Config,
    http::Transport,
    SDKResult,
};
