//! HR 模块测试模板
//!
//! 此文件作为新测试文件的模板参考

use rstest::*;

/// 示例测试模块
#[cfg(test)]
mod template_tests {
    use super::*;

    #[test]
    fn test_example() {
        // TODO: 实现实际测试
        assert!(true);
    }

    #[rstest]
    #[case(1)]
    #[case(2)]
    #[case(3)]
    fn test_with_rstest(#[case] value: i32) {
        assert!(value > 0);
    }
}
