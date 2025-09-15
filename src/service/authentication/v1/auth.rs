use crate::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    endpoints::Endpoints,
    http::Transport,
    req_option::RequestOption,
    standard_response::StandardResponse,
    SDKResult,
};
use serde::Deserialize;

pub struct UserInfoService {
    config: Config,
}

impl UserInfoService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取登录用户信息
    ///
    /// <https://open.feishu.cn/document/server-docs/authentication-v1/user/get>
    pub async fn get(&self, user_access_token: impl ToString) -> SDKResult<UserInfo> {
        let api_req = ApiRequest {
            api_path: Endpoints::AUTHEN_V1_USER_INFO.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            ..Default::default()
        };

        let option = RequestOption::builder()
            .user_access_token(user_access_token)
            .build();
        let api_resp: BaseResponse<UserInfo> =
            Transport::request(api_req, &self.config, Some(option)).await?;
        api_resp.into_result()
    }
}

/// 登录用户信息
#[derive(Debug, Deserialize)]
pub struct UserInfo {
    /// 用户姓名
    pub name: String,
    /// 用户英文名称
    pub en_name: String,
    /// 用户头像
    pub avatar_url: String,
    /// 用户头像 72x72
    pub avatar_thumb: String,
    /// 用户头像 240x240
    pub avatar_middle: String,
    /// 用户头像 640x640
    pub avatar_big: String,
    /// 用户在应用内的唯一标识
    pub open_id: String,
    /// 用户对ISV的唯一标识，对于同一个ISV，用户在其名下所有应用的union_id相同
    pub union_id: String,
    /// 用户邮箱
    pub email: Option<String>,
    /// 企业邮箱，请先确保已在管理后台启用飞书邮箱服务
    pub enterprise_email: Option<String>,
    /// 用户 user_id
    pub user_id: String,
    /// 用户手机号
    pub mobile: Option<String>,
    /// 当前企业标识
    pub tenant_key: String,
    /// 用户工号
    pub employee_no: String,
}

impl ApiResponseTrait for UserInfo {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[test]
fn test_user_info() {
    let json_str = r#"{
        "name": "zhangsan",
        "en_name": "zhangsan",
        "avatar_url": "www.feishu.cn/avatar/icon",
        "avatar_thumb": "www.feishu.cn/avatar/icon_thumb",
        "avatar_middle": "www.feishu.cn/avatar/icon_middle",
        "avatar_big": "www.feishu.cn/avatar/icon_big",
        "open_id": "ou-caecc734c2e3328a62489fe0648c4b98779515d3",
        "union_id": "on-d89jhsdhjsajkda7828enjdj328ydhhw3u43yjhdj",
        "email": "zhangsan@feishu.cn",
        "enterprise_email": "demo@mail.com",
        "user_id": "5d9bdxxx",
        "mobile": "+86130002883xx",
        "tenant_key": "736588c92lxf175d",
		"employee_no": "111222333"
    }"#;

    let user_info: UserInfo =
        serde_json::from_str(json_str).expect("Failed to parse test user info JSON");

    assert_eq!(user_info.name, "zhangsan")
}
