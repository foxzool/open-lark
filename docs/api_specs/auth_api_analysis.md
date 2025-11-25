# 飞书开放平台认证API规范分析

## 概述

基于官方文档分析，飞书开放平台提供了完整的认证API体系，包括企业应用认证、用户身份认证、OAuth授权和登录状态管理。

## API分类与详情

### 1. auth 项目 (企业应用认证) - v3版本

#### 1.1 tenant_access_token_internal - 自建应用获取租户访问令牌
**API端点**: `POST /open-apis/auth/v3/tenant_access_token/internal`
**用途**: 企业自建应用获取租户访问令牌，用于获取企业资源
**请求参数**:
- `app_id` (string, 必需): 应用ID
- `app_secret` (string, 必需): 应用密钥
**响应**:
- `tenant_access_token` (string): 租户访问令牌
- `expire` (integer): 过期时间(秒)
**错误码**: 40001, 40003, 42008, 99999163

#### 1.2 app_access_token_internal - 自建应用获取应用访问令牌
**API端点**: `POST /open-apis/auth/v3/app_access_token/internal`
**用途**: 企业自建应用获取应用访问令牌，用于获取应用资源
**请求参数**:
- `app_id` (string, 必需): 应用ID
- `app_secret` (string, 必需): 应用密钥
**响应**:
- `app_access_token` (string): 应用访问令牌
- `expire` (integer): 过期时间(秒)

#### 1.3 app_ticket_resend - 重新推送应用票据
**API端点**: `POST /open-apis/auth/v3/app_ticket/resend`
**用途**: 触发飞书重新推送app_ticket，不直接获取票据
**请求参数**:
- `app_id` (string, 必需): 应用ID
- `app_secret` (string, 必需): 应用密钥
**响应**:
- `code` (integer): 结果码
- `msg` (string): 结果消息

#### 1.4 tenant_access_token - 商店应用获取租户访问令牌
**API端点**: `POST /open-apis/auth/v3/tenant_access_token`
**用途**: 应用商店应用获取租户访问令牌
**请求参数**: 同1.1
**响应**: 同1.1

#### 1.5 app_access_token - 商店应用获取应用访问令牌
**API端点**: `POST /open-apis/auth/v3/app_access_token`
**用途**: 应用商店应用获取应用访问令牌
**请求参数**: 同1.2
**响应**: 同1.2

### 2. authen 项目 (用户身份认证) - v1版本

#### 2.1 user_info/get - 获取用户信息
**API端点**: `GET /open-apis/authen/v1/user_info`
**用途**: 通过user_access_token获取登录用户信息
**请求头**:
- `Authorization: Bearer <user_access_token>` (必需)
- `Content-Type: application/json; charset=utf-8` (必需)
**响应**:
- `code` (integer): 错误码，0表示成功
- `msg` (string): 错误描述
- `data` (object): 用户信息对象
  - `name` (string): 用户姓名
  - `en_name` (string): 英文姓名
  - `avatar_url` (string): 头像URL
  - `avatar_thumb` (string): 缩略图头像URL
  - `avatar_middle` (string): 中等头像URL
  - `avatar_big` (string): 大头像URL
  - `open_id` (string): 用户Open ID
  - `union_id` (string): 用户Union ID
  - `email` (string): 邮箱地址
  - `enterprise_email` (string): 企业邮箱地址
  - `user_id` (string): 用户ID
  - `mobile` (string): 手机号码
  - `tenant_key` (string): 租户密钥
  - `employee_no` (string): 员工编号

#### 2.2 access_token/create - 获取用户访问令牌
**API端点**: `POST /open-apis/authen/v1/access_token`
**用途**: 根据登录预授权码获取user_access_token
**请求参数**:
- `app_id` (string, 必需): 应用ID
- `app_secret` (string, 必需): 应用密钥
- `code` (string, 必需): 授权码
**响应**:
- `user_access_token` (string): 用户访问令牌
- `expires_in` (integer): 过期时间(秒)
- `refresh_token` (string): 刷新令牌

