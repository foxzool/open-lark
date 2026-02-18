# openlark-hr 测试指南

## 目录结构

测试文件位置: `tests/unit/hr/{module}_tests.rs`

```
tests/unit/hr/
├── mod.rs              # 模块导出
├── attendance_tests.rs # 考勤模块测试
├── payroll_tests.rs    # 工资模块测试
├── hire_tests.rs       # 招聘模块测试
├── feishu_people_tests.rs  # CoreHR 测试
├── performance_tests.rs    # 绩效测试
├── okr_tests.rs        # OKR 测试
├── compensation_tests.rs   # 薪酬测试
└── ehr_tests.rs        # 电子人事测试
```

## 测试命名规范

### 文件命名
- 模块名 + `_tests.rs` (例如: `attendance_tests.rs`)
- 全部小写，使用下划线分隔

### 函数命名
- `test_{api_name}_{scenario}`
- 例如: `test_create_group_success`, `test_create_group_invalid_name`

## Builder 测试模式

### 基本测试
```rust
#[test]
fn test_create_group_request_builder() {
    let request = CreateGroupRequest::builder()
        .group_name("技术部考勤组".to_string())
        .build();
    
    assert_eq!(request.group_name, "技术部考勤组");
}
```

### 参数化测试
```rust
#[rstest]
#[case("open_id", "ou_123456")]
#[case("user_id", "u_123456")]
fn test_create_request_different_id_types(
    #[case] id_type: &str,
    #[case] id: &str
) {
    // 测试不同 ID 类型的处理
}
```

## HTTP Mock 测试

使用 wiremock 模拟飞书 API:

```rust
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path};

#[tokio::test]
async fn test_create_group_mock() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("POST"))
        .and(path("/open-apis/attendance/v1/groups"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(json!({"data": {"group_id": "g123"}})))
        .mount(&mock_server)
        .await;
    
    // 执行测试...
}
```

## Fixture 管理

### JSON Mock 数据
将 mock 响应保存在测试文件中:

```rust
const MOCK_GROUP_RESPONSE: &str = r#"
{
    "code": 0,
    "msg": "success",
    "data": {
        "group_id": "g123",
        "group_name": "技术部考勤组"
    }
}
"#;
```

## 代码审查 Checklist

每个 PR 必须检查:
- [ ] 测试覆盖了 Builder 基本功能
- [ ] 测试覆盖了序列化/反序列化
- [ ] 使用了 `#[rstest]` 进行参数化测试（如适用）
- [ ] 每个 API 至少一个成功场景和一个错误场景
- [ ] 测试不依赖外部网络
- [ ] 测试执行时间合理（< 30秒/文件）
- [ ] 零 clippy 警告

## 示例：完整的测试文件

```rust
//! 考勤模块单元测试

use rstest::*;
use serde_json::json;
use wiremock::{
    matchers::{body_json, method, path},
    Mock, MockServer, ResponseTemplate,
};

// 引入被测试的类型
use openlark_hr::attendance::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_group_request_builder() {
        let request = CreateGroupRequest::builder()
            .group_name("技术部".to_string())
            .time_zone("Asia/Shanghai".to_string())
            .build();
        
        assert_eq!(request.group_name, "技术部");
        assert_eq!(request.time_zone, "Asia/Shanghai");
    }

    #[rstest]
    #[case("", false)]  // 空名称应该失败
    #[case("a", true)]  // 有效名称
    #[case("a".repeat(100), false)]  // 超长名称应该失败
    fn test_group_name_validation(#[case] name: &str, #[case] should_pass: bool) {
        let result = validate_group_name(name);
        assert_eq!(result.is_ok(), should_pass);
    }
}
```
