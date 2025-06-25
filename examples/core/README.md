# 核心API示例

本目录包含飞书SDK最常用API的基础示例，每个示例专注单一功能，帮助开发者快速上手。

## 🎯 设计原则

- **单一职责**: 每个示例只演示一个核心功能
- **简洁明了**: 代码行数控制在50行以内
- **完整可用**: 包含完整的错误处理和环境配置
- **真实场景**: 基于实际使用场景设计

## 📁 示例分类

### IM消息服务 (im/)
- `send_message.rs` - 发送文本消息
- `get_chat_history.rs` - 获取聊天历史

### 云空间服务 (drive/)
- `upload_file.rs` - 文件上传
- `download_file.rs` - 文件下载
- `list_folder.rs` - 文件夹列表

### 多维表格服务 (bitable/)
- `query_records.rs` - 记录查询
- `create_record.rs` - 创建记录

### 电子表格服务 (sheets/)
- `read_write_sheet.rs` - 电子表格读写操作
- `create_sheet.rs` - 创建新电子表格

### 考勤服务 (attendance/)
- `query_attendance.rs` - 查询考勤统计数据
- `process_leave_approval.rs` - 处理请假审批流程

### 搜索服务 (search/)
- `search_user.rs` - 用户搜索和分页查询

### 认证服务 (auth/)
- `refresh_token.rs` - 用户身份验证和令牌管理

## 🚀 使用方法

1. **环境配置**: 复制 `.env-example` 到 `.env` 并填入你的应用凭据
2. **运行示例**: `cargo run --example <示例名称>`
3. **查看文档**: 每个示例都包含详细的注释说明

## 📋 环境变量

所有示例都需要以下环境变量：

```bash
APP_ID=your_app_id
APP_SECRET=your_app_secret
USER_ACCESS_TOKEN=your_user_access_token  # 可选，某些操作需要
```

## 🔧 示例模板

每个示例都遵循统一的结构模板：

```rust
/// 功能描述
/// 
/// 使用方法：
/// cargo run --example <name>
///
/// 环境变量：
/// APP_ID=your_app_id
/// APP_SECRET=your_app_secret

use open_lark::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 环境变量加载
    dotenvy::dotenv().ok();
    
    // 2. 客户端初始化
    let client = LarkClient::builder(
        &std::env::var("APP_ID").expect("APP_ID environment variable not set"),
        &std::env::var("APP_SECRET").expect("APP_SECRET environment variable not set")
    )
    .with_enable_token_cache(true)
    .build();
    
    // 3. 核心功能演示
    demo_function(&client).await?;
    
    Ok(())
}

async fn demo_function(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    // 具体功能实现
    Ok(())
}
```

## 💡 注意事项

- 所有示例都使用 `dotenvy` 加载环境变量
- 错误处理统一使用 `Box<dyn std::error::Error>`
- 客户端配置统一使用 token cache 功能
- 输出信息包含成功和错误两种情况的处理

## 🔗 相关链接

- [飞书开放平台文档](https://open.feishu.cn/document/)
- [项目API文档](../docs/)
- [完整示例目录](../README.md)