use std::{collections::HashSet, io::Read, marker::PhantomData};

use log::debug;
use serde_json::Value;
use ureq::Request;

use crate::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponse, ApiResponseTrait, BaseResp, RawResponse, ResponseFormat},
    app_ticket_manager::apply_app_ticket,
    config::Config,
    constants::*,
    error::LarkAPIError,
    req_option::RequestOption,
    req_translator::ReqTranslator,
    SDKResult,
};

pub struct Transport<T> {
    phantom_data: PhantomData<T>,
}

impl<T: ApiResponseTrait> Transport<T> {
    pub fn request(
        mut req: ApiRequest,
        config: &Config,
        option: Option<RequestOption>,
    ) -> Result<ApiResponse<T>, LarkAPIError> {
        let option = option.unwrap_or_default();

        if req.supported_access_token_types.is_empty() {
            req.supported_access_token_types = vec![AccessTokenType::None];
        }

        validate_token_type(&req.supported_access_token_types, &option)?;
        let access_token_type = determine_token_type(
            &req.supported_access_token_types,
            &option,
            config.enable_token_cache,
        );
        validate(config, &option, access_token_type)?;

        Self::do_request(req, access_token_type, config, option)
    }

    fn do_request(
        mut http_req: ApiRequest,
        access_token_type: AccessTokenType,
        config: &Config,
        option: RequestOption,
    ) -> SDKResult<ApiResponse<T>> {
        let req = ReqTranslator::translate(&mut http_req, access_token_type, config, &option)?;
        debug!("Req:{:?}", req);
        let resp = Self::do_send(req, http_req.body)?;
        debug!("Res:{:?}", resp);

        if let ApiResponse::Error(code_error) = &resp {
            let code = code_error.code;
            if code == ERR_CODE_APP_TICKET_INVALID {
                apply_app_ticket(config)?;
            }
        }

        Ok(resp)
    }

    pub fn do_send(raw_request: Request, body: Vec<u8>) -> SDKResult<ApiResponse<T>> {
        match raw_request.send_bytes(&body) {
            Ok(response) => {
                let status_code = response.status();
                let header = response.headers_names();
                match T::data_format() {
                    ResponseFormat::Data => {
                        let raw_body: Value = response.into_json()?;
                        debug!("raw_body: {:?}", raw_body);
                        match serde_json::from_value::<BaseResp<T>>(raw_body) {
                            Ok(base_resp) => {
                                if base_resp.raw_response.code == 0 {
                                    Ok(ApiResponse::Success {
                                        data: base_resp.data,
                                        status_code,
                                        header,
                                    })
                                } else {
                                    Ok(ApiResponse::Error(base_resp.raw_response))
                                }
                            }
                            Err(err) => Err(LarkAPIError::DeserializeError(err)),
                        }
                    }
                    ResponseFormat::Flatten => {
                        let raw_body: Value = response.into_json()?;
                        debug!("raw_body: {:?}", raw_body);
                        match serde_json::from_value::<T>(raw_body) {
                            Ok(data) => Ok(ApiResponse::Success {
                                data,
                                status_code,
                                header,
                            }),
                            Err(err) => Err(LarkAPIError::DeserializeError(err)),
                        }
                    }
                    // 处理二进制数据
                    ResponseFormat::Binary => {
                        let len: usize =
                            response.header("Content-Length").unwrap().parse().unwrap();

                        let file_name = response
                            .header("Content-Disposition")
                            .unwrap_or_default()
                            .to_string();
                        let file_name = decode_file_name(&file_name).unwrap_or_default();
                        let mut bytes: Vec<u8> = Vec::with_capacity(len);
                        response
                            .into_reader()
                            .take(20_971_520)
                            .read_to_end(&mut bytes)?;

                        let data = T::from_binary(file_name, bytes).unwrap();
                        Ok(ApiResponse::Success {
                            data,
                            status_code,
                            header,
                        })
                    }
                }
            }
            Err(err) => {
                println!("err: {:?}", err);
                let resp = err.into_response().unwrap();
                // 返回4xx或5xx状态码， 但可以读取响应体
                match resp.into_json::<RawResponse>() {
                    Ok(code_msg) => Ok(ApiResponse::Error(code_msg)),
                    Err(err) => Err(LarkAPIError::IOErr(err)),
                }
            }
        }
    }
}

