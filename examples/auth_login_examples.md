# OpenLark 登录示例

本目录包含了使用 OpenLark SDK 进行用户认证的完整示例，演示了飞书开放平台 OAuth 授权流程的实现。

## 🚀 快速开始

### 1. 基础登录演示 (`basic_login_demo.rs`)

展示基础的 OAuth 授权流程，适合理解认证机制和API使用方法。

```bash
# 运行基础登录演示
cargo run --example basic_login_demo --features auth
```

#### 主要功能
- 🔗 生成 OAuth 授权URL
- 🎫 使用授权码获取访问令牌
- 👤 获取用户基本信息
- 🔐 令牌状态检查和管理
- 📋 完整的错误处理演示

#### 环境变量配置
```bash
export LARK_APP_ID="your_app_id"
export LARK_APP_SECRET="your_app_secret"
```

### 2. 基础登录示例 (已实现)

**注意：** 完整的Web应用示例正在开发中，当前提供的是基础命令行演示。

```bash
# 运行基础登录演示
cargo run --example basic_login_demo --features auth
```

#### 主要功能
- 🔗 生成 OAuth 授权 URL
- 🎫 模拟授权码获取流程
- 📱 完整的用户信息获取
- 🔐 令牌管理和状态检查
- 📋 实际API调用演示

#### 使用方法
1. 配置环境变量：
   ```bash
   export LARK_APP_ID="your_app_id"
   export LARK_APP_SECRET="your_app_secret"
   ```

2. 运行演示程序，程序将：
   - 生成有效的飞书授权URL
   - 演示完整的OAuth流程
   - 展示用户信息获取过程
   - 提供令牌管理指导

## 📋 认证流程说明

### OAuth 2.0 授权码流程

```
1. 用户点击登录 → 生成授权URL
      ↓
2. 跳转到飞书授权页面
      ↓
3. 用户授权 → 跳转回调URL + 授权码
      ↓
4. 使用授权码获取访问令牌
      ↓
5. 使用访问令牌获取用户信息
      ↓
6. 创建用户会话 → 登录成功
```

### 关键API调用

#### 1. 生成授权URL
```rust
let oauth_url = auth.oauth.old.authorization()
    .get_index()
    .app_id(&app_id)
    .redirect_uri("http://localhost:3030/callback")
    .scope("user:info")
    .state(&random_state)
    .build_url();
```

#### 2. 获取访问令牌
```rust
let token = auth.authen.v1().access_token()
    .create()
    .grant_type("authorization_code")
    .code(&auth_code)
    .send()
    .await?;
```

#### 3. 获取用户信息
```rust
let user_info = auth.authen.v1().user_info()
    .get()
    .user_access_token(&token.access_token)
    .user_id_type("open_id")
    .send()
    .await?;
```

## 🔧 配置说明

### 1. 创建飞书应用

1. 访问[飞书开放平台](https://open.feishu.cn/)
2. 创建企业自建应用
3. 获取 `App ID` 和 `App Secret`
4. 配置回调URL（Web示例）: `http://localhost:3030/callback`
5. 设置权限范围（Scope）: `user:info`

### 2. 环境变量配置

创建 `.env` 文件：
```env
LARK_APP_ID=your_app_id
LARK_APP_SECRET=your_app_secret
```

或者在运行时设置：
```bash
export LARK_APP_ID=your_app_id
export LARK_APP_SECRET=your_app_secret
```

## 🛡️ 安全最佳实践

### 1. 状态参数管理
```rust
// 使用随机生成的状态参数防止CSRF攻击
let state = generate_random_state();
// 存储状态参数（建议使用session或cookie）
// 回调时验证状态参数是否匹配
```

### 2. 会话管理
```rust
// 实现安全的session存储
// 设置合理的过期时间
// 实现自动刷新机制
```

### 3. 令牌存储
```rust
// 🚫 不要在客户端存储应用密钥
// 🔒 使用加密存储访问令牌
// 🕐 实现令牌自动刷新
// 🛡️ 设置适当的CORS策略
```

### 4. 错误处理
```rust
// 优雅处理网络错误
// 提供用户友好的错误信息
// 实现重试机制
// 记录详细的错误日志
```

## 📊 错误处理

### 常见错误类型

1. **授权失败**
   - 用户拒绝授权
   - 权限不足
   - 应用配置错误

2. **令牌错误**
   - 授权码过期
   - 刷新令牌失效
   - 令牌权限不足

3. **网络错误**
   - 连接超时
   - API限流
   - 服务器错误

### 错误处理示例
```rust
match result {
    Ok(response) => {
        println!("✅ 操作成功: {:?}", response);
    }
    Err(AuthError::ConfigError(msg)) => {
        println!("❌ 配置错误: {}", msg);
    }
    Err(AuthError::NetworkError(e)) => {
        println!("🌐 网络错误: {}", e);
        // 实现重试逻辑
    }
    Err(AuthError::APIError { code, message }) => {
        println!("🚫 API错误 [{}]: {}", code, message);
        // 根据错误码处理具体问题
    }
}
```

## 🔗 相关资源

- [飞书开放平台文档](https://open.feishu.cn/)
- [OpenLark SDK 文档](../docs/auth-api-guide.md)
- [OAuth 2.0 规范](https://tools.ietf.org/html/rfc6749)
- [Warp Web框架](https://github.com/seanmonstar/warp)

## 💡 高级用法

### 1. 自定义回调处理
```rust
// 实现自定义的业务逻辑
// 如用户信息同步、权限分配等
```

### 2. 多租户支持
```rust
// 支持多个应用配置
// 实现动态应用切换
```

### 3. 权限扩展
```rust
// 请求更多权限范围
// 实现渐进式授权
```

### 4. 监控和分析
```rust
// 添加登录行为分析
// 实现安全监控
// 记录审计日志
```

## 🤝 贡献指南

欢迎提交Pull Request来改进这些示例：

1. Fork 项目
2. 创建功能分支
3. 提交变更
4. 发起 Pull Request

## 📝 许可证

本项目采用 Apache License 2.0 许可证。