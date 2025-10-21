/// 标准化响应处理特征
///
/// 为了解决项目中错误处理方式不统一的问题，提供统一的响应处理接口。
/// 这个特征允许不同的响应类型以一致的方式处理成功和错误情况。
use crate::core::{api_resp::BaseResponse, error::LarkAPIError, SDKResult};

/// 标准响应处理特征
///
/// 提供统一的响应处理方法，确保所有API调用都有一致的错误处理体验
pub trait StandardResponse<T> {
    /// 将响应转换为统一的Result格式
    ///
    /// # Returns
    ///
    /// * `Ok(T)` - 当响应成功且包含数据时
    /// * `Err(LarkAPIError)` - 当响应失败或数据处理出错时
    fn into_result(self) -> SDKResult<T>;

    /// 获取响应数据，失败时返回默认值
    ///
    /// 这个方法保持了向后兼容性，对应原来的 `unwrap_or_default()` 行为
    fn data_or_default(self) -> T
    where
        T: Default;
}

impl<T> StandardResponse<T> for BaseResponse<T> {
    fn into_result(self) -> SDKResult<T> {
        if self.success() {
            match self.data {
                Some(data) => Ok(data),
                None => Err(LarkAPIError::DataError(
                    "Response succeeded but contains no data".to_string(),
                )),
            }
        } else {
            Err(LarkAPIError::APIError {
                code: self.code(),
                msg: self.msg().to_string(),
                error: self.err().map(|e| format!("{e:?}")),
            })
        }
    }

    fn data_or_default(self) -> T
    where
        T: Default,
    {
        if self.success() {
            self.data.unwrap_or_default()
        } else {
            T::default()
        }
    }
}

impl<T> StandardResponse<T> for SDKResult<BaseResponse<T>> {
    fn into_result(self) -> SDKResult<T> {
        match self {
            Ok(response) => response.into_result(),
            Err(error) => Err(error),
        }
    }

    fn data_or_default(self) -> T
    where
        T: Default,
    {
        match self {
            Ok(response) => response.data_or_default(),
            Err(_) => T::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::api_resp::{BaseResponse, RawResponse};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
    struct TestData {
        value: String,
    }

    #[test]
    fn test_successful_response_with_data() {
        let response = BaseResponse {
            raw_response: RawResponse {
                code: 0,
                msg: "success".to_string(),
                err: None,
            },
            data: Some(TestData {
                value: "test".to_string(),
            }),
        };

        let result = response.into_result();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().value, "test");
    }

    #[test]
    fn test_successful_response_without_data() {
        let response: BaseResponse<TestData> = BaseResponse {
            raw_response: RawResponse {
                code: 0,
                msg: "success".to_string(),
                err: None,
            },
            data: None,
        };

        let result = response.into_result();
        assert!(result.is_err());
        if let Err(LarkAPIError::DataError(msg)) = result {
            assert!(msg.contains("no data"));
        } else {
            panic!("Expected DataError");
        }
    }

    #[test]
    fn test_failed_response() {
        let response: BaseResponse<TestData> = BaseResponse {
            raw_response: RawResponse {
                code: -1,
                msg: "error".to_string(),
                err: None,
            },
            data: None,
        };

        let result = response.into_result();
        assert!(result.is_err());
    }

    #[test]
    fn test_data_or_default_success() {
        let response = BaseResponse {
            raw_response: RawResponse {
                code: 0,
                msg: "success".to_string(),
                err: None,
            },
            data: Some(TestData {
                value: "test".to_string(),
            }),
        };

        let data = response.data_or_default();
        assert_eq!(data.value, "test");
    }

    #[test]
    fn test_data_or_default_no_data() {
        let response: BaseResponse<TestData> = BaseResponse {
            raw_response: RawResponse {
                code: 0,
                msg: "success".to_string(),
                err: None,
            },
            data: None,
        };

        let data = response.data_or_default();
        assert_eq!(data, TestData::default());
    }

    #[test]
    fn test_data_or_default_failure() {
        let response: BaseResponse<TestData> = BaseResponse {
            raw_response: RawResponse {
                code: -1,
                msg: "error".to_string(),
                err: None,
            },
            data: None,
        };

        let data = response.data_or_default();
        assert_eq!(data, TestData::default());
    }
}
