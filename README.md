[![crates.io](https://img.shields.io/crates/v/open-lark)](https://crates.io/crates/open-lark)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/Seldom-SE/seldom_pixel#license)
[![Quality](https://github.com/foxzool/open-lark/actions/workflows/quality.yml/badge.svg)](https://github.com/foxzool/open-lark/actions/workflows/quality.yml)
[![Documentation](https://docs.rs/open-lark/badge.svg)](https://docs.rs/open-lark)
![Discord Shield](https://discord.com/api/guilds/1319490473060073532/widget.png?style=shield)

# 飞书开放平台非官方SDK, 个人开发, 请谨慎使用

支持自定义机器人、长连接机器人、云文档、飞书卡片、消息、群组、招聘管理等API调用。

## 🎉 最新更新

### v0.13.1 WebSocket 关键修复 🔧
- **🐛 关键问题修复**: 修复 WebSocket frame payload 解析回归问题
- **📡 完整示例**: websocket_client.rs 添加完整的连接和事件处理功能
- **🧪 测试覆盖**: 完整的单包和多包消息处理测试

### v0.12.0+ 全面文档完善与质量提升 📚
- **📝 完整中文文档**: 43个服务模块100%文档覆盖，企业级文档标准
- **✅ 文档测试保证**: 72个文档测试100%通过，确保示例代码正确性
- **🎯 开发者友好**: 丰富的使用示例、最佳实践指导、层次化文档结构
- **🔧 零警告构建**: 消除所有文档警告，提供清洁的开发体验

### v0.11.0+ 现代化Builder模式与统一错误处理 ⭐
- **🏗️ Builder模式**: 流畅链式调用，支持Contact、IM、Drive等核心服务，完全向后兼容
- **📦 统一错误处理**: StandardResponse机制，详细错误信息与智能建议
- **⚡ 类型安全**: ExecutableBuilder特征，编译时参数验证，现代化异步执行

### v0.11.0 招聘管理系统 🚀
- **🎯 企业级招聘**: 6大服务模块，100+ API接口，完整业务流程
- **📊 智能管理**: 人才库、面试、Offer、内推等全功能覆盖

### v0.8.0 AI能力支持 🤖  
- **🧠 智能识别**: 17种证件文档、OCR、语音识别、机器翻译
- **📊 22个AI API**: 覆盖企业智能化应用全场景

### v0.6.0 企业级错误处理 🛡️
- **智能分析**: 错误自动分析、重试机制、实时监控
- **完整重构**: 68个文件优化，消除2000+个clone调用

## 功能特性 (Feature Flags)

从 v0.12.0 开始，open-lark 支持按需编译，你可以只启用需要的服务模块，显著减少编译时间和二进制文件大小。

### 默认功能
```toml
[dependencies]
open-lark = "0.13.2"  # 包含: im, cloud-docs, contact, group, authentication, search
```

### 仅使用云文档 API
```toml
[dependencies]
open-lark = { version = "0.13.2", default-features = false, features = ["cloud-docs"] }
```

### 仅使用消息 API  
```toml
[dependencies]
open-lark = { version = "0.13.2", default-features = false, features = ["im"] }
```

### 启用所有功能
```toml
[dependencies]
open-lark = { version = "0.13.2", features = ["full"] }
```

### 可用的功能模块

#### 核心服务 (默认启用)
- `im` - 即时消息
- `cloud-docs` - 云文档 (包含 drive, sheets, bitable 等)
- `contact` - 通讯录
- `group` - 群组管理
- `authentication` - 认证服务
- `search` - 搜索功能

#### 高级服务 (按需启用)
- `hire` - 招聘管理
- `calendar` - 日历服务
- `approval` - 审批流程
- `attendance` - 考勤管理
- `vc` - 视频会议
- `ai` - AI 服务
- `helpdesk` - 服务台
- 更多服务请参考 `Cargo.toml`

## 使用

将`.env-example`文件重命名为`.env`，并填写相关配置。

### 快速开始 - 招聘管理

```rust,ignore
use open_lark::prelude::*;
use open_lark::service::hire::models::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LarkClient::builder("your_app_id", "your_app_secret")
        .with_app_type(AppType::SelfBuild)
        .build();

    // 获取职位列表
    let job_request = JobListRequest {
        page_size: Some(50),
        page_token: None,
        status: Some("active".to_string()),
        ..Default::default()
    };
    let jobs = client.hire.recruitment_config.job.list_jobs(job_request, None).await?;
    println!("职位列表: {:?}", jobs.data);

    // 获取人才库列表
    let pool_request = TalentPoolListRequest {
        page_size: Some(20),
        ..Default::default()
    };
    let pools = client.hire.candidate_management.talent_pool.list_pools(pool_request, None).await?;
    println!("人才库列表: {:?}", pools.data);

    // 创建人才
    let talent_request = TalentCreateRequest {
        name: "张三".to_string(),
        email: Some("zhangsan@example.com".to_string()),
        phone: Some("13800138000".to_string()),
        ..Default::default()
    };
    let talent = client.hire.candidate_management.talent.create_talent(talent_request, None).await?;
    println!("创建人才: {:?}", talent.data);

    // 获取内推列表
    let referral_request = ReferralListRequest {
        page_size: Some(30),
        ..Default::default()
    };
    let referrals = client.hire.get_candidates.referral.list_referrals(referral_request, None).await?;
    println!("内推列表: {:?}", referrals.data);

    // 查询内推账户余额
    let user_id = "user_123456";
    let balance = client.hire.referral_account.get_balance(user_id, None).await?;
    println!("账户余额: {:?}", balance.data);

    Ok(())
}
```

### 快速开始 - 企业级错误处理

```rust,ignore
use open_lark::prelude::*;
use open_lark::core::{
    error_helper::ErrorHelper,
    error_metrics::ErrorMonitor,
    retry_middleware::{RetryMiddleware, RetryConfig},
    error_logger::{ErrorLogger, LoggerBuilder, LogLevel},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 设置监控和日志
    let monitor = ErrorMonitor::default();
    let logger = LoggerBuilder::new()
        .min_level(LogLevel::Info)
        .json_format()
        .build();

    // 设置智能重试策略
    let retry_middleware = RetryMiddleware::new(
        RetryConfig::new()
            .enabled(true)
            .server_errors_only()
    );

    let client = LarkClient::builder("your_app_id", "your_app_secret").build();

    // 执行API调用（带自动重试）
    let result = retry_middleware.execute(|| async {
        client.im.v1.message.create(&request).await
    }).await;

    // 智能错误处理
    match result {
        Ok(response) => println!("✅ 成功: {:?}", response),
        Err(error) => {
            // 记录错误
            monitor.record_error(error.clone());
            logger.log_api_error(&error);
            
            // 智能错误分析
            println!("❌ {}", error.user_friendly_message());
            let advice = ErrorHelper::handle_error(&error);
            for action in &advice.actions {
                println!("  💡 {}", action);
            }
        }
    }

    // 查看错误统计
    let stats = monitor.get_statistics();
    stats.print_detailed();

    Ok(())
}
```

### 快速开始 - 组织架构管理

```rust,ignore
use open_lark::prelude::*;
use open_lark::service::directory::v1::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LarkClient::builder("your_app_id", "your_app_secret")
        .with_app_type(AppType::SelfBuild)
        .build();

    // 创建部门
    let dept_request = CreateDepartmentRequest::builder()
        .name("技术部")
        .en_name("Technology Department")
        .user_id_type(UserIdType::UserId)
        .department_id_type(DepartmentIdType::DepartmentId)
        .build();
    let dept = client.directory.v1.department.create(dept_request, None).await?;
    println!("创建部门: {:?}", dept.data);

    // 创建员工
    let emp_request = CreateEmployeeRequest::builder()
        .name("张三")
        .email("zhangsan@example.com")
        .job_title("软件工程师")
        .user_id_type(UserIdType::UserId)
        .build();
    let emp = client.directory.v1.employee.create(emp_request, None).await?;
    println!("创建员工: {:?}", emp.data);

    // 搜索员工
    let search_request = SearchEmployeeRequest::builder("张")
        .page_size(10)
        .user_id_type(UserIdType::UserId)
        .build();
    let employees = client.directory.v1.employee.search(search_request, None).await?;
    println!("搜索结果: {:?}", employees.data);

    Ok(())
}
```

### 快速开始 - 考勤模块

```rust,ignore
use open_lark::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建客户端
    let client = LarkClient::builder("your_app_id", "your_app_secret").build();

    // 查询所有班次
    let shifts_req = ListShiftsRequest {
        employee_type: "employee_id".to_string(),
        page_size: Some(20),
        ..Default::default()
    };
    
    let shifts_resp = client.attendance.v1.shift.list(shifts_req, None).await?;
    println!("班次列表: {:?}", shifts_resp.data);

    // 查询打卡记录
    let user_task_req = QueryUserTaskRequest {
        employee_type: "employee_id".to_string(),
        user_ids: vec!["employee_123".to_string()],
        check_date_from: "2024-06-01".to_string(),
        check_date_to: "2024-06-30".to_string(),
        ..Default::default()
    };
    
    let user_task_resp = client.attendance.v1.user_task.query(user_task_req, None).await?;
    println!("打卡记录: {:?}", user_task_resp.data);

    Ok(())
}
```

### 考勤事件监听

```rust,ignore
use open_lark::event::EventDispatcherHandler;

let handler = EventDispatcherHandler::builder()
    .register_p2_attendance_user_task_updated_v1(|event| {
        println!("收到打卡流水事件: {:?}", event);
    })
    .register_p2_attendance_user_task_status_change_v1(|event| {
        println!("收到任务状态变更事件: {:?}", event);
    })
    .build();
```

### AI 能力使用示例

```rust,ignore
use open_lark::prelude::*;
use open_lark::service::ai::models::{
    FileRecognizeRequest, LanguageDetectRequest, TranslateRequest,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LarkClient::builder("your_app_id", "your_app_secret").build();

    // 智能文档处理 - 简历解析
    let resume_request = FileRecognizeRequest {
        file: "resume_file_token_or_base64".to_string(),
    };
    let resume_result = client.ai.document_ai.parse_resume(resume_request, None).await?;
    println!("简历解析结果: {:?}", resume_result.data);

    // OCR文字识别
    let ocr_request = FileRecognizeRequest {
        file: "image_file_token_or_base64".to_string(),
    };
    let ocr_result = client.ai.optical_char_recognition.basic_recognize(ocr_request, None).await?;
    println!("OCR识别结果: {:?}", ocr_result.data);

    // 语种检测
    let detect_request = LanguageDetectRequest {
        text: "Hello, how are you?".to_string(),
    };
    let detect_result = client.ai.translation.detect(detect_request, None).await?;
    println!("语种检测结果: {:?}", detect_result.data);

    // 文本翻译
    let translate_request = TranslateRequest {
        source_language: Some("en".to_string()),
        target_language: "zh".to_string(),
        text: "Hello, how are you?".to_string(),
    };
    let translate_result = client.ai.translation.translate(translate_request, None).await?;
    println!("翻译结果: {:?}", translate_result.data);

    Ok(())
}
```

## 🏗️ 现代化Builder模式与统一错误处理 - v0.11.0+ 重大升级 ⭐

### 🎯 核心特性

我们在v0.11.0+版本中全面升级了API设计，引入了现代化Builder模式和StandardResponse统一错误处理机制：

- **🔗 流畅链式调用**: 更直观的API使用体验
- **🛡️ 类型安全保证**: 编译时参数验证，减少运行时错误
- **📦 统一错误处理**: `StandardResponse.into_result()`提供一致的错误处理
- **⚡ 异步执行支持**: `ExecutableBuilder`特征实现现代化异步模式
- **🔄 完全向后兼容**: 传统API调用方式仍然完全支持
- **🎯 条件构建**: 更优雅地处理可选参数和动态配置

### 🚀 Contact服务Builder模式示例

#### 传统方式 (仍然支持)
```rust,ignore
let request = CreateUserRequest {
    user: user.clone(),
    user_id_type: Some("open_id".to_string()),
    department_id_type: Some("open_department_id".to_string()),
};

let response = client.contact.v3.user.create(&request).await?;
```

#### 新Builder模式 (推荐)
```rust,ignore
use open_lark::core::trait_system::ExecutableBuilder;

let response = client
    .contact
    .v3
    .user
    .create_user_builder()
    .user(user.clone())
    .user_id_type("open_id")
    .department_id_type("open_department_id")
    .execute(&client.contact.v3.user)
    .await?;
```

### 📁 IM文件和图片服务Builder模式 (v0.11.0+ 新增)

#### 文件上传Builder模式
```rust,ignore
use open_lark::core::trait_system::ExecutableBuilder;

// 文件上传
let upload_result = client
    .im
    .v1
    .file
    .upload_builder()
    .file_type("pdf")
    .file_name("document.pdf")
    .file_data(file_data)
    .execute(&client.im.v1.file)
    .await?;

println!("文件上传成功: {}", upload_result.file_key);

// 文件下载
let download_result = client
    .im
    .v1
    .file
    .download_builder()
    .file_key(&upload_result.file_key)
    .execute(&client.im.v1.file)
    .await?;

println!("文件下载成功，大小: {} 字节", download_result.data.len());
```

#### 图片上传Builder模式
```rust,ignore
// 图片上传
let image_upload = client
    .im
    .v1
    .image
    .upload_builder()
    .image_type("png")
    .image_data(image_bytes)
    .execute(&client.im.v1.image)
    .await?;

// 图片下载
let image_download = client
    .im
    .v1
    .image
    .download_builder()
    .image_key(&image_upload.image_key)
    .execute(&client.im.v1.image)
    .await?;
```

### 🔄 批量操作和高级用法

```rust,ignore
// 条件构建示例
let mut builder = client
    .contact
    .v3
    .user
    .create_user_builder()
    .user(user.clone());

// 条件性添加参数
if use_open_id {
    builder = builder.user_id_type("open_id");
}

if include_department {
    builder = builder.department_id_type("open_department_id");
}

let response = builder.execute(&client.contact.v3.user).await?;

// 批量文件上传示例
let files = [("txt", "file1.txt", data1), ("pdf", "file2.pdf", data2)];
for (file_type, file_name, file_data) in files {
    let result = client
        .im
        .v1
        .file
        .upload_builder()
        .file_type(file_type)
        .file_name(file_name)
        .file_data(file_data)
        .execute(&client.im.v1.file)
        .await?;
    
    println!("上传成功: {} -> {}", file_name, result.file_key);
}
```

### 🛡️ 统一错误处理

StandardResponse机制提供详细的错误信息和智能处理建议：

```rust,ignore
use open_lark::core::error::LarkAPIError;

match result {
    Ok(response) => {
        println!("✅ 操作成功: {:?}", response);
    }
    Err(e) => {
        match &e {
            LarkAPIError::APIError { code, msg, .. } => {
                println!("API错误 - 代码: {}, 消息: {}", code, msg);
                
                // 根据错误码提供具体建议
                match *code {
                    429 => println!("💡 建议: 请求频率过高，建议稍后重试"),
                    403 => println!("💡 建议: 权限不足，请检查应用权限配置"),
                    413 => println!("💡 建议: 文件太大，请压缩后重试"),
                    415 => println!("💡 建议: 不支持的文件类型"),
                    _ => println!("💡 建议: 检查网络连接和API配置"),
                }
            }
            LarkAPIError::DataError(msg) => {
                println!("数据错误: {}", msg);
                println!("💡 建议: 检查输入数据格式和内容");
            }
            _ => {
                println!("其他错误: {}", e);
                println!("💡 建议: 查看详细日志获取更多信息");
            }
        }
    }
}
```

### 📚 完整示例代码

查看以下示例了解完整的使用方法：

- **[unified_builder_pattern.rs](examples/api/unified_builder_pattern.rs)** - Builder模式基础用法
- **[im_modern_builder_pattern.rs](examples/api/im_modern_builder_pattern.rs)** - IM消息服务Builder模式
- **[im_file_image_builder_pattern.rs](examples/api/im_file_image_builder_pattern.rs)** - IM文件图片服务Builder模式
- **[drive_builder_pattern.rs](examples/api/drive_builder_pattern.rs)** - Drive服务Builder模式

### 🎯 最佳实践总结

1. **🏗️ 新项目推荐使用Builder模式** - 提供更好的开发体验
2. **🔄 渐进式迁移** - 现有代码可以逐步迁移到Builder模式
3. **🛡️ 统一错误处理** - 使用`StandardResponse.into_result()`获得一致的错误体验
4. **⚡ 异步优先** - 使用`.execute()`方法获得现代化异步体验
5. **🎯 类型安全** - 利用Rust类型系统防止参数错误
6. **📝 文档完整** - 每个Builder都有详细的使用示例和测试

## 已完成

### 认证与授权

- [x] 自建应用获取 tenant_access_token

### 企业信息

- [x] 获取企业信息
- [x] 获取企业席位信息

### 认证信息

- [x] 获取认证信息

### 自定义机器人

- [x] 发送消息
- [x] 签名验证

### 长连接机器人

- [x] 接收事件推送

### 云文档

#### 云空间

- 文件夹
    - [x] 获取我的空间元信息
    - [x] 获取文件夹元信息
    - [x] 新建文件夹
    - [x] 获取文件夹下的清单
- 上传
    - [x] 上传文件
- 下载
    - [x] 下载文件

#### 权限

- [x] 获取云文档权限设置
- [x] 更新云文档权限设置

#### 电子表格

- 表格
    - [x] 创建表格
    - [x] 修改电子表格属性
    - [x] 获取电子表格信息
- 工作表
    - [x] 查询工作表
    - [x] 获取工作表
    - [x] 操作工作表
    - [x] 更新工作表属性
- 行列
    - [x] 增加行列
    - [x] 插入行列
    - [x] 更新行列
    - [x] 移动行列
    - [x] 删除行列
- 单元格
    - [x] 插入数据
    - [x] 追加数据
    - [x] 读取单个范围
    - [x] 向单个范围写入数据
    - [x] 读取多个范围
    - [x] 向多个范围写入数据
    - [x] 设置单元格样式
    - [x] 批量设置单元格样式
    - [x] 写入图片
    - [x] 合并单元格
    - [x] 拆分单元格
    - [x] 查找单元格
    - [x] 替换单元格
- 筛选
    - [x] 获取筛选
    - [x] 创建筛选
    - [x] 更新筛选
    - [x] 删除筛选

#### 多维表格

- 多维表格
    - [x] 获取多维表格元数据
- 字段
    - [x] 列出字段
- 记录
    - [x] 新增记录
    - [x] 查询记录
    - [x] 新增多条记录
    - [x] 更新记录

#### 通讯录

- 用户
    - [x] 搜索用户

#### 组织架构 (Directory v1) 🎉 v0.12.0 新增

- 员工管理
    - [x] 创建员工
    - [x] 更新员工信息 
    - [x] 获取员工列表
    - [x] 批量获取员工信息
    - [x] 搜索员工
    - [x] 设置员工为待离职
    - [x] 恢复员工为在职状态
    - [x] 恢复离职员工
    - [x] 离职员工
- 部门管理  
    - [x] 创建部门
    - [x] 更新部门信息
    - [x] 获取部门列表
    - [x] 批量获取部门信息
    - [x] 搜索部门
    - [x] 删除部门

### 飞书卡片

#### 卡片组件

- [x] 卡片回传交互
- 容器
    - [x] 分栏
    - [x] 表单容器
    - [x] 交互容器
    - [x] 折叠面板
- 展示
    - [x] 标题
    - [x] 普通文本
    - [x] 富文本(Markdown)
    - [x] 图片
    - [x] 多图混排
    - [x] 分割线
    - [x] 人员
    - [x] 人员列表
    - [x] 图表
    - [x] 表格
    - [x] 备注
- 交互
    - [x] 输入框
    - [x] 按钮
    - [x] 折叠按钮组
    - [x] 下拉选择-单选
    - [x] 下拉选择-多选
    - [x] 人员选择-单选
    - [x] 人员选择-多选
    - [x] 日期选择器
    - [x] 时间选择器
    - [x] 日期时间选择器
    - [x] 多图选择
    - [x] 勾选器

### 消息

- [x] 发送消息
- [x] 获取会话历史消息
- 消息内容结构
    - 发送消息内容
        - [x] 文本
        - [x] 富文本
        - [x] 图片
        - [x] 卡片
- 事件
    - [x] 接收消息

### 群组

- [x] 获取用户或机器人所在的群列表

### 招聘管理 🎉 v0.11.0 新增

#### 招聘相关配置

- [x] **地址管理**: 地点列表查询、地址信息获取
- [x] **权限管理**: 角色管理、用户权限分配
- [x] **职位管理**: 职位全生命周期管理(创建、发布、更新、关闭)
- [x] **招聘需求**: 招聘需求创建、模板管理
- [x] **招聘流程**: 招聘流程配置、阶段管理
- [x] **项目管理**: 招聘项目组织、成员管理
- [x] **面试设置**: 面试配置、评价表管理
- [x] **Offer设置**: Offer配置、审批流程设置

#### 获取候选人

- [x] **内推管理**: 内推信息、奖励管理
- [x] **官网管理**: 招聘官网、职位发布、投递管理
- [x] **猎头管理**: 猎头供应商、保护期、推荐管理
- [x] **外部系统**: 第三方HR系统集成

#### 候选人管理

- [x] **人才库**: 人才池组织、人才分组管理
- [x] **人才管理**: 人才档案、标签、批量导入
- [x] **投递管理**: 投递创建、流程推进、状态管理
- [x] **面试管理**: 面试安排、评估、结果记录
- [x] **Offer管理**: Offer发放、审批、接受流程

#### 生态对接

- [x] **背调管理**: 背调订单、报告管理
- [x] **笔试管理**: 在线笔试、试卷、成绩管理

#### 内推账户

- [x] **账户管理**: 内推账户创建、余额查询、收入记录
- [x] **提现管理**: 提现申请、审批流程、账户启停
- [x] **统计分析**: 内推统计数据、奖励计算

#### 附件管理

- [x] **文件管理**: 简历、证书等附件上传下载
- [x] **批量操作**: 文件预览、批量操作

### 考勤管理 🎉 v0.5.0 新增

#### 考勤班次

- [x] 创建班次
- [x] 删除班次
- [x] 按 ID 查询班次
- [x] 按名称查询班次
- [x] 查询所有班次

#### 考勤排版

- [x] 创建或修改排班表
- [x] 查询排班表
- [x] 创建或修改临时排班

#### 考勤管理

- [x] 查询考勤组下所有成员
- [x] 创建或修改考勤组
- [x] 删除考勤组
- [x] 按 ID 查询考勤组
- [x] 按名称查询考勤组
- [x] 查询所有考勤组

#### 考勤用户管理

- [x] 修改用户人脸识别信息
- [x] 批量查询用户人脸识别信息
- [x] 上传用户人脸识别照片
- [x] 下载用户人脸识别照片

#### 考勤统计

- [x] 更新统计设置
- [x] 查询统计表头
- [x] 查询统计设置
- [x] 查询统计数据

#### 假勤审批

- [x] 获取审批数据
- [x] 写入审批结果
- [x] 通知审批状态更新

#### 考勤补卡

- [x] 通知补卡审批发起
- [x] 获取可补卡时间
- [x] 获取补卡记录

#### 归档报表

- [x] 查询归档报表表头
- [x] 写入归档报表结果
- [x] 删除归档报表行数据
- [x] 查询所有归档规则

#### 打卡信息管理

- [x] 导入打卡流水
- [x] 查询打卡流水
- [x] 批量查询打卡流水
- [x] 删除打卡流水
- [x] 查询打卡结果

#### 休假管理

- [x] 通过过期时间获取发放记录
- [x] 修改发放记录

#### 考勤事件

- [x] 打卡流水事件处理
- [x] 用户任务状态变更事件处理

### AI 能力

#### 智能文档处理

- [x] 识别文件中的简历信息
- [x] 识别文件中的身份证
- [x] 识别文件中的驾驶证
- [x] 识别文件中的银行卡
- [x] 识别文件中的营业执照
- [x] 识别文件中的增值税发票
- [x] 提取文件中的合同字段
- [x] 识别文件中的名片
- [x] 识别文件中的机动车发票
- [x] 识别文件中的健康证
- [x] 识别文件中的港澳居民来往内地通行证
- [x] 识别文件中的台湾居民来往大陆通行证
- [x] 识别文件中的中国护照
- [x] 识别文件中的行驶证
- [x] 识别文件中的火车票
- [x] 识别文件中的出租车发票
- [x] 识别文件中的食品生产许可证
- [x] 识别文件中的食品经营许可证

#### 光学字符识别

- [x] 识别图片中的文字

#### 语音识别

- [x] 识别语音文件
- [x] 识别流式语音

#### 机器翻译

- [x] 识别文本语种
- [x] 翻译文本

## 📊 功能完成度统计

| 模块              | API数量   | 完成状态       | 说明                      |
|-----------------|---------|------------|-------------------------|
| **🔐 认证与授权**    | 1       | ✅ 100%     | 应用身份验证                  |
| **🏢 企业信息**     | 2       | ✅ 100%     | 企业基础信息与席位管理             |
| **🛡️ 认证信息**     | 1       | ✅ 100%     | 应用认证状态与权限信息             |
| **🤖 自定义机器人**   | 2       | ✅ 100%     | 消息发送与签名验证               |
| **🔗 长连接机器人**   | 1       | ✅ 100%     | 事件推送接收                  |
| **☁️ 云文档-云空间**  | 7       | ✅ 100%     | 文件夹管理、上传下载              |
| **🛡️ 云文档-权限**  | 2       | ✅ 100%     | 权限设置管理                  |
| **📊 云文档-电子表格** | 33      | ✅ 100%     | 完整表格操作功能                |
| **📋 云文档-多维表格** | 6       | ✅ 100%     | 数据表操作                   |
| **👥 通讯录**      | 1       | ✅ 100%     | 用户搜索                    |
| **🏢 组织架构**     | 15      | ✅ 100%     | **v0.12.0 新增** - 完整员工与部门管理 |
| **🎨 飞书卡片**     | 25      | ✅ 100%     | 完整卡片组件系统                |
| **💬 消息**       | 4       | ✅ 100%     | 消息发送与接收                 |
| **👥 群组**       | 1       | ✅ 100%     | 群组管理                    |
| **🔍 搜索**       | 14      | ✅ 100%     | 套件搜索与搜索连接器管理            |
| **🏢 考勤管理**     | 43      | ✅ 100%     | 完整考勤解决方案                |
| **⚙️ 个人设置**     | 7       | ✅ 100%     | 系统状态管理                  |
| **🤖 AI能力**      | 22      | ✅ 100%     | **v0.8.0 新增** - 智能文档处理、OCR、语音识别、机器翻译 |
| **🎯 招聘管理**     | **100+**| ✅ 100%     | **v0.11.0 新增** - 完整招聘管理系统 |
| **🛡️ 错误处理系统**  | 5       | ✅ 100%     | **v0.6.0 新增** - 企业级错误管理 |
| **📈 总计**       | **291+**| **✅ 100%** | **覆盖企业应用核心功能**          |

### 🎯 v0.11.0 招聘管理系统亮点

- **6大核心服务模块** 招聘配置、候选人获取、候选人管理、生态对接、内推账户、附件管理
- **100+ API接口** 覆盖招聘全流程，从职位发布到候选人入职
- **200+ 数据结构** 类型安全的数据模型定义，充分利用Rust类型系统
- **企业级特性** 模块化设计、异步支持、错误处理、国际化支持、分页查询
- **完整文档示例** 详细的API文档、完整的功能演示、技术实现报告

### 🎯 v0.6.0 企业级错误处理系统亮点

- **5大核心模块** 错误监控、重试中间件、日志系统、智能分析、错误码支持
- **30+ 业务错误码** 覆盖飞书生态全域，语义化分类管理
- **24个单元测试** 100%通过率，确保系统稳定性
- **智能化体验** 从技术错误到用户友好，AI级别的错误分析和建议
- **企业级特性** 实时监控、自动重试、结构化日志、告警系统

### 🎯 v0.5.0 考勤模块特性

- **41个API** 覆盖班次、排班、统计、审批等全业务流程
- **2个事件处理器** 支持实时考勤数据监听
- **43个示例** 每个功能都有完整的使用演示
- **企业级特性** 支持复杂的考勤规则和业务场景

## 📚 文档和资源

### 招聘管理系统文档

- **[招聘系统实现报告](reports/hire_v1_implementation_report.md)** - 详细的技术架构和功能说明
- **[hire_v1_example.rs](examples/api/hire_v1_example.rs)** - 完整的招聘系统功能演示

### 错误处理系统文档

- **[错误处理最佳实践](docs/ERROR_HANDLING_BEST_PRACTICES.md)** (62页) - 完整的开发指导和最佳实践
- **[错误处理功能介绍](docs/ERROR_HANDLING_FEATURES.md)** - 快速上手指南和功能概览
- **[项目完成报告](reports/project_completion_summary.md)** - 详细的技术架构和成果总结

### 示例程序

- **[hire_v1_example.rs](examples/api/hire_v1_example.rs)** - 招聘管理系统完整演示
- **[comprehensive_error_codes_demo.rs](examples/api/comprehensive_error_codes_demo.rs)** - 扩展错误码系统演示
- **[enhanced_error_handling.rs](examples/api/enhanced_error_handling.rs)** - 增强错误处理演示
- **[permission_owned_demo.rs](examples/api/permission_owned_demo.rs)** - owned参数模式演示

### API文档

- **[API参考文档](https://docs.rs/open-lark)** - 完整的API文档
- **[示例代码集合](examples/)** - 30+个完整的演示程序

## 🚀 特性优势

### 企业级文档体系

- **完整中文文档** - 43个服务模块100%文档覆盖，专为中国开发者优化
- **质量保证** - 72个文档测试确保示例代码正确性，零警告构建
- **开发者友好** - 丰富的使用示例、最佳实践指导、层次化文档结构
- **企业级标准** - 统一的文档格式和规范，便于团队协作和维护

### 企业级招聘管理

- **全流程覆盖** - 从职位发布到候选人入职的完整业务流程
- **多渠道集成** - 内推、官网、猎头、外部系统等多种人才来源
- **智能化管理** - 人才库管理、标签系统、搜索筛选、评估体系
- **模块化设计** - 清晰的功能分层和服务组织，易于扩展

### 企业级错误处理

- **零配置使用** - 开箱即用的合理默认配置
- **类型安全** - Rust强类型系统防止错误，统一的`SDKResult<T>`类型
- **并发安全** - Arc/Mutex确保多线程环境安全
- **模块化设计** - 每个模块可独立使用和扩展

### 性能优化

- **异步处理** - 全面使用async/await，零阻塞操作
- **内存效率** - 消除2000+个不必要的clone调用
- **智能缓存** - 自动token缓存管理
- **零开销抽象** - 编译时优化，运行时高效

### 开发体验

- **用户友好** - 智能错误分析，自动生成修复建议
- **完整文档** - 技术架构文档，30+个示例程序
- **类型提示** - 完整的类型定义和IDE支持
- **测试覆盖** - 全面的测试覆盖，确保代码质量

## 📋 TODO

目前主要功能模块均已完成，后续计划：

- [ ] 更多AI能力集成
- [ ] 更多事件处理器支持
- [ ] 性能优化和缓存策略
- [ ] 更多示例和文档

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📄 许可证

MIT OR Apache-2.0