#### 2.3 oidc/access_token/create - OIDC获取访问令牌
**API端点**: `POST /open-apis/authen/v1/oidc/access_token`
**用途**: OIDC方式获取用户访问令牌
**请求参数**:
- `client_id` (string, 必需): 客户端ID
- `client_secret` (string, 必需): 客户端密钥
- `code` (string, 必需): 授权码
**响应**:
- `access_token` (string): 访问令牌
- `expires_in` (integer): 过期时间(秒)
- `refresh_token` (string): 刷新令牌

#### 2.4 oidc/refresh_access_token/create - OIDC刷新访问令牌
**API端点**: `POST /open-apis/authen/v1/oidc/refresh_access_token`
**用途**: OIDC方式刷新用户访问令牌
**请求参数**:
- `client_id` (string, 必需): 客户端ID
- `client_secret` (string, 必需): 客户端密钥
- `refresh_token` (string, 必需): 刷新令牌
**响应**: 同2.3

### 3. passport 项目 (登录状态管理) - v1版本

#### 3.1 session/query - 批量获取用户登录信息
**API端点**: `POST /open-apis/passport/v1/sessions/query`
**用途**: 查询用户登录信息
**请求参数**: (需进一步查询文档获取详细信息)

#### 3.2 session/logout - 退出登录
**API端点**: `POST /open-apis/passport/v1/sessions/logout`
**用途**: 用户退出登录
**请求参数**: (需进一步查询文档获取详细信息)

### 4. oauth 项目 (OAuth授权) - old版本

#### 4.1 authorization/get_index - 获取登录预授权码
**API端点**: `GET /open-apis/authen/v1/index`
**用途**: 构建登录链接，生成预授权码
**请求参数**:
- `app_id` (string, 必需): 应用ID
- `redirect_uri` (string, 必需): 重定向URI
- `state` (string, 可选): 状态参数
**响应**: 302重定向到授权页面

## 数据类型分析

### 令牌类型
1. **tenant_access_token**: 租户级别令牌，用于访问企业资源
2. **app_access_token**: 应用级别令牌，用于访问应用资源
3. **user_access_token**: 用户级别令牌，用于访问用户数据和代表用户操作

### 用户标识符
1. **user_id**: 用户在系统内的唯一ID
2. **open_id**: 用户的开放标识符
3. **union_id**: 用户的联合标识符

### 认证方式
1. **OAuth 2.0授权码模式**: 标准的OAuth流程
2. **OIDC认证**: 基于OpenID Connect的身份认证
3. **应用认证**: 基于app_id和app_secret的直接认证

## 错误处理模式

### 通用错误码范围
- **200xx**: 认证相关错误
- **400xx**: 请求参数错误
- **420xx**: 租户相关错误
- **500xx**: 系统错误

### 常见错误码
- **20001**: 请求参数无效
- **20007**: 生成用户访问令牌失败
- **20008**: 用户不存在
- **20009**: 租户未安装应用
- **20021**: 用户已离职
- **20022**: 用户已冻结
- **20023**: 用户未注册
- **20050**: 系统错误

## 安全考虑

1. **HTTPS**: 所有API调用必须使用HTTPS
2. **令牌管理**: 令牌有有效期，需要实现自动刷新机制
3. **参数验证**: 严格验证所有输入参数
4. **错误处理**: 妥善处理错误信息，避免泄露敏感信息
5. **状态参数**: OAuth流程中使用state参数防止CSRF攻击

## 实现要求

1. **类型安全**: 使用强类型定义所有请求和响应结构
2. **异步处理**: 支持异步HTTP请求
3. **重试机制**: 实现网络错误的重试逻辑
4. **日志记录**: 记录API调用和错误信息
5. **性能优化**: 合理管理HTTP连接和令牌缓存