fn validate_token_type(
    access_token_types: &[AccessTokenType],
    option: &RequestOption,
) -> Result<(), LarkAPIError> {
    if !access_token_types.is_empty() {
        return Ok(());
    }

    let access_token_type = access_token_types[0];

    if access_token_type == AccessTokenType::Tenant && !option.user_access_token.is_empty() {
        return Err(LarkAPIError::IllegalParamError(
            "tenant token type not match user access token".to_string(),
        ));
    }

    if access_token_type == AccessTokenType::App && !option.tenant_access_token.is_empty() {
        return Err(LarkAPIError::IllegalParamError(
            "user token type not match tenant access token".to_string(),
        ));
    }

    Ok(())
}

fn determine_token_type(
    access_token_types: &[AccessTokenType],
    option: &RequestOption,
    enable_token_cache: bool,
) -> AccessTokenType {
    if !enable_token_cache {
        if !option.user_access_token.is_empty() {
            return AccessTokenType::User;
        }
        if !option.tenant_access_token.is_empty() {
            return AccessTokenType::Tenant;
        }
        if !option.app_access_token.is_empty() {
            return AccessTokenType::App;
        }

        return AccessTokenType::None;
    }
    let mut accessible_token_type_set: HashSet<AccessTokenType> = HashSet::new();
    let mut access_token_type = access_token_types[0];

    for t in access_token_types {
        if *t == AccessTokenType::Tenant {
            access_token_type = *t; // 默认值
        }
        accessible_token_type_set.insert(*t);
    }

    if !option.tenant_key.is_empty() && accessible_token_type_set.contains(&AccessTokenType::Tenant)
    {
        access_token_type = AccessTokenType::Tenant;
    }

    if !option.user_access_token.is_empty()
        && accessible_token_type_set.contains(&AccessTokenType::User)
    {
        access_token_type = AccessTokenType::User;
    }

    access_token_type
}

fn validate(
    config: &Config,
    option: &RequestOption,
    access_token_type: AccessTokenType,
) -> Result<(), LarkAPIError> {
    if config.app_id.is_empty() {
        return Err(LarkAPIError::IllegalParamError(
            "AppId is empty".to_string(),
        ));
    }

    if config.app_secret.is_empty() {
        return Err(LarkAPIError::IllegalParamError(
            "AppSecret is empty".to_string(),
        ));
    }

    if !config.enable_token_cache {
        if access_token_type == AccessTokenType::None {
            return Ok(());
        }
        if option.user_access_token.is_empty()
            && option.tenant_access_token.is_empty()
            && option.app_access_token.is_empty()
        {
            return Err(LarkAPIError::IllegalParamError(
                "accessToken is empty".to_string(),
            ));
        }
    }

    if config.app_type == AppType::Marketplace
        && access_token_type == AccessTokenType::Tenant
        && option.tenant_key.is_empty()
    {
        return Err(LarkAPIError::IllegalParamError(
            "accessToken is empty".to_string(),
        ));
    }

    if access_token_type == AccessTokenType::User && option.user_access_token.is_empty() {
        return Err(LarkAPIError::IllegalParamError(
            "user access token is empty".to_string(),
        ));
    }

    if option.header.contains_key(HTTP_HEADER_KEY_REQUEST_ID) {
        return Err(LarkAPIError::IllegalParamError(format!(
            "use {} as header key is not allowed",
            HTTP_HEADER_KEY_REQUEST_ID
        )));
    }
    if option.header.contains_key(HTTP_HEADER_REQUEST_ID) {
        return Err(LarkAPIError::IllegalParamError(format!(
            "use {} as header key is not allowed",
            HTTP_HEADER_REQUEST_ID
        )));
    }

    Ok(())
}

/// 解析文件名
fn decode_file_name(file_name: &str) -> Option<String> {
    let parts = file_name.split(';');

    for part in parts {
        if part.trim().starts_with("filename*=") {
            let filename = part
                .trim()
                .strip_prefix("filename*=UTF-8''")
                .unwrap_or("")
                .to_string();
            return Some(filename);
        }
    }

    None
}

#[cfg(test)]
mod test {
    use crate::core::http::decode_file_name;

    #[test]
    fn test_decode_file_name() {
        let raw = "attachment; filename=\"upload_all.rs\"; filename*=UTF-8''upload_all.rs";
        let file_name = decode_file_name(raw).unwrap();
        assert_eq!(file_name, "upload_all.rs");
    }
}
