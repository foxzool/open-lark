use crate::core::{
    config::Config,
    constants::{CUSTOM_REQUEST_ID, USER_AGENT_HEADER},
    req_option::RequestOption,
    utils::user_agent,
};
use reqwest::RequestBuilder;

/// 构建通用请求头
pub struct HeaderBuilder;

impl HeaderBuilder {
    /// 构建所有必要的请求头
    pub fn build_headers(
        mut req_builder: RequestBuilder,
        config: &Config,
        option: &RequestOption,
    ) -> RequestBuilder {
        // 1. 添加请求ID（如果有）
        if !option.request_id.is_empty() {
            req_builder = req_builder.header(CUSTOM_REQUEST_ID, &option.request_id);
        }

        // 2. 添加选项中的自定义头
        for (key, value) in &option.header {
            req_builder = req_builder.header(key, value);
        }

        // 3. 添加配置中的全局头
        for (key, value) in &config.header {
            req_builder = req_builder.header(key, value);
        }

        // 4. 添加 User-Agent
        req_builder = req_builder.header(USER_AGENT_HEADER, user_agent());

        req_builder
    }

    /// 添加单个请求头（工具方法）
    pub fn add_header(
        req_builder: RequestBuilder,
        key: &str,
        value: &str,
    ) -> RequestBuilder {
        req_builder.header(key, value)
    }

    /// 批量添加请求头（工具方法）
    pub fn add_headers(
        mut req_builder: RequestBuilder,
        headers: &[(String, String)],
    ) -> RequestBuilder {
        for (key, value) in headers {
            req_builder = req_builder.header(key, value);
        }
        req_builder
    }
}