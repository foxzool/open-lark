//! 类型安全的断言宏系统
//!
//! 提供零 unwrap() 的断言宏，确保测试失败时提供清晰的错误消息。

/// 断言 Result 为 Ok，返回值
///
/// # 示例
///
/// ```rust,ignore
/// let result: SDKResult<String> = Ok("test".to_string());
/// let value = assert_res_ok!(result, "test_case");
/// assert_eq!(value, "test");
/// ```
#[macro_export]
macro_rules! assert_res_ok {
    ($result:expr, $test_name:expr) => {
        match $result {
            Ok(value) => value,
            Err(e) => {
                panic!(
                    "{}\n  ➜ 预期: Ok(_)\n  ➜ 实际: Err({})\n  ➜ 位置: {}:{}",
                    $test_name,
                    e,
                    file!(),
                    line!()
                );
            }
        }
    };
}

/// 断言 Result 为 Err，匹配错误模式并返回错误
///
/// # 示例
///
/// ```rust,ignore
/// let result: SDKResult<()> = Err(CoreError::validation_error("field", "error"));
/// let err = assert_res_err!(result, CoreError::Validation { .. }, "test_validation");
/// ```
#[macro_export]
macro_rules! assert_res_err {
    ($result:expr, $error_pattern:pat, $test_name:expr) => {
        match $result {
            Err($error_pattern) => true,
            Ok(v) => {
                panic!(
                    "{}\n  ➜ 预期: Err({})\n  ➜ 实际: Ok({:?})\n  ➜ 位置: {}:{}",
                    $test_name,
                    stringify!($error_pattern),
                    v,
                    file!(),
                    line!()
                );
            }
            _ => false,
        }
    };
}

/// 断言 Result 为 Err，且错误消息包含指定文本
///
/// # 示例
///
/// ```rust,ignore
/// let result: SDKResult<()> = Err(CoreError::validation_error("app_token", "不能为空"));
/// let err = assert_err_contains!(result, "app_token", "test_empty_token");
/// ```
#[macro_export]
macro_rules! assert_err_contains {
    ($result:expr, $msg:expr, $test_name:expr) => {
        match $result {
            Err(e) if e.to_string().contains($msg) => e,
            Err(e) => {
                panic!(
                    "{}\n  ➜ 预期错误包含: '{}'\n  ➜ 实际错误: '{}'\n  ➜ 位置: {}:{}",
                    $test_name,
                    $msg,
                    e,
                    file!(),
                    line!()
                );
            }
            Ok(v) => {
                panic!(
                    "{}\n  ➜ 预期: Err(_)\n  ➜ 实际: Ok({:?})\n  ➜ 位置: {}:{}",
                    $test_name,
                    v,
                    file!(),
                    line!()
                );
            }
        }
    };
}

/// 断言 Option 为 Some，返回值
///
/// # 示例
///
/// ```rust,ignore
/// let value: Option<String> = Some("test".to_string());
/// let inner = assert_some!(value, "test_some");
/// assert_eq!(inner, "test");
/// ```
#[macro_export]
macro_rules! assert_some {
    ($opt:expr) => {
        match $opt {
            Some(v) => v,
            None => panic!("Expected Some, got None\n  ➜ 位置: {}:{}", file!(), line!()),
        }
    };

    ($opt:expr, $msg:expr) => {
        match $opt {
            Some(v) => v,
            None => panic!("{}: got None\n  ➜ 位置: {}:{}", $msg, file!(), line!()),
        }
    };
}

/// 断言 Option 为 None
///
/// # 示例
///
/// ```rust,ignore
/// let value: Option<String> = None;
/// assert_none!(value, "test_none");
/// ```
#[macro_export]
macro_rules! assert_none {
    ($opt:expr) => {
        match $opt {
            None => true,
            Some(v) => panic!(
                "Expected None, got Some({:?})\n  ➜ 位置: {}:{}",
                v,
                file!(),
                line!()
            ),
        }
    };

    ($opt:expr, $msg:expr) => {
        match $opt {
            None => true,
            Some(v) => panic!(
                "{}: got Some({:?})\n  ➜ 位置: {}:{}",
                $msg,
                v,
                file!(),
                line!()
            ),
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_assert_res_ok_success() {
        let result: Result<String, &str> = Ok("test".to_string());
        let value = assert_res_ok!(result, "test_ok");
        assert_eq!(value, "test");
    }

    #[test]
    #[should_panic(expected = "预期: Ok(_)")]
    fn test_assert_res_ok_panic() {
        let result: Result<String, &str> = Err("error");
        assert_res_ok!(result, "test_ok_panic");
    }

    #[test]
    fn test_assert_some_success() {
        let value: Option<String> = Some("test".to_string());
        let inner = assert_some!(value);
        assert_eq!(inner, "test");
    }

    #[test]
    #[should_panic(expected = "Expected Some, got None")]
    fn test_assert_some_panic() {
        let value: Option<String> = None;
        assert_some!(value);
    }

    #[test]
    fn test_assert_none_success() {
        let value: Option<String> = None;
        assert_none!(value);
    }

    #[test]
    #[should_panic(expected = "Expected None, got Some")]
    fn test_assert_none_panic() {
        let value: Option<String> = Some("test".to_string());
        assert_none!(value);
    }
}
