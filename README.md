[![crates.io](https://img.shields.io/crates/v/open-lark)](https://crates.io/crates/open-lark)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/Seldom-SE/seldom_pixel#license)
[![CI](https://github.com/foxzool/open-lark/workflows/CI/badge.svg)](https://github.com/foxzool/open-lark/actions)
[![Documentation](https://docs.rs/open-lark/badge.svg)](https://docs.rs/open-lark)
![Discord Shield](https://discord.com/api/guilds/1319490473060073532/widget.png?style=shield)

# 飞书开放平台非官方SDK, 个人开发, 请谨慎使用

支持自定义机器人、长连接机器人、云文档、飞书卡片、消息、群组、招聘管理等API调用。

## 🎉 v0.11.0 重大更新 - 飞书招聘管理系统全面上线 🚀

- **🎯 完整招聘管理系统**: 6个核心服务模块，17个功能子服务，100+个API接口
- **🏗️ 企业级招聘架构**: 从职位发布到候选人入职的完整业务流程
- **📊 多渠道人才获取**: 内推、官网、猎头、外部系统等多种人才来源
- **🤖 智能化招聘**: 人才库管理、标签系统、搜索筛选、评估体系
- **🔄 协同化面试**: 面试安排、多轮面试、评估体系、结果记录
- **📋 规范化Offer**: Offer模板、审批流程、电子签约、状态管理
- **💰 内推管理**: 内推奖励、账户管理、提现审批、统计分析
- **🔗 生态对接**: 背调、笔试平台对接，扩展招聘能力

## 🎉 v0.8.0 重大更新 - AI能力全面支持 🤖

- **🧠 智能文档处理**: 支持17种证件和文档智能识别，包括简历、身份证、驾驶证、营业执照等
- **👀 光学字符识别**: 高精度图片文字识别，支持多种图片格式和复杂场景
- **🎙️ 语音识别**: 支持文件语音识别和流式语音识别，多语言支持
- **🌐 机器翻译**: 智能语种检测和多语言文本翻译服务
- **📊 新增22个API**: 覆盖AI能力全生态，从176个API扩展企业智能化应用
- **🔧 完整示例**: 提供ai_comprehensive示例，演示所有AI功能的集成使用

## 🎉 v0.6.0 重大更新 - 企业级错误处理系统重构 ⭐

- **🛡️ 企业级错误处理**: 全新5大核心模块，提供智能错误分析、自动重试、实时监控
- **🧠 智能错误分析**: 技术错误转换为用户友好提示，自动生成修复建议和操作步骤
- **🔄 自动重试机制**: 基于错误类型的智能重试策略，指数退避算法避免系统过载
- **📊 实时错误监控**: 错误率计算、分类统计、智能告警，支持文件导出和报告生成
- **📝 结构化日志**: 多格式输出(JSON/文本/结构化)，彩色控制台，灵活输出目标
- **♻️ 大规模重构完成**: 68个文件迁移到owned参数模式，消除2000+个clone调用
- **📚 完整文档体系**: 62页最佳实践指南，5个演示程序，完整的技术架构文档

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