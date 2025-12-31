//! 通用构建器模式模块
//!
//! 提供统一的 Builder 模式实现，减少代码重复并保持 API 一致性。

/// 实现API请求构建器的通用宏
///
/// # 参数说明
/// - `$builder_name`: 构建器结构体名称
/// - `$request_name`: 请求结构体名称
/// - `$field`: 字段名称
/// - `$field_type`: 字段类型
///
/// # 示例
/// ```rust,ignore
/// impl_api_builder!(
///     DeleteRoleV2Builder,
///     DeleteRoleV2Request,
///     app_token: String,
///     role_id: String,
/// );
/// ```
#[macro_export]
macro_rules! impl_api_builder {
    (
        $builder_name:ident,
        $request_name:ident,
        $( $field:ident: $field_type:ty ),* $(,)?
    ) => {
        #[derive(Default)]
        pub struct $builder_name {
            request: $request_name,
        }

        impl $builder_name {
            /// 创建新的构建器实例
            pub fn new() -> Self {
                Self::default()
            }

            $(
                /// 设置字段 $field
                pub fn $field(mut self, $field: impl Into<$field_type>) -> Self {
                    self.request.$field = $field.into();
                    self
                }
            )*

            /// 构建最终的请求实例
            pub fn build(self) -> $request_name {
                self.request
            }
        }

        impl $request_name {
            /// 创建构建器实例
            pub fn builder() -> $builder_name {
                $builder_name::new()
            }
        }
    };
}

/// 实现基础API请求字段的宏
/// 包含常用的字段如 app_token, table_id 等
#[macro_export]
macro_rules! impl_base_api_fields {
    ($request_name:ident) => {
        impl $request_name {
            /// 设置应用令牌
            pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
                self.app_token = app_token.into();
                self
            }

            /// 设置表格ID（如果适用）
            #[cfg(feature = "bitable")]
            pub fn table_id(mut self, table_id: impl Into<String>) -> Self {
                self.table_id = table_id.into();
                self
            }
        }
    };
}

/// 参数验证宏
/// 用于统一验证必填参数
#[macro_export]
macro_rules! validate_required {
    ($field:expr, $error_msg:expr) => {
        if $field.is_empty() {
            return Err(openlark_core::error::CoreError::validation_msg($error_msg));
        }
    };

    ($field:expr, $error_msg:expr, $($fields:expr, $error_msgs:expr),+ $(,)?) => {
        if $field.is_empty() {
            return Err(openlark_core::error::CoreError::validation_msg($error_msg));
        }
        $(
            if $fields.is_empty() {
                return Err(openlark_core::error::CoreError::validation_msg($error_msgs));
            }
        )*
    };
}

/// 路径构建宏
/// 用于统一构建API路径
#[macro_export]
macro_rules! build_api_path {
    ($base:expr, $($segment:expr),+ $(,)?) => {
        format!("/{}/{}", $base.trim_matches('/'), [$($segment),+].join("/"))
    };
}

/// 响应数据结构体宏
/// 统一响应结构的基本字段
#[macro_export]
macro_rules! impl_response_data {
    (
        $response_name:ident,
        $data_name:ident {
            $( $field:ident: $field_type:ty ),* $(,)?
        }
    ) => {
        pub struct $data_name {
            $( pub $field: $field_type, )*
        }

        pub struct $response_name {
            pub data: $data_name,
        }

        impl openlark_core::api::ApiResponseTrait for $response_name {
            fn data_format() -> openlark_core::api::ResponseFormat {
                openlark_core::api::ResponseFormat::Data
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use openlark_core::config::Config;

    // 测试结构体定义
    #[derive(Default)]
    pub struct TestRequest {
        app_token: String,
        table_id: String,
        name: Option<String>,
    }

    // 使用宏生成构建器
    impl_api_builder!(
        TestRequestBuilder,
        TestRequest,
        app_token: String,
        table_id: String,
        name: Option<String>,
    );

    #[test]
    fn test_builder_macro() {
        let _config = Config::builder().app_id("test").app_secret("test").build();

        let request = TestRequest::builder()
            .app_token("test_token")
            .table_id("test_table")
            .name("test_name".to_string())
            .build();

        assert_eq!(request.app_token, "test_token");
        assert_eq!(request.table_id, "test_table");
        assert_eq!(request.name, Some("test_name".to_string()));
    }

    #[test]
    fn test_validate_macro() {
        // 测试必填参数验证 - 在正常函数上下文中使用
        fn test_function() -> Result<(), openlark_core::error::CoreError> {
            // 正常情况不应该返回错误
            validate_required!("valid_field", "字段不能为空");

            // 空字段情况应该返回错误
            validate_required!("", "字段不能为空");

            Ok(())
        }

        // 验证空字段确实会返回错误
        let result = test_function();
        assert!(result.is_err());
    }

    #[test]
    fn test_path_building_macro() {
        let path = build_api_path!("open-apis", "v1", "apps", "123", "tables", "456");
        assert_eq!(path, "/open-apis/v1/apps/123/tables/456");
    }

    #[test]
    fn test_response_data_macro() {
        impl_response_data!(
            TestResponse,
            TestData {
                id: String,
                name: String,
                created_time: String,
            }
        );

        // 验证结构体是否正确生成
        let data = TestData {
            id: "123".to_string(),
            name: "测试".to_string(),
            created_time: "2023-01-01".to_string(),
        };
        let response = TestResponse { data };

        assert_eq!(response.data.id, "123");
        assert_eq!(response.data.name, "测试");
        assert_eq!(response.data.created_time, "2023-01-01");
    }
}
