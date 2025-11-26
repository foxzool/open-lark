# OpenLark Auth 模块测试

本目录包含 openlark-auth 认证模块的测试套件。

## 测试结构

### 核心测试文件

- **`auth_api_tests.rs`** - 认证API接口测试
  - 测试租户访问令牌获取 (`tenant_access_token/internal`)
  - 测试应用访问令牌获取 (`app_access_token/internal`)
  - 测试用户信息获取 (`user_info.get`)
  - 错误场景测试
  - 并发请求测试

- **`auth_mock_tests.rs`** - Mock服务器测试
  - 基于wiremock的Mock服务器测试
  - API错误响应测试
  - 网络错误处理测试
  - OAuth授权测试

## 运行测试

```bash
# 运行所有认证模块测试
cargo test -p openlark-auth

# 运行特定测试文件
cargo test -p openlark-auth --test auth_api_tests
cargo test -p openlark-auth --test auth_mock_tests

# 运行库单元测试
cargo test -p openlark-auth --lib
```

## 测试覆盖范围

当前测试覆盖以下API接口：

### auth v3 - 企业应用认证
- ✅ `tenant_access_token/internal` - 自建应用获取租户访问令牌
- ✅ `app_access_token/internal` - 自建应用获取应用访问令牌

### authen v1 - 用户身份认证
- ✅ `user_info.get` - 获取登录用户信息

### 错误场景
- ✅ 认证失败处理
- ✅ 网络错误处理
- ✅ 并发请求处理

## 测试架构

测试基于项目的实际AuthServices架构：
- 使用 `openlark_auth::prelude::*` 导入
- 遵循 Project-Version-Resource (PVR) 设计模式
- 使用 wiremock 进行HTTP Mock
- 支持异步测试执行

## 测试数据

测试使用预定义的测试数据：
- 标准测试应用配置
- 模拟用户信息响应
- 各种错误场景响应
- 测试令牌生成

## 贡献

添加新测试时请遵循以下原则：
1. 使用现有的Mock工具或创建新的Mock响应
2. 遵循PVR架构模式进行API调用
3. 包含成功和错误场景测试
4. 确保测试异步且可重复运行
5. 使用有意义的测试名称和描述