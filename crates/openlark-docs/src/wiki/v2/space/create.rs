/// 创建知识空间

///

/// 创建一个新的知识空间。

/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space/create

use openlark_core::{

    api::{ApiRequest, ApiResponseTrait, ResponseFormat},

    config::Config,

    http::Transport,

    req_option::RequestOption,

    SDKResult,

};



use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::WikiApiV2;



/// 创建知识空间请求

#[derive(Debug, Serialize, Deserialize, Clone, Default)]

pub struct CreateSpaceRequest {

    /// 空间名称（最大100字符）

    pub name: String,

    /// 空间描述（最大500字符）

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



/// 创建知识空间响应

#[derive(Debug, Serialize, Deserialize, Clone, Default)]

pub struct CreateSpaceResponse {

    /// 空间ID

    pub space_id: String,

    /// 空间名称

    pub name: String,

    /// 空间描述

    pub description: Option<String>,

    /// 空间图标

    pub icon: Option<SpaceIcon>,

}



impl ApiResponseTrait for CreateSpaceResponse {

    fn data_format() -> ResponseFormat {

        ResponseFormat::Data

    }

}



/// 创建知识空间

///

/// 创建一个新的知识空间。

///

/// # 参数

/// * `config` - SDK 配置

/// * `request` - 创建请求

///

/// # 返回

/// 返回创建的知识空间信息

pub async fn create_space(

    config: &Config,

    request: CreateSpaceRequest,

) -> SDKResult<CreateSpaceResponse> {

    // 使用 ApiEndpoint 枚举生成 URL

    let api_endpoint = WikiApiV2::SpaceCreate;



    // 创建 API 请求

    let api_request: ApiRequest<CreateSpaceResponse> =

        ApiRequest::post(&api_endpoint.to_url()).body(serde_json::to_value(&request)?);



    // 发送请求并提取响应数据

    let response = Transport::request(api_request, config, Some(option)).await?;

    Ok(response.data.ok_or_else(|| {

        openlark_core::error::CoreError::validation_msg("响应数据为空")

    })?)

}
