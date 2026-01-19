/// 获取知识空间信息

///

/// 获取指定知识空间的详细信息。

/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space/get

use openlark_core::{

    api::{ApiRequest, ApiResponseTrait, ResponseFormat},

    config::Config,

    http::Transport,

    req_option::RequestOption,

    SDKResult,

};



use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::WikiApiV2;



/// 获取知识空间信息响应

#[derive(Debug, Serialize, Deserialize, Clone, Default)]

pub struct GetSpaceResponse {

    /// 空间ID

    pub space_id: String,

    /// 空间名称

    pub name: String,

    /// 空间描述

    #[serde(skip_serializing_if = "Option::is_none")]

    pub description: Option<String>,

    /// 空间图标

    #[serde(skip_serializing_if = "Option::is_none")]

    pub icon: Option<SpaceIcon>,

}



/// 空间图标

#[derive(Debug, Serialize, Deserialize, Clone, Default)]

pub struct SpaceIcon {

    /// 图标类型

    #[serde(skip_serializing_if = "Option::is_none")]

    pub icon_type: Option<String>,

    /// 图标值

    #[serde(skip_serializing_if = "Option::is_none")]

    pub icon_value: Option<String>,

}



impl ApiResponseTrait for GetSpaceResponse {

    fn data_format() -> ResponseFormat {

        ResponseFormat::Data

    }

}



/// 获取知识空间信息

///

/// 获取指定知识空间的详细信息。

///

/// # 参数

/// * `config` - SDK 配置

/// * `space_id` - 空间ID

///

/// # 返回

/// 返回知识空间详细信息

pub async fn get_space(

    config: &Config,

    space_id: &str,

) -> SDKResult<GetSpaceResponse> {

    // 使用 ApiEndpoint 枚举生成 URL

    let api_endpoint = WikiApiV2::SpaceGet(space_id.to_string());



    // 创建 API 请求

    let api_request: ApiRequest<GetSpaceResponse> = ApiRequest::get(&api_endpoint.to_url());



    // 发送请求并提取响应数据

    let response = Transport::request(api_request, config, Some(option)).await?;

    Ok(response.data.ok_or_else(|| {

        openlark_core::error::CoreError::validation_msg("响应数据为空")

    })?)

}
