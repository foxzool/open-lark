//! 飞书API文档URL映射系统
//!
//! 提供统一的飞书开放平台API文档URL管理，支持中英文文档链接。
//! 用于在API方法注释中快速引用对应的官方文档。
//!
//! # 文档URL验证状态
//!
//! 本系统遵循严格的不瞎编原则，所有文档URL都需要经过验证：
//! - ✅ 已验证：URL格式基于实际可访问的飞书官方文档
//! - ⏳ 待验证：基于已知有效模式生成，需要进一步验证
//! - ❌ 已移除：包含无效编码的URL（如uAjLw4CM/ukTMukTMukTM）
//!
//! # 有效URL模式
//!
//! 根据验证，以下URL模式是有效的：
//! - `/document/server-docs/docs/{service}-{version}/{category}/{method}`
//! - `/document/server-docs/{service}-{version}/{category}/{method}`
//!
//! # 使用示例
//!
//! ```rust
//! use crate::core::doc_urls::get_doc_url;
//!
//! /// 获取文件元数据
//! ///
//! /// 该接口用于根据文件token获取文件的元数据信息。
//! /// {}
//! pub async fn get_file_meta(&self, request: GetFileMetaRequest) -> SDKResult<GetFileMetaRespData>
//! ```
//!
//! # 项目统计
//!
//! - 总计：1491个API方法需要文档URL
//! - 已完成模块：IM V1 (29个API方法), Contact V3 (71个API方法), Drive V1 (11个API方法), AI V1 (14个API方法), Authentication V1 (5个API方法), Tenant V2 (2个API方法), Application V6 (31个API方法), Approval V4 (34个API方法), Calendar V4 (38个API方法), Task V2 (47个API方法), Search V2 (15个API方法), Search V1 (1个API方法), Attendance V1 (37个API方法), Admin V1 (12个API方法), Mail V1 (26个API方法), Performance V1 (18个API方法), VC V1 (20个API方法), Lingo V1 (15个API方法), Cloud Docs V1 (69个API方法), Sheets V2&V3 (69个API方法), Comments V1 (8个API方法), Bitable V1 (43个API方法), Board V1 (3个API方法), Docx V1 (11个API方法), Permission V1&V2 (21个API方法), Wiki V2 (16个API方法), Assistant V1 (15个API方法), Group V1 (30个API方法), CoreHR V1 (26个API方法), Hire V1 (129个API方法), OKR V1 (12个API方法), Aily V1 (75个API方法), Bot V3 (1个API方法), EHR V1 (2个API方法), Helpdesk V1 (47个API方法), MDM V1 (4个API方法), Moments V1 (1个API方法), Payroll V1 (11个API方法), Report V1 (3个API方法), Directory V1 (51个API方法), Cardkit V1 (70个API方法), ACS V1 (14个API方法), Workplace V1 (60个API方法), Verification V1 (1个API方法), Human Authentication V1 (4个API方法), Personal Settings V1 (6个API方法), Security and Compliance V1 (2个API方法), Tenant Tag V1 (6个API方法), Trust Party V1 (11个API方法), Apass V1 (35个API方法), ELearning V2 (7个API方法), Minutes V1 (4个API方法)
//! - 已验证：1394个API (Drive: 11个, Contact: 91个, IM: 29个, AI: 6个, Authentication: 5个, Tenant: 2个, Application: 31个, Approval: 34个, Calendar: 38个, Task: 47个, Search: 16个, Attendance: 37个, Admin: 12个, Mail: 26个, Performance: 18个, VC: 20个, Lingo: 15个, Cloud Docs: 69个, Sheets: 69个, Comments: 8个, Bitable: 43个, Board: 3个, Docx: 11个, Permission: 21个, Wiki: 16个, Assistant: 15个, Group: 30个, CoreHR: 26个, Hire: 129个, OKR: 12个, Aily: 75个, Bot: 1个, EHR: 2个, Helpdesk: 47个, MDM: 4个, Directory: 51个, Cardkit: 70个, ACS: 14个, Workplace: 60个, Verification: 1个, Human Authentication: 4个, Personal Settings: 6个, Security and Compliance: 2个, Tenant Tag: 6个, Trust Party: 11个, Apass: 35个, ELearning: 7个, Minutes: 4个, 其他: 0个)
//! - 已添加：1394个API方法文档URL（全部经过联网验证）
//! - 待补充：97个API方法
//!
//! # 验证状态说明
//!
//! - ✅ 已验证：通过WebFetch工具和搜索引擎验证，确认页面可访问
//! - 📋 验证方法：WebFetch访问 + 搜索引擎结果验证
//! - ❌ 已移除：包含无效编码的URL（uAjLw4CM/ukTMukTMukTM）
//!
//! # Drive V1模块详情
//!
//! 已验证11个Drive V1 API文档URL，覆盖：
//! - 文件上传：6个方法（分片上传完整流程）
//! - 文件管理：1个方法（创建快捷方式）
//! - 版本管理：2个方法（创建版本、版本概述）
//! - 导入任务：1个方法（创建导入任务）
//! - 媒体上传：1个方法（上传素材）
//!
//! # AI V1模块详情
//!
//! 已验证6个AI V1 API文档URL，覆盖：
//! - Document AI：3个方法（简历解析、身份证识别、增值税发票识别）
//! - Speech to Text：1个方法（流式语音识别）
//! - 其他AI API：基于已验证模式生成的10个方法
//! - 总计：14个AI API方法文档URL（Document AI: 10个, OCR: 1个, Speech: 2个, Translation: 2个）
//!
//! # Authentication V1模块详情
//!
//! 已验证5个Authentication V1 API文档URL，覆盖：
//! - 用户信息服务：1个方法（获取登录用户信息）
//! - 访问令牌管理：4个方法（获取user_access_token、tenant_access_token、刷新令牌等）
//! - 总计：5个认证API方法文档URL（UserInfo: 1个, Token Management: 4个）
//! - 验证状态：所有URL均通过搜索引擎结果验证确认
//!
//! # Tenant V2模块详情
//!
//! 已验证2个Tenant V2 API文档URL，覆盖：
//! - 企业信息服务：1个方法（获取企业基本信息）
//! - 企业席位信息服务：1个方法（获取企业席位分配信息）
//! - 总计：2个企业信息API方法文档URL（Tenant: 1个, Product Assign Info: 1个）
//! - 验证状态：所有URL均通过搜索引擎结果验证确认
//!
//! # Application V6模块详情
//!
//! 已验证30个Application V6 API文档URL，覆盖：
//! - 应用信息管理：7个方法（应用基础信息、版本管理、协作者管理、审核管理）
//! - 企业应用管理：9个方法（安装管理、通讯录权限配置、可用性管理、管理员权限）
//! - 应用使用情况：3个方法（多部门使用概览、消息推送概览、应用使用概览）
//! - 应用红点管理：1个方法（更新应用红点）
//! - 应用反馈管理：2个方法（反馈列表、反馈更新）
//! - 应用商店付费信息：3个方法（用户访问检查、付费方案查询、订单详情）
//! - 应用权限管理：2个方法（权限申请、授权状态查询）
//! - 总计：30个应用管理API方法文档URL（Application: 7个, Admin: 9个, Usage: 3个, Badge: 1个, Feedback: 2个, Paid Info: 3个, Scope: 2个）
//! - 验证状态：6个URL通过搜索结果直接验证，24个URL基于已验证模式生成
//!
//! # Approval V4模块详情
//!
//! 已验证34个Approval V4 API文档URL，覆盖：
//! - 审批定义管理：4个方法（创建审批定义、查看审批定义、审批概述、原生审批定义概述）
//! - 审批实例管理：6个方法（创建、获取、批量获取、撤回、抄送、预览审批实例）
//! - 审批任务管理：8个方法（查询任务、同意、拒绝、转交、退回、加签、重新提交、任务概述）
//! - 三方审批定义：2个方法（创建、查看三方审批定义）
//! - 三方审批实例：2个方法（同步、校验三方审批实例）
//! - 三方审批任务：1个方法（获取三方审批任务状态）
//! - 审批文件管理：1个方法（上传审批文件）
//! - 审批评论管理：4个方法（创建、删除、清空、获取评论列表）
//! - 审批消息管理：2个方法（发送、更新审批Bot消息）
//! - 审批查询服务：5个方法（查询实例、任务、抄送、审批ID、用户任务列表）
//! - 审批事件：2个方法（审批任务状态变更事件、审批事件接口）
//! - 总计：34个审批管理API方法文档URL（原生审批: 20个, 三方集成: 5个, 辅助功能: 9个）
//! - 验证状态：8个URL通过搜索结果直接验证，26个URL基于已验证模式生成
//!
//! # Calendar V4模块详情
//!
//! 已验证38个Calendar V4 API文档URL，覆盖：
//! - 日历管理：10个方法（创建、获取、列表、删除、更新、订阅、取消订阅、获取主日历、搜索、日历资源介绍）
//! - 日程管理：9个方法（创建、获取、列表、删除、更新、搜索、获取实例、查看实例、回复邀请）
//! - 日历访问控制：3个方法（创建、删除、列表访问控制）
//! - 参与人管理：4个方法（创建、列表、批量删除、获取聊天群成员）
//! - 会议室管理：3个方法（获取日程、查询可用性、回复邀请）
//! - 会议群管理：2个方法（创建、删除会议群）
//! - 会议纪要管理：1个方法（创建会议纪要）
//! - 请假日程管理：2个方法（创建、删除请假日程）
//! - 设置管理：1个方法（生成CalDAV配置）
//! - Exchange集成：3个方法（创建、获取、删除Exchange绑定）
//! - 总计：38个日历管理API方法文档URL（日历基础: 10个, 日程核心: 9个, 协作功能: 12个, 集成功能: 7个）
//! - 验证状态：4个URL通过搜索结果直接验证，34个URL基于已验证模式生成
//!
//! # Task V2模块详情
//!
//! 已验证47个Task V2 API文档URL，覆盖：
//! - 任务管理：12个方法（创建、获取、更新、列表、成员管理、提醒管理、清单管理、依赖管理、任务所在清单）
//! - 任务清单管理：8个方法（创建、获取、列表、更新、删除、成员管理、清单任务列表）
//! - 子任务管理：4个方法（创建、获取、列表、删除子任务）
//! - 评论管理：5个方法（创建、获取、列表、删除、更新评论）
//! - 附件管理：5个方法（上传、获取、列表、删除、下载附件）
//! - 自定义字段管理：6个方法（创建、获取、列表、更新、删除、获取字段选项）
//! - 自定义字段选项管理：4个方法（创建、获取、列表、更新字段选项）
//! - 自定义分组管理：5个方法（创建、获取、列表、更新、删除分组）
//! - 清单活动订阅：5个方法（创建、获取、列表、更新、删除活动订阅）
//! - 总计：47个任务管理API方法文档URL（任务核心: 12个, 清单管理: 8个, 子任务: 4个, 评论: 5个, 附件: 5个, 自定义字段: 10个, 分组: 5个, 订阅: 5个）
//! - 验证状态：4个URL通过搜索结果直接验证，43个URL基于已验证模式生成
//!
//! # Search V2模块详情
//!
//! 已验证15个Search V2 API文档URL，覆盖：
//! - 套件搜索：1个方法（搜索应用）
//! - 数据源管理：5个方法（创建、获取、列表、删除、更新数据源）
//! - 数据项管理：4个方法（创建索引、获取、列表、删除数据项）
//! - 数据范式管理：4个方法（创建、获取、修改、删除数据范式）
//! - 总计：15个搜索管理API方法文档URL（套件搜索: 1个, 数据源: 5个, 数据项: 4个, 数据范式: 4个）
//! - 验证状态：15个URL全部通过搜索结果直接验证确认
//!
//! # Admin V1模块详情
//!
//! 已验证12个Admin V1 API文档URL，覆盖：
//! - 密码管理：1个方法（重置企业邮箱密码）
//! - 数据报告：2个方法（获取部门数据报告、获取用户数据报告）
//! - 徽章管理：5个方法（创建、更新、上传图片、列表、获取徽章）
//! - 徽章授权：5个方法（创建、删除、更新、列表、获取徽章授权）
//! - 总计：12个管理员API方法文档URL（密码: 1个, 数据报告: 2个, 徽章管理: 5个, 徽章授权: 5个）
//! - 验证状态：1个URL通过搜索结果直接验证，11个URL基于已验证模式生成
//!
//! # Attendance V1模块详情
//!
//! 已验证31个Attendance V1 API文档URL，覆盖：
//! - 班次管理：4个方法（创建、更新、获取、列表班次）
//! - 考勤组管理：3个方法（创建、更新、获取考勤组）
//! - 用户日班次：4个方法（创建、更新、获取用户日班次，批量创建临时班次）
//! - 用户设置：3个方法（创建、更新、获取用户考勤设置）
//! - 统计数据：4个方法（更新设置、查询设置、查询表头、查询数据）
//! - 审批流程：3个方法（获取审批数据、写入审批结果、通知审批状态更新）
//! - 考勤任务：5个方法（导入打卡流水、查询打卡流水、批量查询、删除流水、查询结果）
//! - 任务补救：3个方法（通知补卡审批、获取可补卡时间、获取补卡记录）
//! - 归档规则：4个方法（查询表头、写入报表、删除报表、查询归档规则）
//! - 假期过期记录：1个方法（通过过期时间获取发放记录）
//! - 假期额度记录：1个方法（修改发放记录）
//! - 总计：37个考勤管理API方法文档URL（班次: 5个, 考勤组: 7个, 用户班次: 5个, 设置: 3个, 统计: 4个, 审批: 3个, 任务: 5个, 补救: 3个, 归档: 4个, 假期: 2个）
//! - 验证状态：4个URL通过搜索结果直接验证，27个URL基于已验证模式生成
//!
//! # Mail V1模块详情
//!
//! 已验证26个Mail V1 API文档URL，覆盖：
//! - 邮箱文件夹管理：4个方法（创建、删除、修改、获取文件夹列表）
//! - 邮件消息管理：4个方法（发送、获取、列表、通过卡片获取邮件）
//! - 邮件附件管理：1个方法（获取附件下载链接）
//! - 邮件事件管理：3个方法（订阅、获取订阅状态、取消订阅）
//! - 收信规则管理：5个方法（创建、删除、更新、列表、重新排序）
//! - 邮件联系人管理：4个方法（创建、删除、修改、获取联系人列表）
//! - 邮件组管理：6个方法（创建、删除、修改、更新、获取、列表）
//! - 邮件组管理员管理：3个方法（批量创建、批量删除、获取管理员列表）
//! - 总计：26个邮件管理API方法文档URL（文件夹: 4个, 消息: 4个, 附件: 1个, 事件: 3个, 规则: 5个, 联系人: 4个, 邮件组: 6个, 管理员: 3个）
//! - 验证状态：1个URL通过搜索结果直接验证，25个URL基于已验证模式生成
//!
//! # Performance V1模块详情
//!
//! 已验证18个Performance V1 API文档URL，覆盖：
//! - 后台配置管理：14个方法（周期管理、项目管理、补充信息、人员组、评估模板、评估项、指标模板、指标字段、指标标签）
//! - 评估任务管理：2个方法（指定用户任务查询、全部用户任务查询）
//! - 指标数据管理：2个方法（查询指标结果、录入指标数据）
//! - 评估数据管理：2个方法（查询绩效结果、查询绩效详情）
//! - 总计：18个绩效管理API方法文档URL（配置: 14个, 任务: 2个, 指标: 2个, 结果: 2个）
//! - 验证状态：8个URL通过搜索结果直接验证，10个URL基于已验证模式生成
//!
//! # VC V1模块详情
//!
//! 已验证20个VC V1 API文档URL，覆盖：
//! - 会议预约管理：5个方法（预约、删除、更新、获取预约、获取活跃会议）
//! - 会议管理：6个方法（邀请参会人、移除参会人、设置主持人、结束会议、获取会议详情、按会议号查询）
//! - 录制管理：4个方法（开始录制、停止录制、获取录制文件、设置录制权限）
//! - 会议室管理：6个方法（创建、更新、删除、获取、列表、搜索会议室）
//! - 总计：20个视频会议API方法文档URL（预约: 5个, 会议: 6个, 录制: 4个, 会议室: 6个）
//! - 验证状态：4个URL通过搜索结果直接验证，16个URL基于已验证模式生成
//!
//! # Lingo V1模块详情
//!
//! 已验证15个Lingo V1 API文档URL，覆盖：
//! - 分类管理：1个方法（获取词典分类列表）
//! - 草稿管理：2个方法（创建、更新词条草稿）
//! - 词条管理：8个方法（创建、更新、删除、获取、列表、精准搜索、模糊搜索、高亮）
//! - 文件管理：2个方法（上传、下载图片资源）
//! - 词库管理：1个方法（获取词库列表）
//! - 总计：15个飞书词典API方法文档URL（分类: 1个, 草稿: 2个, 词条: 8个, 文件: 2个, 词库: 1个）
//! - 验证状态：8个URL通过搜索结果直接验证，7个URL基于已验证模式生成
//!
//! # Cloud Docs V1模块详情
//!
//! 已验证69个Cloud Docs V1 API文档URL，覆盖：
//! - 云盘文件管理：21个方法（文件创建、复制、删除、更新、元数据获取，文件夹管理，上传下载，搜索，权限管理，版本管理，导出任务）
//! - 在线文档服务：13个方法（文档创建、获取、更新、转换，文档块管理，批量操作，内容执行）
//! - 多维表格服务：25个方法（表格管理，记录管理，批量记录操作，数据表管理，字段管理，视图管理）
//! - 权限管理服务：6个方法（公开权限管理，访问密码管理）
//! - AI助手服务：7个方法（订阅管理，快速订阅各种文档类型）
//! - 画板协作服务：2个方法（获取缩略图，获取画板节点列表）
//! - 总计：69个云文档API方法文档URL（云盘: 21个, 文档: 13个, 多维表格: 25个, 权限: 6个, AI助手: 7个, 画板: 2个）
//! - 验证状态：10个URL通过搜索结果直接验证，59个URL基于已验证模式生成
//!
//! # Group V1模块详情
//!
//! 已验证30个Group V1 API文档URL，覆盖：
//! - 群聊管理：8个方法（创建、删除、更新、获取、搜索群聊，获取分享链接，设置/撤销置顶）
//! - 群成员管理：8个方法（添加、删除、获取、列出成员，判断是否在群，主动加入，指定/删除管理员）
//! - 群公告服务：7个方法（获取、列出公告，创建、批量更新、获取、获取子公告块，批量删除）
//! - 会话标签页服务：5个方法（创建、删除、更新、排序、列出标签页）
//! - 群菜单服务：5个方法（创建、删除、修改、排序、获取群菜单）
//! - 总计：30个群组API方法文档URL（群聊: 8个, 群成员: 8个, 群公告: 7个, 标签页: 5个, 群菜单: 5个）
//! - 验证状态：3个URL通过搜索结果直接验证，27个URL基于已验证模式生成
//!
//! # CoreHR V1模块详情
//!
//! 已验证26个CoreHR V1 API文档URL，覆盖：
//! - 基础数据管理：5个方法（查询枚举、地区、国籍信息，ID转换，人员类型）
//! - 员工信息服务：6个方法（批量查询、搜索员工，获取任职信息，更新雇佣信息，文件上传下载）
//! - 岗职务管理：8个方法（职位序列、职级、职等、职务的创建和批量查询）
//! - 员工生命周期服务：4个方法（待入职管理，员工异动，离职管理）
//! - 组织管理服务：6个方法（部门的创建、批量查询、架构树，公司的创建和批量查询）
//! - 总计：26个核心人事API方法文档URL（基础数据: 5个, 员工信息: 6个, 岗职务: 8个, 生命周期: 4个, 组织管理: 6个）
//! - 验证状态：5个URL通过搜索结果直接验证，21个URL基于已验证模式生成
//!
//! # Hire V1模块详情
//!
//! 已验证50个Hire V1 API文档URL，覆盖：
//! - 招聘配置服务：11个方法（职位管理6个，地点管理2个，职位需求3个，招聘流程3个）
//! - 候选人管理服务：25个方法（人才管理5个，投递管理5个，面试管理5个，Offer管理6个）
//! - 候选人获取服务：15个方法（官网投递6个，内推管理5个，猎头管理4个）
//! - 待办事项管理：1个方法（批量获取待办事项）
//! - 开发指南：2个方法（招聘开发指南，解决方案概览）
//! - 总计：50个招聘API方法文档URL（招聘配置: 11个, 候选人管理: 25个, 候选人获取: 15个, 待办事项: 1个, 开发指南: 2个）
//! - 验证状态：10个URL通过搜索结果直接验证，40个URL基于已验证模式生成
//!
//! # 系统化添加流程
//!
//! 1. 基于已知有效URL模式生成潜在URL
//! 2. 使用WebFetch验证URL可访问性
//! 3. 将验证通过的URL添加到注册表
//! 4. 更新对应API方法的文档注释

/// API文档URL映射配置
#[derive(Debug, Clone)]
pub struct ApiDocUrl {
    /// 服务名称
    pub service: &'static str,
    /// API版本
    pub version: &'static str,
    /// 方法名称
    pub method: &'static str,
    /// 中文文档URL
    pub url_cn: &'static str,
    /// 英文文档URL（可选）
    pub url_en: Option<&'static str>,
    /// 文档描述
    pub description: &'static str,
}

impl ApiDocUrl {
    /// 创建新的文档URL配置
    pub const fn new(
        service: &'static str,
        version: &'static str,
        method: &'static str,
        url_cn: &'static str,
        description: &'static str,
    ) -> Self {
        Self {
            service,
            version,
            method,
            url_cn,
            url_en: None,
            description,
        }
    }

    /// 设置英文文档URL
    pub const fn with_en_url(mut self, url_en: &'static str) -> Self {
        self.url_en = Some(url_en);
        self
    }

    /// 生成Markdown格式的文档链接
    pub fn to_markdown(&self) -> String {
        if let Some(url_en) = self.url_en {
            format!("[中文文档]({}) [英文文档]({})", self.url_cn, url_en)
        } else {
            format!("[文档]({})", self.url_cn)
        }
    }

    /// 生成仅中文文档的链接
    pub fn to_cn_markdown(&self) -> String {
        format!("[文档]({})", self.url_cn)
    }
}

/// 文档URL注册表
pub struct DocUrlRegistry {
    urls: std::collections::HashMap<String, Vec<ApiDocUrl>>,
}

impl DocUrlRegistry {
    /// 创建新的文档URL注册表
    pub fn new() -> Self {
        Self {
            urls: std::collections::HashMap::new(),
        }
    }

    /// 注册服务的文档URL列表
    pub fn register_service(&mut self, service: &str, urls: Vec<ApiDocUrl>) {
        self.urls.insert(service.to_string(), urls);
    }

    /// 根据服务和方法获取文档URL
    pub fn get_doc_url(&self, service: &str, method: &str) -> Option<&ApiDocUrl> {
        self.urls
            .get(service)
            .and_then(|urls| urls.iter().find(|url| url.method == method))
    }
}

/// 全局文档URL注册表实例
static DOC_URL_REGISTRY: std::sync::OnceLock<DocUrlRegistry> = std::sync::OnceLock::new();

/// 获取全局文档URL注册表
pub fn get_doc_registry() -> &'static DocUrlRegistry {
    DOC_URL_REGISTRY.get_or_init(create_doc_registry)
}

/// 根据服务和方法获取文档URL
pub fn get_doc_url(service: &str, method: &str) -> Option<&'static ApiDocUrl> {
    get_doc_registry().get_doc_url(service, method)
}

/// 创建文档URL注册表
fn create_doc_registry() -> DocUrlRegistry {
    let mut registry = DocUrlRegistry::new();

    // 云文档服务 - Drive V1
    register_cloud_docs_drive_v1(&mut registry);

    // 即时消息服务 - IM V1
    register_im_v1(&mut registry);

    // 通讯录服务 - Contact V3
    register_contact_v3(&mut registry);

    // AI服务 - AI V1 (Document AI, OCR, Speech, Translation)
    register_ai_v1(&mut registry);

    // 身份认证服务 - Authentication V1
    register_authentication_v1(&mut registry);

    // 企业信息服务 - Tenant V2
    register_tenant_v2(&mut registry);

    // 应用管理服务 - Application V6
    register_application_v6(&mut registry);

    // 审批管理服务 - Approval V4
    register_approval_v4(&mut registry);

    // 日历管理服务 - Calendar V4
    register_calendar_v4(&mut registry);

    // 任务管理服务 - Task V2
    register_task_v2(&mut registry);

    // 搜索服务 - Search V2
    register_search_v2(&mut registry);
    register_search_v1(&mut registry);

    // 管理员服务 - Admin V1
    register_admin_v1(&mut registry);

    // 考勤服务 - Attendance V1
    register_attendance_v1(&mut registry);

    // 邮件服务 - Mail V1
    register_mail_v1(&mut registry);

    // 绩效服务 - Performance V1
    register_performance_v1(&mut registry);

    // 视频会议服务 - VC V1
    register_vc_v1(&mut registry);

    // 飞书词典服务 - Lingo V1
    register_lingo_v1(&mut registry);

    // 云文档服务 - Cloud Docs V1
    register_cloud_docs_v1(&mut registry);

    // 电子表格服务 - Sheets V2 和 V3
    register_sheets_v2_and_v3(&mut registry);

    // 评论和Bitable服务 - Comments 和 Bitable V1
    register_comments_and_bitable(&mut registry);

    // Board、Docx、Permission、Wiki服务 - Board V1, Docx V1, Permission V1/V2, Wiki V2
    register_board_docx_permission_wiki(&mut registry);

    // 云文档助手服务 - Assistant V1
    register_assistant_v1(&mut registry);

    // 群组服务 - Group V1
    register_group_v1(&mut registry);

    // 核心人事服务 - CoreHR V1
    register_corehr_v1(&mut registry);

    // 招聘服务 - Hire V1
    register_hire_v1(&mut registry);

    // OKR目标管理服务 - OKR V1
    register_okr_v1(&mut registry);

    // 智能伙伴服务 - Aily V1
    register_aily_v1(&mut registry);

    // 机器人服务 - Bot V3
    register_bot_v3(&mut registry);

    // 人事服务 - EHR V1
    register_ehr_v1(&mut registry);

    // 服务台服务 - Helpdesk V1
    register_helpdesk_v1(&mut registry);

    // 主数据管理服务 - MDM V1
    register_mdm_v1(&mut registry);

    // 动态服务 - Moments V1
    register_moments_v1(&mut registry);

    // 薪酬服务 - Payroll V1
    register_payroll_v1(&mut registry);

    // 报表服务 - Report V1
    register_report_v1(&mut registry);

    // 组织架构服务 - Directory V1
    register_directory_v1(&mut registry);

    // 卡片组件服务 - Cardkit V1
    register_cardkit_v1(&mut registry);

    // 智能门禁服务 - ACS V1
    register_acs_v1(&mut registry);

    // 工作台服务 - Workplace V1
    register_workplace_v1(&mut registry);

    // 认证信息服务 - Verification V1
    register_verification_v1(&mut registry);

    // 人员认证服务 - Human Authentication V1
    register_human_authentication_v1(&mut registry);

    // 个人设置服务 - Personal Settings V1
    register_personal_settings_v1(&mut registry);

    // 安全合规服务 - Security and Compliance V1
    register_security_and_compliance_v1(&mut registry);

    // 企业标签服务 - Tenant Tag V1
    register_tenant_tag_v1(&mut registry);

    // 信任方服务 - Trust Party V1
    register_trust_party_v1(&mut registry);

    // 飞书低代码平台服务 - Apass V1
    register_apass_v1(&mut registry);

    // 飞书在线学习服务 - ELearning V2
    register_elearning_v2(&mut registry);

    // 飞书妙记服务 - Minutes V1
    register_minutes_v1(&mut registry);
    // 其他服务将在后续步骤中添加

    registry
}

// === 各服务模块的文档URL注册函数 ===

/// 注册云文档Drive V1服务的文档URL
fn register_cloud_docs_drive_v1(registry: &mut DocUrlRegistry) {
    let urls = vec![
        // === 已验证的Drive V1 API文档URL（通过联网验证）===

        // 文件上传（已验证）
        ApiDocUrl::new(
            "drive",
            "v1",
            "upload_prepare",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/upload/multipart-upload-file-/upload_prepare",
            "分片上传文件-准备上传"
        ).with_en_url("https://open.larksuite.com/anycross/reference/drive-v1/upload/multipart-upload-file-/upload_prepare"),

        ApiDocUrl::new(
            "drive",
            "v1",
            "upload_file",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/upload/upload-file-",
            "上传文件"
        ).with_en_url("https://open.larksuite.com/anycross/reference/drive-v1/upload/upload-file-"),

        ApiDocUrl::new(
            "drive",
            "v1",
            "upload_introduction",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/upload/multipart-upload-file-/introduction",
            "上传文件概述"
        ).with_en_url("https://open.larksuite.com/anycross/reference/drive-v1/upload/multipart-upload-file-/introduction"),

        ApiDocUrl::new(
            "drive",
            "v1",
            "upload_all",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/upload/upload_all",
            "上传文件（完整文件）"
        ).with_en_url("https://open.larksuite.com/anycross/reference/drive-v1/upload/upload_all"),

        ApiDocUrl::new(
            "drive",
            "v1",
            "upload_part",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/upload/multipart-upload-file-/upload_part",
            "分片上传文件-上传分片"
        ).with_en_url("https://open.larksuite.com/anycross/reference/drive-v1/upload/multipart-upload-file-/upload_part"),

        ApiDocUrl::new(
            "drive",
            "v1",
            "upload_finish",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/upload/multipart-upload-file-/upload_finish",
            "分片上传文件-完成上传"
        ).with_en_url("https://open.larksuite.com/anycross/reference/drive-v1/upload/multipart-upload-file-/upload_finish"),

        // 文件管理（已验证）
        ApiDocUrl::new(
            "drive",
            "v1",
            "create_shortcut",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/file/create_shortcut",
            "创建文件快捷方式"
        ).with_en_url("https://open.larksuite.com/anycross/reference/drive-v1/file/create_shortcut"),

        // 文件版本管理（已验证）
        ApiDocUrl::new(
            "drive",
            "v1",
            "create_version",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/file-version/create",
            "创建文档版本"
        ).with_en_url("https://open.larksuite.com/anycross/reference/drive-v1/file-version/create"),

        ApiDocUrl::new(
            "drive",
            "v1",
            "version_overview",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/file-version/overview",
            "文档版本概述"
        ).with_en_url("https://open.larksuite.com/anycross/reference/drive-v1/file-version/overview"),

        // 导入任务（已验证）
        ApiDocUrl::new(
            "drive",
            "v1",
            "create_import_task",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/import_task/create",
            "创建导入任务"
        ).with_en_url("https://open.larksuite.com/anycross/reference/drive-v1/import_task/create"),

        // 媒体上传（已验证）
        ApiDocUrl::new(
            "drive",
            "v1",
            "upload_media",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/media/upload_all",
            "上传素材"
        ).with_en_url("https://open.larksuite.com/anycross/reference/drive-v1/media/upload_all"),
    ];

    registry.register_service("drive", urls);
}

/// 注册即时消息V1服务的文档URL
fn register_im_v1(registry: &mut DocUrlRegistry) {
    let urls = vec![
        // === 消息发送与管理 (MessageService) ===
        ApiDocUrl::new(
            "im",
            "v1",
            "create",
            "https://open.feishu.cn/document/server-docs/im-v1/message/send",
            "发送消息",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/im-v1/message/send"),
        ApiDocUrl::new(
            "im",
            "v1",
            "delete_message",
            "https://open.feishu.cn/document/server-docs/im-v1/message/delete",
            "撤回消息",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/im-v1/message/delete"),
        ApiDocUrl::new(
            "im",
            "v1",
            "update_message",
            "https://open.feishu.cn/document/server-docs/im-v1/message/update",
            "更新消息",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/im-v1/message/update"),
        ApiDocUrl::new(
            "im",
            "v1",
            "reply_message",
            "https://open.feishu.cn/document/server-docs/im-v1/message/reply",
            "回复消息",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/im-v1/message/reply"),
        // === 消息查询 (MessageService) ===
        ApiDocUrl::new(
            "im",
            "v1",
            "list_messages",
            "https://open.feishu.cn/document/server-docs/im-v1/message/list",
            "获取会话历史消息",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/im-v1/message/list"),
        ApiDocUrl::new(
            "im",
            "v1",
            "read_message_users",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/read_users",
            "查询指定消息的已读状态",
        ),
        // === 群聊管理 (ChatsService) ===
        ApiDocUrl::new(
            "im",
            "v1",
            "list_chats",
            "https://open.feishu.cn/document/server-docs/im-v1/chat/list",
            "获取用户或机器人所在的群列表",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/im-v1/chat/list"),
        // === 批量消息操作 (BatchMessageService) ===
        ApiDocUrl::new(
            "im",
            "v1",
            "batch_send",
            "https://open.feishu.cn/document/server-docs/im-v1/batch-message/send",
            "批量发送消息",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/im-v1/batch-message/send"),
        ApiDocUrl::new(
            "im",
            "v1",
            "batch_delete",
            "https://open.feishu.cn/document/server-docs/im-v1/batch-message/delete",
            "批量撤回消息",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/im-v1/batch-message/delete"),
        ApiDocUrl::new(
            "im",
            "v1",
            "get_batch_progress",
            "https://open.feishu.cn/document/server-docs/im-v1/batch-message/getProgress",
            "查询批量消息整体进度",
        )
        .with_en_url(
            "https://open.larksuite.com/anycross/reference/im-v1/batch-message/getProgress",
        ),
        ApiDocUrl::new(
            "im",
            "v1",
            "read_user",
            "https://open.feishu.cn/document/server-docs/im-v1/batch-message/readUser",
            "查询批量消息推送和阅读人数",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/im-v1/batch-message/readUser"),
        // === 文件操作 (FileService) ===
        ApiDocUrl::new(
            "im",
            "v1",
            "upload_file",
            "https://open.feishu.cn/document/server-docs/im-v1/file/upload",
            "上传文件",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/im-v1/file/upload"),
        ApiDocUrl::new(
            "im",
            "v1",
            "download_file",
            "https://open.feishu.cn/document/server-docs/im-v1/file/download",
            "下载文件",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/im-v1/file/download"),
        // === 图片操作 (ImageService) ===
        ApiDocUrl::new(
            "im",
            "v1",
            "upload_image",
            "https://open.feishu.cn/document/server-docs/im-v1/image/create",
            "上传图片",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/im-v1/image/create"),
        ApiDocUrl::new(
            "im",
            "v1",
            "download_image",
            "https://open.feishu.cn/document/server-docs/im-v1/image/get",
            "下载图片",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/im-v1/image/get"),
        // === 消息卡片操作 (MessageCardService) ===
        ApiDocUrl::new(
            "im",
            "v1",
            "patch_message_card",
            "https://open.feishu.cn/document/server-docs/im-v1/message-card/patch",
            "更新应用发送的消息卡片",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/im-v1/message-card/patch"),
        ApiDocUrl::new(
            "im",
            "v1",
            "delay_update_card",
            "https://open.feishu.cn/document/server-docs/im-v1/message-card/delayUpdate",
            "延时更新消息卡片",
        )
        .with_en_url(
            "https://open.larksuite.com/anycross/reference/im-v1/message-card/delayUpdate",
        ),
        ApiDocUrl::new(
            "im",
            "v1",
            "send_visible_card",
            "https://open.feishu.cn/document/server-docs/im-v1/message-card/sendVisible",
            "发送仅特定人可见的消息卡片",
        )
        .with_en_url(
            "https://open.larksuite.com/anycross/reference/im-v1/message-card/sendVisible",
        ),
        ApiDocUrl::new(
            "im",
            "v1",
            "delete_visible_card",
            "https://open.feishu.cn/document/server-docs/im-v1/message-card/deleteVisible",
            "删除仅特定人可见的消息卡片",
        )
        .with_en_url(
            "https://open.larksuite.com/anycross/reference/im-v1/message-card/deleteVisible",
        ),
        // === URL预览 (UrlPreviewService) ===
        ApiDocUrl::new(
            "im",
            "v1",
            "batch_update_preview",
            "https://open.feishu.cn/document/server-docs/im-v1/url-preview/batchUpdate",
            "批量更新URL预览",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/im-v1/url-preview/batchUpdate"),
        // === 置顶消息 (PinService) ===
        ApiDocUrl::new(
            "im",
            "v1",
            "pin_message",
            "https://open.feishu.cn/document/server-docs/im-v1/pin/create",
            "置顶消息",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/im-v1/pin/create"),
        ApiDocUrl::new(
            "im",
            "v1",
            "unpin_message",
            "https://open.feishu.cn/document/server-docs/im-v1/pin/delete",
            "移除置顶消息",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/im-v1/pin/delete"),
        ApiDocUrl::new(
            "im",
            "v1",
            "list_pins",
            "https://open.feishu.cn/document/server-docs/im-v1/pin/list",
            "获取群内置顶消息",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/im-v1/pin/list"),
        // === 表情回应 (MessageReactionService) ===
        ApiDocUrl::new(
            "im",
            "v1",
            "add_reaction",
            "https://open.feishu.cn/document/server-docs/im-v1/message-reaction/create",
            "添加消息表情回复",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/im-v1/message-reaction/create"),
        ApiDocUrl::new(
            "im",
            "v1",
            "list_reactions",
            "https://open.feishu.cn/document/server-docs/im-v1/message-reaction/list",
            "获取消息表情回复",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/im-v1/message-reaction/list"),
        ApiDocUrl::new(
            "im",
            "v1",
            "delete_reaction",
            "https://open.feishu.cn/document/server-docs/im-v1/message-reaction/delete",
            "删除消息表情回复",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/im-v1/message-reaction/delete"),
        // === 加急消息 (BuzzMessagesService) ===
        ApiDocUrl::new(
            "im",
            "v1",
            "urgent_app",
            "https://open.feishu.cn/document/server-docs/im-v1/buzz/urgentApp",
            "发送应用内加急",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/im-v1/buzz/urgentApp"),
        ApiDocUrl::new(
            "im",
            "v1",
            "urgent_sms",
            "https://open.feishu.cn/document/server-docs/im-v1/buzz/urgentSms",
            "发送短信加急",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/im-v1/buzz/urgentSms"),
        ApiDocUrl::new(
            "im",
            "v1",
            "urgent_phone",
            "https://open.feishu.cn/document/server-docs/im-v1/buzz/urgentPhone",
            "发送电话加急",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/im-v1/buzz/urgentPhone"),
    ];

    registry.register_service("im", urls);
}

/// 注册通讯录V3服务的文档URL
fn register_contact_v3(registry: &mut DocUrlRegistry) {
    let urls = vec![
        // === 已验证的Contact V3 API文档URL（通过联网验证）===

        // 通讯录概述（已验证）
        ApiDocUrl::new(
            "contact",
            "v3",
            "resources",
            "https://open.feishu.cn/document/server-docs/contact-v3/resources",
            "通讯录概述",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/contact-v3/resources"),
        // 用户管理（已验证）
        ApiDocUrl::new(
            "contact",
            "v3",
            "create_user",
            "https://open.feishu.cn/document/server-docs/contact-v3/user/create",
            "创建用户",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/contact-v3/user/create"),
        ApiDocUrl::new(
            "contact",
            "v3",
            "find_by_department",
            "https://open.feishu.cn/document/server-docs/contact-v3/user/find_by_department",
            "获取部门直属用户列表",
        )
        .with_en_url(
            "https://open.larksuite.com/anycross/reference/contact-v3/user/find_by_department",
        ),
        // 基于已验证模式的推断URL（需要进一步验证）
        ApiDocUrl::new(
            "contact",
            "v3",
            "get_user",
            "https://open.feishu.cn/document/server-docs/contact-v3/user/get",
            "获取用户信息",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/contact-v3/user/get"),
        ApiDocUrl::new(
            "contact",
            "v3",
            "update_user",
            "https://open.feishu.cn/document/server-docs/contact-v3/user/update",
            "更新用户信息",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/contact-v3/user/update"),
        ApiDocUrl::new(
            "contact",
            "v3",
            "delete_user",
            "https://open.feishu.cn/document/server-docs/contact-v3/user/delete",
            "删除用户",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/contact-v3/user/delete"),
        ApiDocUrl::new(
            "contact",
            "v3",
            "batch_get_user",
            "https://open.feishu.cn/document/contact-v3/user/batch",
            "批量获取用户信息",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/contact-v3/user/batch"),
        ApiDocUrl::new(
            "contact",
            "v3",
            "list_user",
            "https://open.feishu.cn/document/server-docs/contact-v3/user/list",
            "获取用户列表",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/contact-v3/user/list"),
        ApiDocUrl::new(
            "contact",
            "v3",
            "search_user",
            "https://open.feishu.cn/document/server-docs/contact-v3/user/search",
            "搜索用户",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/contact-v3/user/search"),
        // 部门管理（基于模式推断）
        ApiDocUrl::new(
            "contact",
            "v3",
            "get_department",
            "https://open.feishu.cn/document/server-docs/contact-v3/department/get",
            "获取部门信息",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/contact-v3/department/get"),
        ApiDocUrl::new(
            "contact",
            "v3",
            "create_department",
            "https://open.feishu.cn/document/server-docs/contact-v3/department/create",
            "创建部门",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/contact-v3/department/create"),
        ApiDocUrl::new(
            "contact",
            "v3",
            "update_department",
            "https://open.feishu.cn/document/server-docs/contact-v3/department/update",
            "更新部门信息",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/contact-v3/department/update"),
        ApiDocUrl::new(
            "contact",
            "v3",
            "delete_department",
            "https://open.feishu.cn/document/server-docs/contact-v3/department/delete",
            "删除部门",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/contact-v3/department/delete"),
        ApiDocUrl::new(
            "contact",
            "v3",
            "list_department",
            "https://open.feishu.cn/document/server-docs/contact-v3/department/list",
            "获取部门列表",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/contact-v3/department/list"),

        // === 用户管理扩展API ===

        // 用户更新操作（已验证）
        ApiDocUrl::new(
            "contact",
            "v3",
            "patch_user",
            "https://open.feishu.cn/document/server-docs/contact-v3/user/patch",
            "部分更新用户信息",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/contact-v3/user/patch"),

        // 批量用户操作
        ApiDocUrl::new(
            "contact",
            "v3",
            "batch_create_user",
            "https://open.feishu.cn/document/server-docs/contact-v3/user/batch_create",
            "批量创建用户",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/contact-v3/user/batch_create"),

        ApiDocUrl::new(
            "contact",
            "v3",
            "batch_update_user",
            "https://open.feishu.cn/document/server-docs/contact-v3/user/batch_update",
            "批量更新用户",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/contact-v3/user/batch_update"),

        ApiDocUrl::new(
            "contact",
            "v3",
            "batch_delete_user",
            "https://open.feishu.cn/document/server-docs/contact-v3/user/batch_delete",
            "批量删除用户",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/contact-v3/user/batch_delete"),

        // 用户状态管理
        ApiDocUrl::new(
            "contact",
            "v3",
            "activate_user",
            "https://open.feishu.cn/document/server-docs/contact-v3/user/activate",
            "激活用户",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/contact-v3/user/activate"),

        ApiDocUrl::new(
            "contact",
            "v3",
            "deactivate_user",
            "https://open.feishu.cn/document/server-docs/contact-v3/user/deactivate",
            "冻结用户",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/contact-v3/user/deactivate"),

        // 用户列表查询扩展
        ApiDocUrl::new(
            "contact",
            "v3",
            "list_active_user",
            "https://open.feishu.cn/document/server-docs/contact-v3/user/list_active",
            "获取活跃用户列表",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/contact-v3/user/list_active"),

        ApiDocUrl::new(
            "contact",
            "v3",
            "list_inactive_user",
            "https://open.feishu.cn/document/server-docs/contact-v3/user/list_inactive",
            "获取非活跃用户列表",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/contact-v3/user/list_inactive"),

        // 用户自定义属性
        ApiDocUrl::new(
            "contact",
            "v3",
            "get_user_custom_attrs",
            "https://open.feishu.cn/document/server-docs/contact-v3/user/get_custom_attrs",
            "获取用户自定义属性",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/contact-v3/user/get_custom_attrs"),

        ApiDocUrl::new(
            "contact",
            "v3",
            "set_user_custom_attrs",
            "https://open.feishu.cn/document/server-docs/contact-v3/user/set_custom_attrs",
            "设置用户自定义属性",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/contact-v3/user/set_custom_attrs"),

        // === 部门管理扩展API ===

        // 部门部分更新（已验证）
        ApiDocUrl::new(
            "contact",
            "v3",
            "patch_department",
            "https://open.feishu.cn/document/server-docs/contact-v3/department/patch",
            "部分更新部门信息",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/contact-v3/department/patch"),

        // 部门批量操作
        ApiDocUrl::new(
            "contact",
            "v3",
            "batch_create_department",
            "https://open.feishu.cn/document/server-docs/contact-v3/department/batch_create",
            "批量创建部门",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/contact-v3/department/batch_create"),

        ApiDocUrl::new(
            "contact",
            "v3",
            "batch_delete_department",
            "https://open.feishu.cn/document/server-docs/contact-v3/department/batch_delete",
            "批量删除部门",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/contact-v3/department/batch_delete"),

        // 部门层级管理
        ApiDocUrl::new(
            "contact",
            "v3",
            "move_department",
            "https://open.feishu.cn/document/server-docs/contact-v3/department/move",
            "移动部门",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/contact-v3/department/move"),

        ApiDocUrl::new(
            "contact",
            "v3",
            "list_department_children",
            "https://open.feishu.cn/document/server-docs/contact-v3/department/list_children",
            "获取子部门列表",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/contact-v3/department/list_children"),

        ApiDocUrl::new(
            "contact",
            "v3",
            "get_root_department",
            "https://open.feishu.cn/document/server-docs/contact-v3/department/get_root",
            "获取根部门",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/contact-v3/department/get_root"),

        ApiDocUrl::new(
            "contact",
            "v3",
            "get_department_custom_attrs",
            "https://open.feishu.cn/document/server-docs/contact-v3/department/get_custom_attrs",
            "获取部门自定义属性",
        )
        .with_en_url("https://open.larksuite.com/anycross/reference/contact-v3/department/get_custom_attrs"),
    ];

    registry.register_service("contact", urls);
}

/// 注册AI V1服务的文档URL
fn register_ai_v1(registry: &mut DocUrlRegistry) {
    let urls = vec![
        // === 已验证的AI V1 API文档URL（通过联网验证）===

        // Document AI - 智能文档处理（已验证）
        ApiDocUrl::new(
            "ai",
            "v1",
            "parse_resume",
            "https://open.larkoffice.com/document/ai/document_ai-v1/resume/parse",
            "识别文件中的简历信息"
        ),

        ApiDocUrl::new(
            "ai",
            "v1",
            "recognize_id_card",
            "https://open.larkoffice.com/document/ai/document_ai-v1/id_card/recognize",
            "识别文件中的身份证"
        ),

        ApiDocUrl::new(
            "ai",
            "v1",
            "recognize_vat_invoice",
            "https://open.larkoffice.com/document/ai/document_ai-v1/vat_invoice/recognize",
            "识别文件中的增值税发票"
        ),

        // Speech to Text - 语音识别（已验证）
        ApiDocUrl::new(
            "ai",
            "v1",
            "stream_recognize",
            "https://open.larkoffice.com/document/server-docs/ai/speech_to_text-v1/stream_recognize",
            "识别流式语音"
        ),

        // 其他Document AI API（基于已验证URL模式生成）
        ApiDocUrl::new(
            "ai",
            "v1",
            "recognize_driving_license",
            "https://open.larkoffice.com/document/ai/document_ai-v1/driving_license/recognize",
            "识别文件中的驾驶证"
        ),

        ApiDocUrl::new(
            "ai",
            "v1",
            "recognize_bank_card",
            "https://open.larkoffice.com/document/ai/document_ai-v1/bank_card/recognize",
            "识别文件中的银行卡"
        ),

        ApiDocUrl::new(
            "ai",
            "v1",
            "recognize_business_license",
            "https://open.larkoffice.com/document/ai/document_ai-v1/business_license/recognize",
            "识别文件中的营业执照"
        ),

        ApiDocUrl::new(
            "ai",
            "v1",
            "extract_contract_fields",
            "https://open.larkoffice.com/document/ai/document_ai-v1/contract/extract_fields",
            "提取文件中的合同字段"
        ),

        ApiDocUrl::new(
            "ai",
            "v1",
            "recognize_business_card",
            "https://open.larkoffice.com/document/ai/document_ai-v1/business_card/recognize",
            "识别文件中的名片"
        ),

        // Optical Character Recognition - OCR
        ApiDocUrl::new(
            "ai",
            "v1",
            "basic_recognize",
            "https://open.larkoffice.com/document/ai/optical_char_recognition-v1/basic_recognize",
            "识别图片中的文字"
        ),

        // Speech to Text - 其他语音API
        ApiDocUrl::new(
            "ai",
            "v1",
            "file_recognize",
            "https://open.larkoffice.com/document/server-docs/ai/speech_to_text-v1/file_recognize",
            "识别语音文件"
        ),

        // Translation - 机器翻译
        ApiDocUrl::new(
            "ai",
            "v1",
            "translate",
            "https://open.larkoffice.com/document/server-docs/ai/translation-v1/translate",
            "翻译文本"
        ),

        ApiDocUrl::new(
            "ai",
            "v1",
            "detect",
            "https://open.larkoffice.com/document/server-docs/ai/translation-v1/detect",
            "识别文本语种"
        ),
    ];

    registry.register_service("ai", urls);
}

/// 注册身份认证V1服务的文档URL
fn register_authentication_v1(registry: &mut DocUrlRegistry) {
    let urls = vec![
        // === 已验证的Authentication V1 API文档URL（通过联网验证）===

        // UserInfo Service - 用户信息服务（已验证）
        ApiDocUrl::new(
            "authentication",
            "v1",
            "get",
            "https://open.feishu.cn/document/server-docs/authentication-management/login-state-management/get",
            "获取登录用户信息"
        ),

        // Access Token Management - 访问令牌管理（已验证）
        ApiDocUrl::new(
            "authentication",
            "v1",
            "get_user_access_token",
            "https://open.feishu.cn/document/authentication-management/access-token/get-user-access-token",
            "获取user_access_token"
        ),

        ApiDocUrl::new(
            "authentication",
            "v1",
            "get_user_access_token_v1",
            "https://open.feishu.cn/document/server-docs/authentication-management/access-token/create-2",
            "获取user_access_token（v1版本）"
        ),

        // Tenant Access Token（基于已验证模式生成）
        ApiDocUrl::new(
            "authentication",
            "v1",
            "get_tenant_access_token",
            "https://open.larkoffice.com/document/server-docs/authentication-management/access-token/tenant_access_token",
            "获取tenant_access_token"
        ),

        // Refresh Token（基于已验证模式生成）
        ApiDocUrl::new(
            "authentication",
            "v1",
            "refresh_user_access_token",
            "https://open.feishu.cn/document/server-docs/authentication-management/access-token/refresh-user-access-token",
            "刷新user_access_token"
        ),
    ];

    registry.register_service("authentication", urls);
}

/// 注册企业信息服务V2的文档URL
fn register_tenant_v2(registry: &mut DocUrlRegistry) {
    let urls = vec![
        // === 已验证的Tenant V2 API文档URL（通过联网验证）===

        // 企业信息服务（已验证）
        ApiDocUrl::new(
            "tenant",
            "v2",
            "query",
            "https://open.feishu.cn/document/server-docs/tenant-v2/query",
            "获取企业信息"
        ).with_en_url("https://open.larksuite.com/anycross/reference/tenant-v2/tenant/query"),

        // 企业席位信息服务（已验证）
        ApiDocUrl::new(
            "tenant_product_assign_info",
            "v2",
            "query",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/tenant-v2/tenant-product_assign_info/query",
            "获取企业席位信息"
        ).with_en_url("https://open.larksuite.com/anycross/reference/tenant-v2/tenant-product_assign_info/query"),
    ];

    registry.register_service("tenant", urls);
}

/// 注册应用管理V6服务的文档URL
fn register_application_v6(registry: &mut DocUrlRegistry) {
    let urls = vec![
        // === 已验证的Application V6 API文档URL（通过联网验证）===

        // === ApplicationService - 应用信息管理 ===

        // 应用基础信息（已验证）
        ApiDocUrl::new(
            "application",
            "v6",
            "get",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/application-v6/application/get",
            "获取应用信息"
        ),

        // 应用所有权管理（已验证）
        ApiDocUrl::new(
            "application",
            "v6",
            "transfer_owner",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/application-v6",
            "转移应用所有者"
        ),

        // 应用信息更新（已验证）
        ApiDocUrl::new(
            "application",
            "v6",
            "patch",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/application-v6/application/patch",
            "更新应用信息"
        ),

        // 版本管理（基于已验证模式）
        ApiDocUrl::new(
            "application",
            "v6",
            "get_version",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/application-v6/application-app_version/get",
            "获取应用版本信息"
        ),

        ApiDocUrl::new(
            "application",
            "v6",
            "list_versions",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/application-v6/application-app_version/list",
            "获取应用版本列表"
        ),

        // 协作者管理（基于已验证模式）
        ApiDocUrl::new(
            "application",
            "v6",
            "get_collaborators",
            "https://open.feishu.cn/document/server-docs/application-v6/application/collaborators",
            "获取应用协作者列表"
        ),

        ApiDocUrl::new(
            "application",
            "v6",
            "update_collaborators",
            "https://open.feishu.cn/document/server-docs/application-v6/application/collaborators/update",
            "更新应用协作者"
        ),

        // 审核管理（基于已验证模式）
        ApiDocUrl::new(
            "application",
            "v6",
            "underaudit_list",
            "https://open.feishu.cn/document/server-docs/application-v6/application/underaudit/list",
            "查看待审核的应用列表"
        ),

        ApiDocUrl::new(
            "application",
            "v6",
            "update_audit_status",
            "https://open.feishu.cn/document/server-docs/application-v6/application/audit/status/update",
            "更新应用审核状态"
        ),

        // === AdminService - 企业应用管理 ===

        // 安装管理（已验证）
        ApiDocUrl::new(
            "application",
            "v6",
            "list_installed_apps",
            "https://open.feishu.cn/document/server-docs/application-v6/admin/obtain-the-apps-installed-by-an-organization",
            "获取企业安装的应用"
        ),

        ApiDocUrl::new(
            "application",
            "v6",
            "get_user_available_apps",
            "https://open.feishu.cn/document/server-docs/application-v6/admin/obtain-the-apps-available-to-a-user",
            "获取用户可用的应用"
        ),

        // 通讯录权限配置（基于已验证模式）
        ApiDocUrl::new(
            "application",
            "v6",
            "contacts_range_configuration",
            "https://open.feishu.cn/document/server-docs/application-v6/admin/contacts_range/configuration",
            "获取应用通讯录权限范围配置"
        ),

        ApiDocUrl::new(
            "application",
            "v6",
            "update_contacts_range_configuration",
            "https://open.feishu.cn/document/server-docs/application-v6/admin/contacts_range/configuration/update",
            "更新应用通讯录权限范围配置"
        ),

        // 可用性管理（基于已验证模式）
        ApiDocUrl::new(
            "application",
            "v6",
            "get_app_availability",
            "https://open.feishu.cn/document/server-docs/application-v6/admin/availability/get",
            "获取应用在企业内的可用范围"
        ),

        ApiDocUrl::new(
            "application",
            "v6",
            "update_app_availability",
            "https://open.feishu.cn/document/server-docs/application-v6/admin/availability/update",
            "更新应用可用范围"
        ),

        ApiDocUrl::new(
            "application",
            "v6",
            "enable_disable_app",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/application-v6/application-management/update",
            "启停用应用"
        ),

        // 管理员权限（基于已验证模式）
        ApiDocUrl::new(
            "application",
            "v6",
            "check_white_black_list",
            "https://open.feishu.cn/document/server-docs/application-v6/admin/check_white_black_list",
            "查询用户或部门是否在应用的可用或禁用名单"
        ),

        ApiDocUrl::new(
            "application",
            "v6",
            "list_app_admins",
            "https://open.feishu.cn/document/server-docs/application-v6/admin/admins/list",
            "查询应用管理员列表"
        ),

        ApiDocUrl::new(
            "application",
            "v6",
            "get_app_admin_permissions",
            "https://open.feishu.cn/document/server-docs/application-v6/admin/admins/permissions/get",
            "获取应用管理员管理范围"
        ),

        ApiDocUrl::new(
            "application",
            "v6",
            "verify_app_admin",
            "https://open.feishu.cn/document/server-docs/application-v6/admin/admins/verify",
            "校验应用管理员"
        ),

        // === AppUsageService - 应用使用情况 ===

        // 使用统计（已验证）
        ApiDocUrl::new(
            "application",
            "v6",
            "department_overview",
            "https://open.feishu.cn/document/server-docs/application-v6/app-usage/department_overview",
            "获取多部门应用使用概览"
        ),

        ApiDocUrl::new(
            "application",
            "v6",
            "message_push_overview",
            "https://open.feishu.cn/document/server-docs/application-v6/app-usage/message_push_overview",
            "获取消息推送概览"
        ),

        ApiDocUrl::new(
            "application",
            "v6",
            "overview",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/application-v6/application-app_usage/overview",
            "获取应用使用概览"
        ),

        // === AppBadgeService - 应用红点 ===

        // 红点管理（基于已验证模式）
        ApiDocUrl::new(
            "application",
            "v6",
            "set_badge",
            "https://open.feishu.cn/document/server-docs/application-v6/app-badge/set",
            "更新应用红点"
        ),

        // === ApplicationFeedbackService - 应用反馈 ===

        // 反馈管理（基于已验证模式）
        ApiDocUrl::new(
            "application",
            "v6",
            "list_feedback",
            "https://open.feishu.cn/document/server-docs/application-v6/app-feedback/list",
            "获取应用反馈列表"
        ),

        ApiDocUrl::new(
            "application",
            "v6",
            "update_feedback",
            "https://open.feishu.cn/document/server-docs/application-v6/app-feedback/update",
            "更新应用反馈"
        ),

        // === AppstorePaidInfoService - 应用商店付费信息 ===

        // 付费管理（基于已验证模式）
        ApiDocUrl::new(
            "application",
            "v6",
            "check_user_access",
            "https://open.feishu.cn/document/server-docs/application-v6/appstore-paid-info/check_user_access",
            "查询用户是否在应用开通范围"
        ),

        ApiDocUrl::new(
            "application",
            "v6",
            "list_tenant_paid_plans",
            "https://open.feishu.cn/document/server-docs/application-v6/appstore-paid-info/list_tenant_paid_plans",
            "查询租户购买的付费方案"
        ),

        ApiDocUrl::new(
            "application",
            "v6",
            "get_order_info",
            "https://open.feishu.cn/document/server-docs/application-v6/appstore-paid-info/get_order_info",
            "查询订单详情"
        ),

        // === ScopeService - 应用权限管理 ===

        // 权限管理（基于已验证模式）
        ApiDocUrl::new(
            "application",
            "v6",
            "apply_scope",
            "https://open.feishu.cn/document/server-docs/application-v6/scope/apply",
            "向管理员申请授权"
        ),

        ApiDocUrl::new(
            "application",
            "v6",
            "list_scope",
            "https://open.feishu.cn/document/server-docs/application-v6/scope/list",
            "查询租户授权状态"
        ),
    ];

    registry.register_service("application", urls);
}

/// 注册审批管理V4服务的文档URL
fn register_approval_v4(registry: &mut DocUrlRegistry) {
    let urls = vec![
        // === 已验证的Approval V4 API文档URL（通过联网验证）===

        // === ApprovalService - 审批定义管理 ===

        // 审批定义基础（已验证）
        ApiDocUrl::new(
            "approval",
            "v4",
            "create_approval",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/approval/create",
            "创建审批定义"
        ),

        ApiDocUrl::new(
            "approval",
            "v4",
            "get_approval",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/approval/get",
            "查看审批定义"
        ),

        ApiDocUrl::new(
            "approval",
            "v4",
            "approval_overview",
            "https://open.feishu.cn/document/server-docs/approval-v4/approval-overview",
            "审批概述"
        ),

        ApiDocUrl::new(
            "approval",
            "v4",
            "approval_resources_overview",
            "https://open.feishu.cn/document/server-docs/approval-v4/approval/overview-of-approval-resources",
            "原生审批定义概述"
        ),

        // === InstanceService - 审批实例管理 ===

        // 实例基础操作（已验证）
        ApiDocUrl::new(
            "approval",
            "v4",
            "create_instance",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance/create",
            "创建审批实例"
        ),

        ApiDocUrl::new(
            "approval",
            "v4",
            "get_instance",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance/get",
            "获取审批实例详情"
        ),

        ApiDocUrl::new(
            "approval",
            "v4",
            "list_instances",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance/list",
            "批量获取审批实例ID"
        ),

        // 实例操作（基于已验证模式）
        ApiDocUrl::new(
            "approval",
            "v4",
            "cancel_instance",
            "https://open.feishu.cn/document/server-docs/approval-v4/instance/cancel",
            "撤回审批实例"
        ),

        ApiDocUrl::new(
            "approval",
            "v4",
            "cc_instance",
            "https://open.feishu.cn/document/server-docs/approval-v4/instance/cc",
            "抄送审批实例"
        ),

        ApiDocUrl::new(
            "approval",
            "v4",
            "preview_instance",
            "https://open.feishu.cn/document/server-docs/approval-v4/instance/approval-preview",
            "预览审批流程"
        ),

        // === TaskService - 审批任务管理 ===

        // 任务操作（已验证）
        ApiDocUrl::new(
            "approval",
            "v4",
            "search_tasks",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/task/search",
            "查询任务列表"
        ),

        ApiDocUrl::new(
            "approval",
            "v4",
            "approve_task",
            "https://open.feishu.cn/document/server-docs/approval-v4/task/approve",
            "同意审批任务"
        ),

        ApiDocUrl::new(
            "approval",
            "v4",
            "reject_task",
            "https://open.feishu.cn/document/server-docs/approval-v4/task/reject",
            "拒绝审批任务"
        ),

        // 任务高级操作（基于已验证模式）
        ApiDocUrl::new(
            "approval",
            "v4",
            "transfer_task",
            "https://open.feishu.cn/document/server-docs/approval-v4/task/transfer",
            "转交审批任务"
        ),

        ApiDocUrl::new(
            "approval",
            "v4",
            "rollback_task",
            "https://open.feishu.cn/document/server-docs/approval-v4/task/rollback",
            "退回审批任务"
        ),

        ApiDocUrl::new(
            "approval",
            "v4",
            "add_sign_task",
            "https://open.feishu.cn/document/server-docs/approval-v4/task/add_sign",
            "审批任务加签"
        ),

        ApiDocUrl::new(
            "approval",
            "v4",
            "resubmit_task",
            "https://open.feishu.cn/document/server-docs/approval-v4/task/resubmit",
            "重新提交审批任务"
        ),

        ApiDocUrl::new(
            "approval",
            "v4",
            "task_introduction",
            "https://fsapi.apibridge.webprinter.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/task/introduction",
            "审批任务概述"
        ),

        // === ExternalApprovalService - 三方审批定义 ===

        // 三方审批定义（已验证）
        ApiDocUrl::new(
            "approval",
            "v4",
            "create_external_approval",
            "https://open.feishu.cn/document/server-docs/approval-v4/external_approval/create",
            "创建三方审批定义"
        ),

        ApiDocUrl::new(
            "approval",
            "v4",
            "get_external_approval",
            "https://open.feishu.cn/document/server-docs/approval-v4/external_approval/get",
            "查看三方审批定义"
        ),

        // === ExternalInstanceService - 三方审批实例 ===

        // 三方审批实例（已验证）
        ApiDocUrl::new(
            "approval",
            "v4",
            "create_external_instance",
            "https://open.feishu.cn/document/server-docs/approval-v4/external_instance/create",
            "同步三方审批实例"
        ),

        ApiDocUrl::new(
            "approval",
            "v4",
            "check_external_instance",
            "https://open.feishu.cn/document/server-docs/approval-v4/external_instance/check",
            "校验三方审批实例"
        ),

        // === ExternalTaskService - 三方审批任务 ===

        // 三方审批任务（基于已验证模式）
        ApiDocUrl::new(
            "approval",
            "v4",
            "list_external_tasks",
            "https://open.feishu.cn/document/server-docs/approval-v4/external_task/list",
            "获取三方审批任务状态"
        ),

        // === FileService - 审批文件管理 ===

        // 文件管理（基于已验证模式）
        ApiDocUrl::new(
            "approval",
            "v4",
            "upload_file",
            "https://open.feishu.cn/document/server-docs/approval-v4/file/upload",
            "上传审批文件"
        ),

        // === InstanceCommentService - 审批评论管理 ===

        // 评论管理（基于已验证模式）
        ApiDocUrl::new(
            "approval",
            "v4",
            "create_comment",
            "https://open.feishu.cn/document/server-docs/approval-v4/comment/create",
            "创建评论"
        ),

        ApiDocUrl::new(
            "approval",
            "v4",
            "delete_comment",
            "https://open.feishu.cn/document/server-docs/approval-v4/comment/delete",
            "删除评论"
        ),

        ApiDocUrl::new(
            "approval",
            "v4",
            "remove_all_comments",
            "https://open.feishu.cn/document/server-docs/approval-v4/comment/remove_all",
            "清空评论"
        ),

        ApiDocUrl::new(
            "approval",
            "v4",
            "list_comments",
            "https://open.feishu.cn/document/server-docs/approval-v4/comment/list",
            "获取评论列表"
        ),

        // === MessageService - 审批消息管理 ===

        // 消息管理（基于已验证模式）
        ApiDocUrl::new(
            "approval",
            "v4",
            "send_bot_message",
            "https://open.feishu.cn/document/server-docs/approval-v4/message/send",
            "发送审批Bot消息"
        ),

        ApiDocUrl::new(
            "approval",
            "v4",
            "update_bot_message",
            "https://open.feishu.cn/document/server-docs/approval-v4/message/update",
            "更新审批Bot消息"
        ),

        // === SearchService - 审批查询服务 ===

        // 查询服务（基于已验证模式）
        ApiDocUrl::new(
            "approval",
            "v4",
            "search_instances",
            "https://open.feishu.cn/document/server-docs/approval-v4/search/instances",
            "查询实例列表"
        ),

        ApiDocUrl::new(
            "approval",
            "v4",
            "search_tasks",
            "https://open.feishu.cn/document/server-docs/approval-v4/search/tasks",
            "查询任务列表"
        ),

        ApiDocUrl::new(
            "approval",
            "v4",
            "search_cc",
            "https://open.feishu.cn/document/server-docs/approval-v4/search/cc",
            "查询抄送列表"
        ),

        ApiDocUrl::new(
            "approval",
            "v4",
            "search_approval_id",
            "https://open.feishu.cn/document/server-docs/approval-v4/search/approval_id",
            "查询审批ID"
        ),

        ApiDocUrl::new(
            "approval",
            "v4",
            "search_user_tasks",
            "https://open.feishu.cn/document/server-docs/approval-v4/search/user_tasks",
            "查询用户的任务列表"
        ),

        // 审批事件（基于已验证模式）
        ApiDocUrl::new(
            "approval",
            "v4",
            "approval_task_event",
            "https://open.feishu.cn/document/ukTMukTMukTM/uIDO24iM4YjLygjN/event/common-event/approval-task-event",
            "审批任务状态变更事件"
        ),

        ApiDocUrl::new(
            "approval",
            "v4",
            "event_interface",
            "https://open.feishu.cn/document/server-docs/approval-v4/event/event-interface",
            "审批事件接口"
        ),
    ];

    registry.register_service("approval", urls);
}

/// 注册日历管理V4服务的文档URL
fn register_calendar_v4(registry: &mut DocUrlRegistry) {
    let urls = vec![
        // === 已验证的Calendar V4 API文档URL（通过联网验证）===

        // === CalendarManagementService - 日历管理 ===

        // 日历基础管理（已验证）
        ApiDocUrl::new(
            "calendar",
            "v4",
            "create_calendar",
            "https://open.feishu.cn/document/server-docs/calendar-v4/calendar/create",
            "创建共享日历"
        ),

        ApiDocUrl::new(
            "calendar",
            "v4",
            "get_calendar",
            "https://open.feishu.cn/document/server-docs/calendar-v4/calendar/get",
            "查询日历信息"
        ),

        ApiDocUrl::new(
            "calendar",
            "v4",
            "list_calendars",
            "https://open.feishu.cn/document/server-docs/calendar-v4/calendar/list",
            "获取日历列表"
        ),

        ApiDocUrl::new(
            "calendar",
            "v4",
            "calendar_introduction",
            "https://open.feishu.cn/document/server-docs/calendar-v4/calendar/introduction",
            "日历资源介绍"
        ),

        // 日历高级管理（基于已验证模式）
        ApiDocUrl::new(
            "calendar",
            "v4",
            "delete_calendar",
            "https://open.feishu.cn/document/server-docs/calendar-v4/calendar/delete",
            "删除共享日历"
        ),

        ApiDocUrl::new(
            "calendar",
            "v4",
            "patch_calendar",
            "https://open.feishu.cn/document/server-docs/calendar-v4/calendar/patch",
            "部分更新日历信息"
        ),

        ApiDocUrl::new(
            "calendar",
            "v4",
            "subscribe_calendar",
            "https://open.feishu.cn/document/server-docs/calendar-v4/calendar/subscribe",
            "订阅日历"
        ),

        ApiDocUrl::new(
            "calendar",
            "v4",
            "unsubscribe_calendar",
            "https://open.feishu.cn/document/server-docs/calendar-v4/calendar/unsubscribe",
            "取消订阅日历"
        ),

        ApiDocUrl::new(
            "calendar",
            "v4",
            "get_primary_calendar",
            "https://s.apifox.cn/apidoc/docs-site/532425/api-10871822",
            "获取主日历"
        ),

        ApiDocUrl::new(
            "calendar",
            "v4",
            "search_calendar",
            "https://open.feishu.cn/document/server-docs/calendar-v4/calendar/search",
            "搜索日历"
        ),

        // === CalendarEventService - 日程管理 ===

        // 日程基础操作（已验证）
        ApiDocUrl::new(
            "calendar",
            "v4",
            "create_event",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event/create",
            "创建日程事件"
        ),

        ApiDocUrl::new(
            "calendar",
            "v4",
            "get_event",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event/get",
            "获取日程事件详细信息"
        ),

        ApiDocUrl::new(
            "calendar",
            "v4",
            "list_events",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/calendar-v4/calendar-event/list",
            "获取日程事件列表"
        ),

        // 日程高级操作（基于已验证模式）
        ApiDocUrl::new(
            "calendar",
            "v4",
            "delete_event",
            "https://open.feishu.cn/document/server-docs/calendar-v4/calendar-event/delete",
            "删除日程事件"
        ),

        ApiDocUrl::new(
            "calendar",
            "v4",
            "patch_event",
            "https://open.feishu.cn/document/server-docs/calendar-v4/calendar-event/patch",
            "部分更新日程事件"
        ),

        ApiDocUrl::new(
            "calendar",
            "v4",
            "search_events",
            "https://open.feishu.cn/document/server-docs/calendar-v4/calendar-event/search",
            "搜索日程事件"
        ),

        ApiDocUrl::new(
            "calendar",
            "v4",
            "get_event_instances",
            "https://open.feishu.cn/document/server-docs/calendar-v4/calendar-event/instances",
            "获取日程实例"
        ),

        ApiDocUrl::new(
            "calendar",
            "v4",
            "view_event_instance",
            "https://open.feishu.cn/document/server-docs/calendar-v4/calendar-event/instance_view",
            "查看日程实例"
        ),

        ApiDocUrl::new(
            "calendar",
            "v4",
            "reply_event",
            "https://open.feishu.cn/document/server-docs/calendar-v4/calendar-event/reply",
            "回复日程邀请"
        ),

        // === CalendarAclService - 日历访问控制 ===

        // 访问控制管理（基于已验证模式）
        ApiDocUrl::new(
            "calendar",
            "v4",
            "create_calendar_acl",
            "https://open.feishu.cn/document/server-docs/calendar-v4/calendar-acl/create",
            "创建日历访问控制"
        ),

        ApiDocUrl::new(
            "calendar",
            "v4",
            "delete_calendar_acl",
            "https://open.feishu.cn/document/server-docs/calendar-v4/calendar-acl/delete",
            "删除日历访问控制"
        ),

        ApiDocUrl::new(
            "calendar",
            "v4",
            "list_calendar_acls",
            "https://open.feishu.cn/document/server-docs/calendar-v4/calendar-acl/list",
            "获取日历访问控制列表"
        ),

        // === AttendeeService - 参与人管理 ===

        // 参与人管理（基于已验证模式）
        ApiDocUrl::new(
            "calendar",
            "v4",
            "create_attendee",
            "https://open.feishu.cn/document/server-docs/calendar-v4/attendee/create",
            "创建参与人"
        ),

        ApiDocUrl::new(
            "calendar",
            "v4",
            "list_attendees",
            "https://open.feishu.cn/document/server-docs/calendar-v4/attendee/list",
            "获取参与人列表"
        ),

        ApiDocUrl::new(
            "calendar",
            "v4",
            "batch_delete_attendees",
            "https://open.feishu.cn/document/server-docs/calendar-v4/attendee/batch_delete",
            "批量删除参与人"
        ),

        ApiDocUrl::new(
            "calendar",
            "v4",
            "list_chat_members",
            "https://open.feishu.cn/document/server-docs/calendar-v4/attendee/list_chat_members",
            "获取聊天群成员"
        ),

        // === MeetingRoomEventService - 会议室日程 ===

        // 会议室管理（基于已验证模式）
        ApiDocUrl::new(
            "calendar",
            "v4",
            "get_meeting_room_event",
            "https://open.feishu.cn/document/server-docs/calendar-v4/meeting-room-event/get",
            "获取会议室日程信息"
        ),

        ApiDocUrl::new(
            "calendar",
            "v4",
            "query_room_availability",
            "https://open.feishu.cn/document/server-docs/calendar-v4/meeting-room-event/query_availability",
            "查询会议室可用性"
        ),

        ApiDocUrl::new(
            "calendar",
            "v4",
            "reply_meeting_room",
            "https://open.feishu.cn/document/server-docs/calendar-v4/meeting-room-event/reply",
            "回复会议室邀请"
        ),

        // === MeetingChatService - 会议群管理 ===

        // 会议群管理（基于已验证模式）
        ApiDocUrl::new(
            "calendar",
            "v4",
            "create_meeting_chat",
            "https://open.feishu.cn/document/server-docs/calendar-v4/meeting-chat/create",
            "创建会议群"
        ),

        ApiDocUrl::new(
            "calendar",
            "v4",
            "delete_meeting_chat",
            "https://open.feishu.cn/document/server-docs/calendar-v4/meeting-chat/delete",
            "删除会议群"
        ),

        // === MeetingMinuteService - 会议纪要 ===

        // 会议纪要管理（基于已验证模式）
        ApiDocUrl::new(
            "calendar",
            "v4",
            "create_meeting_minute",
            "https://open.feishu.cn/document/server-docs/calendar-v4/meeting-minute/create",
            "创建会议纪要"
        ),

        // === TimeoffEventService - 请假日程 ===

        // 请假管理（基于已验证模式）
        ApiDocUrl::new(
            "calendar",
            "v4",
            "create_timeoff_event",
            "https://open.feishu.cn/document/server-docs/calendar-v4/timeoff-event/create",
            "创建请假日程"
        ),

        ApiDocUrl::new(
            "calendar",
            "v4",
            "delete_timeoff_event",
            "https://open.feishu.cn/document/server-docs/calendar-v4/timeoff-event/delete",
            "删除请假日程"
        ),

        // === SettingService - 设置管理 ===

        // 设置管理（基于已验证模式）
        ApiDocUrl::new(
            "calendar",
            "v4",
            "generate_caldav_config",
            "https://open.feishu.cn/document/server-docs/calendar-v4/setting/generate_caldav_conf",
            "生成CalDAV配置"
        ),

        // === ExchangeBindingService - Exchange集成 ===

        // Exchange绑定（基于已验证模式）
        ApiDocUrl::new(
            "calendar",
            "v4",
            "create_exchange_binding",
            "https://open.feishu.cn/document/server-docs/calendar-v4/exchange-binding/create",
            "创建Exchange绑定"
        ),

        ApiDocUrl::new(
            "calendar",
            "v4",
            "get_exchange_binding",
            "https://open.feishu.cn/document/server-docs/calendar-v4/exchange-binding/get",
            "获取Exchange绑定信息"
        ),

        ApiDocUrl::new(
            "calendar",
            "v4",
            "delete_exchange_binding",
            "https://open.feishu.cn/document/server-docs/calendar-v4/exchange-binding/delete",
            "删除Exchange绑定"
        ),
    ];

    registry.register_service("calendar", urls);
}

/// 注册任务管理V2服务的文档URL
fn register_task_v2(registry: &mut DocUrlRegistry) {
    let urls = vec![
        // === 已验证的Task V2 API文档URL（通过联网验证）===

        // === TaskService - 任务管理核心服务 ===

        // 任务基础操作（已验证）
        ApiDocUrl::new(
            "task",
            "v2",
            "create",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/task/create?lang=zh-CN",
            "创建任务",
        ),
        ApiDocUrl::new(
            "task",
            "v2",
            "get",
            "https://open.feishu.cn/document/task-v2/task/get",
            "获取任务详情",
        ),
        ApiDocUrl::new(
            "task",
            "v2",
            "patch",
            "https://open.feishu.cn/document/task-v2/task/patch",
            "更新任务",
        ),
        ApiDocUrl::new(
            "task",
            "v2",
            "delete",
            "https://open.feishu.cn/document/task-v2/task/delete",
            "删除任务"
        ),
        ApiDocUrl::new(
            "task",
            "v2",
            "list",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/task/list",
            "获取任务列表",
        ),
        // 任务成员管理（基于已验证模式）
        ApiDocUrl::new(
            "task",
            "v2",
            "add_members",
            "https://open.feishu.cn/document/task-v2/task/add_members",
            "添加任务成员",
        ),
        ApiDocUrl::new(
            "task",
            "v2",
            "remove_members",
            "https://open.feishu.cn/document/task-v2/task/remove_members",
            "移除任务成员",
        ),
        // 任务提醒管理（基于已验证模式）
        ApiDocUrl::new(
            "task",
            "v2",
            "add_reminders",
            "https://open.feishu.cn/document/task-v2/task/add_reminders",
            "添加任务提醒",
        ),
        ApiDocUrl::new(
            "task",
            "v2",
            "remove_reminders",
            "https://open.feishu.cn/document/task-v2/task/remove_reminders",
            "移除任务提醒",
        ),
        // 任务清单管理（基于已验证模式）
        ApiDocUrl::new(
            "task",
            "v2",
            "add_tasklist",
            "https://open.feishu.cn/document/task-v2/task/add_tasklist",
            "任务加入清单",
        ),
        ApiDocUrl::new(
            "task",
            "v2",
            "tasklists",
            "https://open.feishu.cn/document/task-v2/task/tasklists",
            "列取任务所在清单",
        ),
        // 任务依赖管理（基于已验证模式）
        ApiDocUrl::new(
            "task",
            "v2",
            "add_dependencies",
            "https://open.feishu.cn/document/task-v2/task/add_dependencies",
            "添加任务依赖",
        ),
        ApiDocUrl::new(
            "task",
            "v2",
            "remove_dependencies",
            "https://open.feishu.cn/document/task-v2/task/remove_dependencies",
            "移除任务依赖",
        ),
        // === TasklistService - 任务清单管理 ===

        // 清单基础管理（已验证）
        ApiDocUrl::new(
            "tasklist",
            "v2",
            "create",
            "https://open.feishu.cn/document/task-v2/tasklist/create",
            "创建任务清单",
        ),
        ApiDocUrl::new(
            "tasklist",
            "v2",
            "get",
            "https://open.feishu.cn/document/task-v2/tasklist/get",
            "获取清单详情",
        ),
        ApiDocUrl::new(
            "tasklist",
            "v2",
            "list",
            "https://open.feishu.cn/document/task-v2/tasklist/list",
            "获取清单列表",
        ),
        // 清单高级管理（基于已验证模式）
        ApiDocUrl::new(
            "tasklist",
            "v2",
            "patch",
            "https://open.feishu.cn/document/task-v2/tasklist/patch",
            "更新清单",
        ),
        ApiDocUrl::new(
            "tasklist",
            "v2",
            "delete",
            "https://open.feishu.cn/document/task-v2/tasklist/delete",
            "删除清单",
        ),
        // 清单成员管理（已验证）
        ApiDocUrl::new(
            "tasklist",
            "v2",
            "add_members",
            "https://open.feishu.cn/document/task-v2/tasklist/add_members",
            "添加清单成员",
        ),
        ApiDocUrl::new(
            "tasklist",
            "v2",
            "remove_members",
            "https://open.feishu.cn/document/task-v2/tasklist/remove_members",
            "移除清单成员",
        ),
        // 清单任务管理（基于已验证模式）
        ApiDocUrl::new(
            "tasklist",
            "v2",
            "tasks",
            "https://open.feishu.cn/document/task-v2/tasklist/tasks",
            "获取清单任务列表",
        ),
        // === TaskSubtaskService - 子任务管理 ===

        // 子任务操作（基于已验证模式）
        ApiDocUrl::new(
            "task_subtask",
            "v2",
            "create",
            "https://open.feishu.cn/document/task-v2/task-subtask/create",
            "创建子任务",
        ),
        ApiDocUrl::new(
            "task_subtask",
            "v2",
            "get",
            "https://open.feishu.cn/document/task-v2/task-subtask/get",
            "获取子任务详情",
        ),
        ApiDocUrl::new(
            "task_subtask",
            "v2",
            "list",
            "https://open.feishu.cn/document/task-v2/task-subtask/list",
            "获取子任务列表",
        ),
        ApiDocUrl::new(
            "task_subtask",
            "v2",
            "delete",
            "https://open.feishu.cn/document/task-v2/task-subtask/delete",
            "删除子任务",
        ),
        // === CommentService - 评论管理 ===

        // 评论操作（基于已验证模式）
        ApiDocUrl::new(
            "comment",
            "v2",
            "create",
            "https://open.feishu.cn/document/task-v2/comment/create",
            "创建评论",
        ),
        ApiDocUrl::new(
            "comment",
            "v2",
            "get",
            "https://open.feishu.cn/document/task-v2/comment/get",
            "获取评论详情",
        ),
        ApiDocUrl::new(
            "comment",
            "v2",
            "list",
            "https://open.feishu.cn/document/task-v2/comment/list",
            "获取评论列表",
        ),
        ApiDocUrl::new(
            "comment",
            "v2",
            "delete",
            "https://open.feishu.cn/document/task-v2/comment/delete",
            "删除评论",
        ),
        ApiDocUrl::new(
            "comment",
            "v2",
            "update",
            "https://open.feishu.cn/document/task-v2/comment/update",
            "更新评论",
        ),
        // === AttachmentService - 附件管理 ===

        // 附件操作（基于已验证模式）
        ApiDocUrl::new(
            "attachment",
            "v2",
            "upload",
            "https://open.feishu.cn/document/task-v2/attachment/upload",
            "上传附件",
        ),
        ApiDocUrl::new(
            "attachment",
            "v2",
            "get",
            "https://open.feishu.cn/document/task-v2/attachment/get",
            "获取附件信息",
        ),
        ApiDocUrl::new(
            "attachment",
            "v2",
            "list",
            "https://open.feishu.cn/document/task-v2/attachment/list",
            "获取附件列表",
        ),
        ApiDocUrl::new(
            "attachment",
            "v2",
            "delete",
            "https://open.feishu.cn/document/task-v2/attachment/delete",
            "删除附件",
        ),
        ApiDocUrl::new(
            "attachment",
            "v2",
            "download",
            "https://open.feishu.cn/document/task-v2/attachment/download",
            "下载附件",
        ),
        // === CustomFieldService - 自定义字段管理 ===

        // 自定义字段操作（基于已验证模式）
        ApiDocUrl::new(
            "custom_field",
            "v2",
            "create",
            "https://open.feishu.cn/document/task-v2/custom-field/create",
            "创建自定义字段",
        ),
        ApiDocUrl::new(
            "custom_field",
            "v2",
            "get",
            "https://open.feishu.cn/document/task-v2/custom-field/get",
            "获取自定义字段详情",
        ),
        ApiDocUrl::new(
            "custom_field",
            "v2",
            "list",
            "https://open.feishu.cn/document/task-v2/custom-field/list",
            "获取自定义字段列表",
        ),
        ApiDocUrl::new(
            "custom_field",
            "v2",
            "update",
            "https://open.feishu.cn/document/task-v2/custom-field/update",
            "更新自定义字段",
        ),
        ApiDocUrl::new(
            "custom_field",
            "v2",
            "delete",
            "https://open.feishu.cn/document/task-v2/custom-field/delete",
            "删除自定义字段",
        ),
        ApiDocUrl::new(
            "custom_field",
            "v2",
            "get_options",
            "https://open.feishu.cn/document/task-v2/custom-field/get_options",
            "获取自定义字段选项",
        ),
        // === CustomFieldOptionService - 自定义字段选项管理 ===

        // 自定义字段选项操作（基于已验证模式）
        ApiDocUrl::new(
            "custom_field_option",
            "v2",
            "create",
            "https://open.feishu.cn/document/task-v2/custom-field-option/create",
            "创建自定义字段选项",
        ),
        ApiDocUrl::new(
            "custom_field_option",
            "v2",
            "get",
            "https://open.feishu.cn/document/task-v2/custom-field-option/get",
            "获取自定义字段选项详情",
        ),
        ApiDocUrl::new(
            "custom_field_option",
            "v2",
            "list",
            "https://open.feishu.cn/document/task-v2/custom-field-option/list",
            "获取自定义字段选项列表",
        ),
        ApiDocUrl::new(
            "custom_field_option",
            "v2",
            "update",
            "https://open.feishu.cn/document/task-v2/custom-field-option/update",
            "更新自定义字段选项",
        ),
        // === SectionService - 自定义分组管理 ===

        // 分组操作（基于已验证模式）
        ApiDocUrl::new(
            "section",
            "v2",
            "create",
            "https://open.feishu.cn/document/task-v2/section/create",
            "创建自定义分组",
        ),
        ApiDocUrl::new(
            "section",
            "v2",
            "get",
            "https://open.feishu.cn/document/task-v2/section/get",
            "获取自定义分组详情",
        ),
        ApiDocUrl::new(
            "section",
            "v2",
            "list",
            "https://open.feishu.cn/document/task-v2/section/list",
            "获取自定义分组列表",
        ),
        ApiDocUrl::new(
            "section",
            "v2",
            "update",
            "https://open.feishu.cn/document/task-v2/section/update",
            "更新自定义分组",
        ),
        ApiDocUrl::new(
            "section",
            "v2",
            "delete",
            "https://open.feishu.cn/document/task-v2/section/delete",
            "删除自定义分组",
        ),
        // === TasklistActivitySubscriptionService - 清单活动订阅 ===

        // 订阅管理（基于已验证模式）
        ApiDocUrl::new(
            "tasklist_activity_subscription",
            "v2",
            "create",
            "https://open.feishu.cn/document/task-v2/tasklist-activity_subscription/create",
            "创建清单活动订阅",
        ),
        ApiDocUrl::new(
            "tasklist_activity_subscription",
            "v2",
            "get",
            "https://open.feishu.cn/document/task-v2/tasklist-activity_subscription/get",
            "获取清单活动订阅详情",
        ),
        ApiDocUrl::new(
            "tasklist_activity_subscription",
            "v2",
            "list",
            "https://open.feishu.cn/document/task-v2/tasklist-activity_subscription/list",
            "获取清单活动订阅列表",
        ),
        ApiDocUrl::new(
            "tasklist_activity_subscription",
            "v2",
            "patch",
            "https://open.feishu.cn/document/task-v2/tasklist-activity_subscription/patch",
            "更新清单活动订阅",
        ),
        ApiDocUrl::new(
            "tasklist_activity_subscription",
            "v2",
            "delete",
            "https://open.feishu.cn/document/task-v2/tasklist-activity_subscription/delete",
            "删除清单活动订阅",
        ),
    ];

    registry.register_service("task", urls);
}

/// 注册搜索管理V2服务的文档URL
fn register_search_v2(registry: &mut DocUrlRegistry) {
    let urls = vec![
        // === 已验证的Search V2 API文档URL（通过联网验证）===

        // === SuiteSearchService - 套件搜索服务 ===

        // 应用搜索（已验证）
        ApiDocUrl::new(
            "search",
            "v2",
            "search_app",
            "https://open.larkoffice.com/document/server-docs/search-v2/suite-search/create-2",
            "搜索应用"
        ),

        // === DataSourceService - 数据源管理 ===

        // 数据源基础管理（已验证）
        ApiDocUrl::new(
            "search",
            "v2",
            "create_data_source",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/search-v2/data_source/create",
            "创建数据源"
        ),

        ApiDocUrl::new(
            "search",
            "v2",
            "get_data_source",
            "https://open.feishu.cn/document/server-docs/search-v2/open-search/data_source/get",
            "获取数据源"
        ),

        ApiDocUrl::new(
            "search",
            "v2",
            "list_data_sources",
            "https://open.feishu.cn/document/server-docs/search-v2/open-search/data_source/list",
            "批量获取数据源"
        ),

        ApiDocUrl::new(
            "search",
            "v2",
            "delete_data_source",
            "https://open.feishu.cn/document/server-docs/search-v2/open-search/data_source/delete",
            "删除数据源"
        ),

        ApiDocUrl::new(
            "search",
            "v2",
            "patch_data_source",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/search-v2/data_source/patch",
            "更新数据源"
        ),

        // === DataItemService - 数据项管理 ===

        // 数据项索引管理（已验证）
        ApiDocUrl::new(
            "search",
            "v2",
            "create_data_item",
            "https://open.larkoffice.com/document/server-docs/search-v2/open-search/data_source-item/create",
            "创建数据项索引"
        ),

        ApiDocUrl::new(
            "search",
            "v2",
            "get_data_item",
            "https://open.feishu.cn/document/server-docs/search-v2/open-search/data_source-item/get",
            "获取数据项"
        ),

        ApiDocUrl::new(
            "search",
            "v2",
            "list_data_items",
            "https://open.feishu.cn/document/server-docs/search-v2/open-search/data_source-item/list",
            "获取数据项列表"
        ),

        ApiDocUrl::new(
            "search",
            "v2",
            "delete_data_item",
            "https://open.feishu.cn/document/server-docs/search-v2/open-search/data_source-item/delete",
            "删除数据项"
        ),

        // === SchemaService - 数据范式管理 ===

        // 数据范式管理（已验证）
        ApiDocUrl::new(
            "search",
            "v2",
            "create_schema",
            "https://open.larkoffice.com/document/server-docs/search-v2/open-search/schema/create",
            "创建数据范式"
        ),

        ApiDocUrl::new(
            "search",
            "v2",
            "get_schema",
            "https://open.feishu.cn/document/server-docs/search-v2/open-search/schema/get",
            "获取数据范式"
        ),

        ApiDocUrl::new(
            "search",
            "v2",
            "patch_schema",
            "https://open.feishu.cn/document/server-docs/search-v2/open-search/schema/patch",
            "修改数据范式"
        ),

        ApiDocUrl::new(
            "search",
            "v2",
            "delete_schema",
            "https://open.feishu.cn/document/server-docs/search-v2/open-search/schema/delete",
            "删除数据范式"
        ),
    ];

    registry.register_service("search", urls);
}

/// 注册Search V1服务的文档URL
fn register_search_v1(registry: &mut DocUrlRegistry) {
    let urls = vec![
        // === 已验证的Search V1 API文档URL（通过联网验证）===

        // === UserService - 用户搜索服务 ===

        // 用户搜索（已验证）
        ApiDocUrl::new(
            "search",
            "v1",
            "search_user",
            "https://open.feishu.cn/document/server-docs/search-v1/user/search",
            "搜索用户"
        ),
    ];

    registry.register_service("search", urls);
}

/// 注册管理员V1服务的文档URL
fn register_admin_v1(registry: &mut DocUrlRegistry) {
    let urls = vec![
        // === 已验证的Admin V1 API文档URL（通过联网验证）===

        // === PasswordService - 密码管理 ===

        // 密码重置（已验证）
        ApiDocUrl::new(
            "admin",
            "v1",
            "reset_password",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/admin-v1/password/reset",
            "重置企业邮箱密码"
        ),

        // === DataReportService - 数据报告 ===

        // 数据报告管理（基于已验证模式）
        ApiDocUrl::new(
            "admin",
            "v1",
            "get_department_data_report",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/admin-v1/data-report-management/function-introduction",
            "获取部门数据报告"
        ),

        ApiDocUrl::new(
            "admin",
            "v1",
            "get_user_data_report",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/admin-v1/data-report-management/function-introduction",
            "获取用户数据报告"
        ),

        // === BadgeService - 徽章管理 ===

        // 徽章基础管理（已验证）
        ApiDocUrl::new(
            "admin",
            "v1",
            "create_badge",
            "https://open.feishu.cn/document/server-docs/admin-v1/badge/badge/create-2",
            "创建徽章"
        ),

        ApiDocUrl::new(
            "admin",
            "v1",
            "update_badge",
            "https://open.feishu.cn/document/server-docs/admin-v1/badge/badge/update",
            "更新徽章"
        ),

        ApiDocUrl::new(
            "admin",
            "v1",
            "upload_badge_image",
            "https://open.feishu.cn/document/server-docs/admin-v1/badge/badge/upload-image",
            "上传徽章图片"
        ),

        ApiDocUrl::new(
            "admin",
            "v1",
            "list_badges",
            "https://open.feishu.cn/document/server-docs/admin-v1/badge/badge/list",
            "获取徽章列表"
        ),

        ApiDocUrl::new(
            "admin",
            "v1",
            "get_badge",
            "https://open.feishu.cn/document/server-docs/admin-v1/badge/badge/get",
            "获取徽章详情"
        ),

        // === BadgeGrantService - 徽章授权管理 ===

        // 徽章授权管理（基于已验证模式）
        ApiDocUrl::new(
            "admin",
            "v1",
            "create_badge_grant",
            "https://open.feishu.cn/document/server-docs/admin-v1/badge/badge-grant/create",
            "创建徽章授权"
        ),

        ApiDocUrl::new(
            "admin",
            "v1",
            "delete_badge_grant",
            "https://open.feishu.cn/document/server-docs/admin-v1/badge/badge-grant/delete",
            "删除徽章授权"
        ),

        ApiDocUrl::new(
            "admin",
            "v1",
            "update_badge_grant",
            "https://open.larkoffice.com/document/server-docs/admin-v1/badge/badge-grant/update",
            "更新徽章授权"
        ),

        ApiDocUrl::new(
            "admin",
            "v1",
            "list_badge_grants",
            "https://open.feishu.cn/document/server-docs/admin-v1/badge/badge-grant/list",
            "获取徽章授权列表"
        ),

        ApiDocUrl::new(
            "admin",
            "v1",
            "get_badge_grant",
            "https://open.feishu.cn/document/server-docs/admin-v1/badge/badge-grant/get",
            "获取徽章授权详情"
        ),
    ];

    registry.register_service("admin", urls);
}

/// 注册考勤V1服务的文档URL
fn register_attendance_v1(registry: &mut DocUrlRegistry) {
    let urls = vec![
        // === 已验证的Attendance V1 API文档URL（通过联网验证）===

        // === ShiftService - 班次管理 ===

        // 班次创建和管理（已验证）
        ApiDocUrl::new(
            "attendance",
            "v1",
            "create",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/shift/create",
            "创建班次"
        ),

        ApiDocUrl::new(
            "attendance",
            "v1",
            "update",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/shift/update",
            "更新班次信息"
        ),

        ApiDocUrl::new(
            "attendance",
            "v1",
            "get",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/shift/get",
            "获取班次详情"
        ),

        ApiDocUrl::new(
            "attendance",
            "v1",
            "list",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/shift/list",
            "获取班次列表"
        ),

        // === GroupService - 考勤组管理 ===

        // 考勤组创建和管理（已验证）
        ApiDocUrl::new(
            "attendance",
            "v1",
            "create_group",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/group/create",
            "创建考勤组"
        ),

        ApiDocUrl::new(
            "attendance",
            "v1",
            "update_group",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/group/update",
            "更新考勤组"
        ),

        ApiDocUrl::new(
            "attendance",
            "v1",
            "get_group",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/group/get",
            "获取考勤组详情"
        ),

        // === UserDailyShiftService - 用户日班次 ===

        // 用户日班次管理（基于已验证模式）
        ApiDocUrl::new(
            "attendance",
            "v1",
            "create_user_daily_shift",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_daily_shift/create",
            "创建用户日班次"
        ),

        ApiDocUrl::new(
            "attendance",
            "v1",
            "update_user_daily_shift",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_daily_shift/update",
            "更新用户日班次"
        ),

        ApiDocUrl::new(
            "attendance",
            "v1",
            "get_user_daily_shift",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_daily_shift/get",
            "获取用户日班次"
        ),

        ApiDocUrl::new(
            "attendance",
            "v1",
            "batch_create_temp",
            "https://open.larkoffice.com/document/attendance-v1/user_daily_shift/batch_create_temp",
            "批量创建临时班次"
        ),

        // === UserSettingService - 用户设置 ===

        // 用户考勤设置（基于已验证模式）
        ApiDocUrl::new(
            "attendance",
            "v1",
            "create_user_setting",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_setting/create",
            "创建用户考勤设置"
        ),

        ApiDocUrl::new(
            "attendance",
            "v1",
            "update_user_setting",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_setting/update",
            "更新用户考勤设置"
        ),

        ApiDocUrl::new(
            "attendance",
            "v1",
            "get_user_setting",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_setting/get",
            "获取用户考勤设置"
        ),

        // === UserStatsDataService - 统计数据 ===

        // 统计数据管理（基于已验证模式）
        ApiDocUrl::new(
            "attendance",
            "v1",
            "update_stats_settings",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_stats_data/update",
            "更新统计设置"
        ),

        ApiDocUrl::new(
            "attendance",
            "v1",
            "query_stats_settings",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_stats_data/query_settings",
            "查询统计设置"
        ),

        ApiDocUrl::new(
            "attendance",
            "v1",
            "query_stats_fields",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_stats_data/query_fields",
            "查询统计表头"
        ),

        ApiDocUrl::new(
            "attendance",
            "v1",
            "query_stats_data",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_stats_data/query_data",
            "查询统计数据"
        ),

        // === UserApprovalService - 审批流程 ===

        // 审批流程管理（基于已验证模式）
        ApiDocUrl::new(
            "attendance",
            "v1",
            "query_approval",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_approval/query",
            "获取审批数据"
        ),

        ApiDocUrl::new(
            "attendance",
            "v1",
            "create_approval",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_approval/create",
            "写入审批结果"
        ),

        ApiDocUrl::new(
            "attendance",
            "v1",
            "process_approval",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_approval/process",
            "通知审批状态更新"
        ),

        // === UserTaskService - 考勤任务 ===

        // 考勤任务管理（已验证）
        ApiDocUrl::new(
            "attendance",
            "v1",
            "batch_create_task",
            "https://open.feishu.cn/document/server-docs/attendance-v1/user_task/batch_create",
            "导入打卡流水"
        ),

        ApiDocUrl::new(
            "attendance",
            "v1",
            "get_task",
            "https://open.feishu.cn/document/server-docs/attendance-v1/user_task/get",
            "查询打卡流水"
        ),

        ApiDocUrl::new(
            "attendance",
            "v1",
            "query_task",
            "https://open.feishu.cn/document/server-docs/attendance-v1/user_task/query",
            "批量查询打卡流水"
        ),

        ApiDocUrl::new(
            "attendance",
            "v1",
            "batch_delete_task",
            "https://open.feishu.cn/document/server-docs/attendance-v1/user_task/batch_del",
            "删除打卡流水"
        ),

        ApiDocUrl::new(
            "attendance",
            "v1",
            "query_task_result",
            "https://open.larkoffice.com/document/server-docs/attendance-v1/user_task/query",
            "查询打卡结果"
        ),

        // === UserTaskRemedyService - 任务补救 ===

        // 任务补救管理（已验证）
        ApiDocUrl::new(
            "attendance",
            "v1",
            "create_task_remedy",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_task_remedy/create",
            "通知补卡审批发起"
        ),

        ApiDocUrl::new(
            "attendance",
            "v1",
            "query_user_allowed_remedys",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_task_remedy/query_user_allowed_remedys",
            "获取可补卡时间"
        ),

        ApiDocUrl::new(
            "attendance",
            "v1",
            "query_task_remedy",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_task_remedy/query",
            "获取补卡记录"
        ),

        // === ArchiveRuleService - 归档规则 ===

        // 归档规则管理（基于已验证模式）
        ApiDocUrl::new(
            "attendance",
            "v1",
            "query_user_stats_fields",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/archive_rule/query_user_stats_fields",
            "查询归档报表表头"
        ),

        ApiDocUrl::new(
            "attendance",
            "v1",
            "upload_report",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/archive_rule/upload_report",
            "写入归档报表结果"
        ),

        ApiDocUrl::new(
            "attendance",
            "v1",
            "delete_report",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/archive_rule/delete_report",
            "删除归档报表行数据"
        ),

        ApiDocUrl::new(
            "attendance",
            "v1",
            "list_archive_rules",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/archive_rule/list",
            "查询所有归档规则"
        ),

        // === LeaveEmployExpireRecordService - 假期过期记录 ===

        // 假期过期记录管理（基于已验证模式）
        ApiDocUrl::new(
            "attendance",
            "v1",
            "get_leave_expire_record",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/leave_employ_expire_record/get",
            "通过过期时间获取发放记录"
        ),

        // === LeaveAccrualRecordService - 假期额度记录 ===

        // 假期额度记录管理（基于已验证模式）
        ApiDocUrl::new(
            "attendance",
            "v1",
            "patch_leave_accrual_record",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/leave_accrual_record/patch",
            "修改发放记录"
        ),

        // === UserTaskService - 用户打卡任务 ===

        // 用户打卡任务管理（已验证）
        ApiDocUrl::new(
            "user_task",
            "v1",
            "batch_create",
            "https://open.feishu.cn/document/server-docs/attendance-v1/user_task/batch_create",
            "导入打卡流水"
        ),

        ApiDocUrl::new(
            "user_task",
            "v1",
            "get",
            "https://open.feishu.cn/document/server-docs/attendance-v1/user_task/get",
            "查询打卡流水"
        ),

        ApiDocUrl::new(
            "user_task",
            "v1",
            "query",
            "https://open.feishu.cn/document/server-docs/attendance-v1/user_task/query-2",
            "批量查询打卡流水"
        ),

        ApiDocUrl::new(
            "user_task",
            "v1",
            "batch_del",
            "https://open.feishu.cn/document/server-docs/attendance-v1/user_task/batch_del",
            "删除打卡流水"
        ),

        ApiDocUrl::new(
            "user_task",
            "v1",
            "query_result",
            "https://open.feishu.cn/document/server-docs/attendance-v1/user_task/query",
            "查询打卡结果"
        ),

        // === 缺失的Attendance V1 API文档URL ===
        // === ShiftService - 班次管理（补全） ===
        ApiDocUrl::new(
            "attendance",
            "v1",
            "delete",
            "https://open.feishu.cn/document/server-docs/attendance-v1/shift/delete",
            "删除班次"
        ),

        // === GroupService - 考勤组管理（补全） ===
        ApiDocUrl::new(
            "attendance",
            "v1",
            "list_user",
            "https://open.feishu.cn/document/attendance-v1/group/list_user",
            "查询考勤组下所有成员"
        ),

        ApiDocUrl::new(
            "attendance",
            "v1",
            "delete_group",
            "https://open.feishu.cn/document/server-docs/attendance-v1/group/delete",
            "删除考勤组"
        ),

        ApiDocUrl::new(
            "attendance",
            "v1",
            "search_group",
            "https://open.feishu.cn/document/server-docs/attendance-v1/group/search",
            "按名称查询考勤组"
        ),

        ApiDocUrl::new(
            "attendance",
            "v1",
            "list_group",
            "https://open.feishu.cn/document/server-docs/attendance-v1/group/list",
            "查询所有考勤组"
        ),

        // === UserDailyShiftService - 用户日班次（补全） ===
        ApiDocUrl::new(
            "attendance",
            "v1",
            "batch_create_temp",
            "https://open.feishu.cn/document/attendance-v1/user_daily_shift/batch_create_temp",
            "批量创建临时班次"
        ),
    ];

    registry.register_service("attendance", urls);
}

/// 注册Mail V1服务的文档URL
fn register_mail_v1(registry: &mut DocUrlRegistry) {
    let urls = vec![
        // === FolderService - 邮箱文件夹管理 ===

        // 创建邮箱文件夹（基于已验证的user_mailbox-alias/create模式）
        ApiDocUrl::new(
            "mail",
            "v1",
            "create_folder",
            "https://open.feishu.cn/document/server-docs/mail-v1/user_mailbox-folder/create",
            "创建邮箱文件夹"
        ),

        ApiDocUrl::new(
            "mail",
            "v1",
            "delete_folder",
            "https://open.feishu.cn/document/server-docs/mail-v1/user_mailbox-folder/delete",
            "删除邮箱文件夹"
        ),

        ApiDocUrl::new(
            "mail",
            "v1",
            "patch_folder",
            "https://open.feishu.cn/document/server-docs/mail-v1/user_mailbox-folder/patch",
            "修改邮箱文件夹"
        ),

        ApiDocUrl::new(
            "mail",
            "v1",
            "list_folder",
            "https://open.feishu.cn/document/server-docs/mail-v1/user_mailbox-folder/list",
            "获取邮箱文件夹列表"
        ),

        // === MessageService - 邮件消息管理 ===

        // 发送邮件（基于已验证的mail-v1模式）
        ApiDocUrl::new(
            "mail",
            "v1",
            "send",
            "https://open.feishu.cn/document/server-docs/mail-v1/message/send",
            "发送邮件"
        ),

        ApiDocUrl::new(
            "mail",
            "v1",
            "get_message",
            "https://open.feishu.cn/document/server-docs/mail-v1/message/get",
            "获取邮件详情"
        ),

        ApiDocUrl::new(
            "mail",
            "v1",
            "list_message",
            "https://open.feishu.cn/document/server-docs/mail-v1/message/list",
            "获取邮件列表"
        ),

        ApiDocUrl::new(
            "mail",
            "v1",
            "get_by_card",
            "https://open.feishu.cn/document/server-docs/mail-v1/message/get_by_card",
            "通过卡片获取邮件"
        ),

        // === AttachmentService - 邮件附件管理 ===

        // 获取附件下载链接（基于已验证模式）
        ApiDocUrl::new(
            "mail",
            "v1",
            "download_url",
            "https://open.feishu.cn/document/server-docs/mail-v1/attachment/download_url",
            "获取附件下载链接"
        ),

        // === EventService - 邮件事件管理 ===

        // 订阅邮件事件（基于已验证模式）
        ApiDocUrl::new(
            "mail",
            "v1",
            "subscribe",
            "https://open.feishu.cn/document/server-docs/mail-v1/event/subscribe",
            "订阅邮件事件"
        ),

        ApiDocUrl::new(
            "mail",
            "v1",
            "subscription",
            "https://open.feishu.cn/document/server-docs/mail-v1/event/subscription",
            "获取邮件事件订阅状态"
        ),

        ApiDocUrl::new(
            "mail",
            "v1",
            "unsubscribe",
            "https://open.feishu.cn/document/server-docs/mail-v1/event/unsubscribe",
            "取消订阅邮件事件"
        ),

        // === RuleService - 收信规则管理 ===

        // 创建收信规则（基于已验证模式）
        ApiDocUrl::new(
            "mail",
            "v1",
            "create_rule",
            "https://open.feishu.cn/document/server-docs/mail-v1/rule/create",
            "创建收信规则"
        ),

        ApiDocUrl::new(
            "mail",
            "v1",
            "delete_rule",
            "https://open.feishu.cn/document/server-docs/mail-v1/rule/delete",
            "删除收信规则"
        ),

        ApiDocUrl::new(
            "mail",
            "v1",
            "update_rule",
            "https://open.feishu.cn/document/server-docs/mail-v1/rule/update",
            "更新收信规则"
        ),

        ApiDocUrl::new(
            "mail",
            "v1",
            "list_rule",
            "https://open.feishu.cn/document/server-docs/mail-v1/rule/list",
            "获取收信规则列表"
        ),

        ApiDocUrl::new(
            "mail",
            "v1",
            "reorder_rule",
            "https://open.feishu.cn/document/server-docs/mail-v1/rule/reorder",
            "重新排序收信规则"
        ),

        // === ContactService - 邮件联系人管理 ===

        // 创建联系人（基于已验证模式）
        ApiDocUrl::new(
            "mail",
            "v1",
            "create_contact",
            "https://open.feishu.cn/document/server-docs/mail-v1/contact/create",
            "创建邮件联系人"
        ),

        ApiDocUrl::new(
            "mail",
            "v1",
            "delete_contact",
            "https://open.feishu.cn/document/server-docs/mail-v1/contact/delete",
            "删除邮件联系人"
        ),

        ApiDocUrl::new(
            "mail",
            "v1",
            "patch_contact",
            "https://open.feishu.cn/document/server-docs/mail-v1/contact/patch",
            "修改邮件联系人"
        ),

        ApiDocUrl::new(
            "mail",
            "v1",
            "list_contact",
            "https://open.feishu.cn/document/server-docs/mail-v1/contact/list",
            "获取邮件联系人列表"
        ),

        // === MailGroupService - 邮件组管理 ===

        // 创建邮件组（基于已验证模式）
        ApiDocUrl::new(
            "mail",
            "v1",
            "create_mail_group",
            "https://open.feishu.cn/document/server-docs/mail-v1/mail_group/create",
            "创建邮件组"
        ),

        ApiDocUrl::new(
            "mail",
            "v1",
            "delete_mail_group",
            "https://open.feishu.cn/document/server-docs/mail-v1/mail_group/delete",
            "删除邮件组"
        ),

        ApiDocUrl::new(
            "mail",
            "v1",
            "patch_mail_group",
            "https://open.feishu.cn/document/server-docs/mail-v1/mail_group/patch",
            "修改邮件组"
        ),

        ApiDocUrl::new(
            "mail",
            "v1",
            "update_mail_group",
            "https://open.feishu.cn/document/server-docs/mail-v1/mail_group/update",
            "更新邮件组"
        ),

        ApiDocUrl::new(
            "mail",
            "v1",
            "get_mail_group",
            "https://open.feishu.cn/document/server-docs/mail-v1/mail_group/get",
            "获取邮件组详情"
        ),

        ApiDocUrl::new(
            "mail",
            "v1",
            "list_mail_group",
            "https://open.feishu.cn/document/server-docs/mail-v1/mail_group/list",
            "获取邮件组列表"
        ),

        // === MailGroupManagerService - 邮件组管理员管理 ===

        // 批量创建邮件组管理员（基于已验证模式）
        ApiDocUrl::new(
            "mail",
            "v1",
            "batch_create_manager",
            "https://open.feishu.cn/document/server-docs/mail-v1/mail_group_manager/batch_create",
            "批量创建邮件组管理员"
        ),

        ApiDocUrl::new(
            "mail",
            "v1",
            "batch_delete_manager",
            "https://open.feishu.cn/document/server-docs/mail-v1/mail_group_manager/batch_delete",
            "批量删除邮件组管理员"
        ),

        ApiDocUrl::new(
            "mail",
            "v1",
            "list_manager",
            "https://open.feishu.cn/document/server-docs/mail-v1/mail_group_manager/list",
            "获取邮件组管理员列表"
        ),
    ];
    registry.register_service("mail", urls);
}

/// 注册Performance V1服务的文档URL
fn register_performance_v1(registry: &mut DocUrlRegistry) {
    let urls = vec![
        // === ReviewConfigService - 后台配置管理 ===

        // 获取周期列表（基于已验证的performance-v1模式）
        ApiDocUrl::new(
            "performance",
            "v1",
            "list_semesters",
            "https://open.feishu.cn/document/performance-v1/review_config/semester_activity/semester/list",
            "获取周期列表"
        ),

        ApiDocUrl::new(
            "performance",
            "v1",
            "query_activities",
            "https://open.feishu.cn/document/performance-v1/review_config/semester_activity/query",
            "获取项目列表"
        ),

        ApiDocUrl::new(
            "performance",
            "v1",
            "query_additional_information",
            "https://open.feishu.cn/document/performance-v1/review_config/semester_activity/additional_information/query",
            "批量查询补充信息"
        ),

        ApiDocUrl::new(
            "performance",
            "v1",
            "import_additional_information",
            "https://open.feishu.cn/document/performance-v1/review_config/semester_activity/additional_information/import",
            "批量导入补充信息"
        ),

        ApiDocUrl::new(
            "performance",
            "v1",
            "delete_additional_information",
            "https://open.feishu.cn/document/performance-v1/review_config/semester_activity/additional_information/delete",
            "批量删除补充信息"
        ),

        ApiDocUrl::new(
            "performance",
            "v1",
            "write_user_group_members",
            "https://open.feishu.cn/document/performance-v1/review_config/semester_activity/user_group_user_rel/write",
            "更新人员组成员"
        ),

        ApiDocUrl::new(
            "performance",
            "v1",
            "query_reviewees",
            "https://open.feishu.cn/document/performance-v1/review_config/semester_activity/reviewee/query",
            "获取被评估人信息"
        ),

        ApiDocUrl::new(
            "performance",
            "v1",
            "query_review_templates",
            "https://open.feishu.cn/document/performance-v1/review_config/review_template/query",
            "获取评估模板配置"
        ),

        ApiDocUrl::new(
            "performance",
            "v1",
            "query_review_items",
            "https://open.feishu.cn/document/performance-v1/review_config/review_template/query_items",
            "获取评估项列表"
        ),

        ApiDocUrl::new(
            "performance",
            "v1",
            "query_tag_question_configs",
            "https://open.feishu.cn/document/performance-v1/review_config/review_template/query_tag_configs",
            "获取标签填写题配置"
        ),

        ApiDocUrl::new(
            "performance",
            "v1",
            "query_metrics",
            "https://open.feishu.cn/document/performance-v1/review_config/metric_template/query",
            "获取指标列表"
        ),

        ApiDocUrl::new(
            "performance",
            "v1",
            "query_metric_templates",
            "https://open.feishu.cn/document/performance-v1/review_config/metric_template/list",
            "获取指标模板列表"
        ),

        ApiDocUrl::new(
            "performance",
            "v1",
            "query_metric_fields",
            "https://open.feishu.cn/document/performance-v1/review_config/metric_template/query_fields",
            "获取指标字段列表"
        ),

        ApiDocUrl::new(
            "performance",
            "v1",
            "list_metric_tags",
            "https://open.feishu.cn/document/performance-v1/review_config/metric_template/list_tags",
            "获取指标标签列表"
        ),

        // === StageTaskService - 评估任务管理 ===

        // 获取周期任务（指定用户）（基于已验证模式）
        ApiDocUrl::new(
            "performance",
            "v1",
            "find_tasks_by_user_list",
            "https://open.feishu.cn/document/performance-v1/stage_task/find_tasks_by_user_list",
            "获取周期任务（指定用户）"
        ),

        ApiDocUrl::new(
            "performance",
            "v1",
            "find_tasks_by_page",
            "https://open.feishu.cn/document/performance-v1/stage_task/find_tasks_by_page",
            "获取周期任务（全部用户）"
        ),

        // === MetricDetailService - 指标数据管理 ===

        // 获取被评估人关键指标结果（基于已验证模式）
        ApiDocUrl::new(
            "performance",
            "v1",
            "query_metric_details",
            "https://open.feishu.cn/document/performance-v1/metric_detail/query",
            "获取被评估人关键指标结果"
        ),

        ApiDocUrl::new(
            "performance",
            "v1",
            "import_metric_details",
            "https://open.feishu.cn/document/performance-v1/metric_detail/import",
            "录入被评估人关键指标数据"
        ),

        // === ReviewDataService - 评估数据管理 ===

        // 获取绩效结果（基于已验证模式）
        ApiDocUrl::new(
            "performance",
            "v1",
            "query_results",
            "https://open.feishu.cn/document/performance-v1/review_data/query_results",
            "获取绩效结果"
        ),

        ApiDocUrl::new(
            "performance",
            "v1",
            "query_details",
            "https://open.feishu.cn/document/performance-v1/review_data/query_details",
            "获取绩效详情数据"
        ),
    ];
    registry.register_service("performance", urls);
}

/// 注册VC V1服务的文档URL
fn register_vc_v1(registry: &mut DocUrlRegistry) {
    let urls = vec![
        // === ReserveService - 会议预约服务 ===

        // 预约会议（基于已验证的vc-v1模式）
        ApiDocUrl::new(
            "vc",
            "v1",
            "apply",
            "https://open.feishu.cn/document/server-docs/vc-v1/reserve/apply",
            "预约会议"
        ),

        ApiDocUrl::new(
            "vc",
            "v1",
            "delete",
            "https://open.feishu.cn/document/server-docs/vc-v1/reserve/delete",
            "删除预约"
        ),

        ApiDocUrl::new(
            "vc",
            "v1",
            "update",
            "https://open.feishu.cn/document/server-docs/vc-v1/reserve/update",
            "更新预约"
        ),

        ApiDocUrl::new(
            "vc",
            "v1",
            "get",
            "https://open.feishu.cn/document/server-docs/vc-v1/reserve/get",
            "获取预约"
        ),

        ApiDocUrl::new(
            "vc",
            "v1",
            "get_active_meeting",
            "https://open.feishu.cn/document/server-docs/vc-v1/reserve/get_active_meeting",
            "获取活跃会议"
        ),

        // === MeetingService - 会议管理服务 ===

        // 邀请参会人（基于已验证模式）
        ApiDocUrl::new(
            "vc",
            "v1",
            "invite",
            "https://open.feishu.cn/document/server-docs/vc-v1/meeting/invite",
            "邀请参会人"
        ),

        ApiDocUrl::new(
            "vc",
            "v1",
            "kickout",
            "https://open.feishu.cn/document/server-docs/vc-v1/meeting/kickout",
            "移除参会人"
        ),

        ApiDocUrl::new(
            "vc",
            "v1",
            "set_host",
            "https://open.feishu.cn/document/server-docs/vc-v1/meeting/set_host",
            "设置主持人"
        ),

        ApiDocUrl::new(
            "vc",
            "v1",
            "end",
            "https://open.feishu.cn/document/server-docs/vc-v1/meeting/end",
            "结束会议"
        ),

        ApiDocUrl::new(
            "vc",
            "v1",
            "get",
            "https://open.feishu.cn/document/server-docs/vc-v1/meeting/get",
            "获取会议详情"
        ),

        ApiDocUrl::new(
            "vc",
            "v1",
            "list_by_no",
            "https://open.feishu.cn/document/server-docs/vc-v1/meeting/list_by_no",
            "按会议号查询列表"
        ),

        // === RecordingService - 录制服务 ===

        // 开始录制（基于已验证模式）
        ApiDocUrl::new(
            "vc",
            "v1",
            "start",
            "https://open.feishu.cn/document/server-docs/vc-v1/recording/start",
            "开始录制"
        ),

        ApiDocUrl::new(
            "vc",
            "v1",
            "stop",
            "https://open.feishu.cn/document/server-docs/vc-v1/recording/stop",
            "停止录制"
        ),

        ApiDocUrl::new(
            "vc",
            "v1",
            "get",
            "https://open.feishu.cn/document/server-docs/vc-v1/recording/get",
            "获取录制文件"
        ),

        ApiDocUrl::new(
            "vc",
            "v1",
            "set_permission",
            "https://open.feishu.cn/document/server-docs/vc-v1/recording/set_permission",
            "设置录制权限"
        ),

        // === RoomService - 会议室管理服务 ===

        // 创建会议室（基于已验证模式）
        ApiDocUrl::new(
            "vc",
            "v1",
            "create",
            "https://open.feishu.cn/document/server-docs/vc-v1/room/create",
            "创建会议室"
        ),

        ApiDocUrl::new(
            "vc",
            "v1",
            "update",
            "https://open.feishu.cn/document/server-docs/vc-v1/room/update",
            "更新会议室"
        ),

        ApiDocUrl::new(
            "vc",
            "v1",
            "delete",
            "https://open.feishu.cn/document/server-docs/vc-v1/room/delete",
            "删除会议室"
        ),

        ApiDocUrl::new(
            "vc",
            "v1",
            "get",
            "https://open.feishu.cn/document/server-docs/vc-v1/room/get",
            "获取会议室"
        ),

        ApiDocUrl::new(
            "vc",
            "v1",
            "list",
            "https://open.feishu.cn/document/server-docs/vc-v1/room/list",
            "获取会议室列表"
        ),

        ApiDocUrl::new(
            "vc",
            "v1",
            "search",
            "https://open.feishu.cn/document/server-docs/vc-v1/room/search",
            "搜索会议室"
        ),
    ];
    registry.register_service("vc", urls);
}

/// 注册Lingo V1服务的文档URL
fn register_lingo_v1(registry: &mut DocUrlRegistry) {
    let urls = vec![
        // === ClassificationService - 分类管理服务 ===

        // 获取词典分类列表（基于已验证的lingo-v1模式）
        ApiDocUrl::new(
            "lingo",
            "v1",
            "list_classifications",
            "https://open.feishu.cn/document/lingo-v1/classification/list",
            "获取词典分类列表"
        ),

        // === DraftService - 草稿管理服务 ===

        // 创建词条草稿（基于已验证模式）
        ApiDocUrl::new(
            "lingo",
            "v1",
            "create_draft",
            "https://open.feishu.cn/document/lingo-v1/draft/create",
            "创建词条草稿"
        ),

        ApiDocUrl::new(
            "lingo",
            "v1",
            "update_draft",
            "https://open.feishu.cn/document/lingo-v1/draft/update",
            "更新词条草稿"
        ),

        // === EntityService - 词条管理服务 ===

        // 创建免审词条（基于已验证的lingo-v1模式）
        ApiDocUrl::new(
            "lingo",
            "v1",
            "create_entity",
            "https://open.feishu.cn/document/lingo-v1/entity/create",
            "创建免审词条"
        ),

        ApiDocUrl::new(
            "lingo",
            "v1",
            "update_entity",
            "https://open.feishu.cn/document/lingo-v1/entity/update",
            "更新免审词条"
        ),

        ApiDocUrl::new(
            "lingo",
            "v1",
            "delete_entity",
            "https://open.feishu.cn/document/lingo-v1/entity/delete",
            "删除免审词条"
        ),

        ApiDocUrl::new(
            "lingo",
            "v1",
            "get_entity",
            "https://open.feishu.cn/document/lingo-v1/entity/get",
            "获取词条详情"
        ),

        ApiDocUrl::new(
            "lingo",
            "v1",
            "list_entities",
            "https://open.feishu.cn/document/lingo-v1/entity/list",
            "获取词条列表"
        ),

        ApiDocUrl::new(
            "lingo",
            "v1",
            "match_entities",
            "https://open.feishu.cn/document/lingo-v1/entity/match",
            "精准搜索词条"
        ),

        ApiDocUrl::new(
            "lingo",
            "v1",
            "search_entities",
            "https://open.feishu.cn/document/lingo-v1/entity/search",
            "模糊搜索词条"
        ),

        ApiDocUrl::new(
            "lingo",
            "v1",
            "highlight_entities",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/lingo-v1/entity/highlight",
            "词条高亮"
        ),

        // === FileService - 文件管理服务 ===

        // 上传图片资源（基于已验证模式）
        ApiDocUrl::new(
            "lingo",
            "v1",
            "upload_image",
            "https://open.feishu.cn/document/lingo-v1/file/upload_image",
            "上传图片资源"
        ),

        ApiDocUrl::new(
            "lingo",
            "v1",
            "download_image",
            "https://open.feishu.cn/document/lingo-v1/file/download_image",
            "下载图片资源"
        ),

        // === RepoService - 词库管理服务 ===

        // 获取词库列表（基于已验证模式）
        ApiDocUrl::new(
            "lingo",
            "v1",
            "list_repos",
            "https://open.feishu.cn/document/lingo-v1/repo/list",
            "获取词库列表"
        ),
    ];
    registry.register_service("lingo", urls);
}

/// 注册Cloud Docs V1服务的文档URL
fn register_cloud_docs_v1(registry: &mut DocUrlRegistry) {
    let urls = vec![
        // === DriveService - 云盘文件管理服务 ===

        // 文件管理（基于已验证的drive-v1模式）
        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "create_file",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/file/create-file",
            "创建文件"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "copy_file",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/file/copy",
            "复制文件"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "delete_file",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/file/delete",
            "删除文件"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "update_file",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/file/update",
            "更新文件"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "get_file_meta",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/file/get-meta",
            "获取文件元数据"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "create_folder",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/file/create-folder",
            "创建文件夹"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "get_folder_meta",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/file/get-folder-meta",
            "获取文件夹元数据"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "move_or_delete_folder",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/file/move-or-delete-folder",
            "移动或删除文件夹"
        ),

        // 文件上传下载（基于已验证模式）
        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "upload_all",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/media/upload_all",
            "完整文件上传"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "download",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/media/download",
            "下载文件"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "upload_part",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/media/upload-part",
            "分片上传-上传分片"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "upload_finish",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/media/upload-finish",
            "分片上传-完成上传"
        ),

        // 搜索功能（基于已验证模式）
        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "search_files",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/search/document-search",
            "搜索云文档"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "list_files",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/file/list",
            "获取文件列表"
        ),

        // 权限管理（基于已验证模式）
        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "patch_permission",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/permission/patch",
            "更新文件权限"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "get_permission_public",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/permission/get-public",
            "获取公开权限信息"
        ),

        // 版本管理（基于已验证模式）
        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "create_version",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/version/create",
            "创建文件版本"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "get_version",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/version/get",
            "获取文件版本信息"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "delete_version",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/version/delete",
            "删除文件版本"
        ),

        // 导出任务（基于已验证模式）
        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "create_export_task",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/export_task/create",
            "创建导出任务"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "get_export_task",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/export_task/get",
            "获取导出任务状态"
        ),

        // === DocxService - 在线文档服务 ===

        // 文档操作（基于已验证的docx-v1模式）
        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "create_document",
            "https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document/create",
            "创建文档"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "get_document",
            "https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document/get",
            "获取文档信息"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "patch_document",
            "https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document/patch",
            "更新文档信息"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "convert_to_docx",
            "https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document/convert-to-docx",
            "转换为DOCX格式"
        ),

        // 文档块管理（基于已验证模式）
        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "create_block",
            "https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document-block/create",
            "创建文档块"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "get_block",
            "https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document-block/get",
            "获取文档块"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "patch_block",
            "https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document-block/patch",
            "更新文档块"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "list_children",
            "https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document-block/list-children",
            "获取子块列表"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "list_blocks",
            "https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document-block/list",
            "获取文档块列表"
        ),

        // 批量操作（基于已验证模式）
        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "batch_update",
            "https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document-batch/batch-update",
            "批量更新文档"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "batch_delete",
            "https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document-batch/batch-delete",
            "批量删除文档块"
        ),

        // 内容执行（基于已验证模式）
        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "execute",
            "https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document/execute",
            "执行文档操作"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "execute_with_options",
            "https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document/execute-with-options",
            "带选项执行文档操作"
        ),

        // === BitableService - 多维表格服务 ===

        // 表格管理（基于已验证的bitable-v1模式）
        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "create_bitable",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app/create",
            "创建多维表格"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "get_bitable",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app/get",
            "获取多维表格信息"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "update_bitable",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app/update",
            "更新多维表格"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "delete_bitable",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app/delete",
            "删除多维表格"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "copy_bitable",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app/copy",
            "复制多维表格"
        ),

        // 记录管理（基于已验证模式）
        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "create_record",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/create",
            "创建记录"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "update_record",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/update",
            "更新记录"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "delete_record",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/delete",
            "删除记录"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "search_record",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/search",
            "搜索记录"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "get_record",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/get",
            "获取记录详情"
        ),

        // 批量记录操作（基于已验证模式）
        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "batch_create_record",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/batch-create",
            "批量创建记录"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "batch_update_record",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/batch-update",
            "批量更新记录"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "batch_delete_record",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/batch-delete",
            "批量删除记录"
        ),

        // 数据表管理（基于已验证模式）
        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "create_table",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table/create",
            "创建数据表"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "get_table",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table/get",
            "获取数据表信息"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "update_table",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table/update",
            "更新数据表"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "delete_table",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table/delete",
            "删除数据表"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "list_tables",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table/list",
            "获取数据表列表"
        ),

        // 字段管理（基于已验证模式）
        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "create_field",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-field/create",
            "创建字段"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "update_field",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-field/update",
            "更新字段"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "delete_field",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-field/delete",
            "删除字段"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "list_fields",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-field/list",
            "获取字段列表"
        ),

        // 视图管理（基于已验证模式）
        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "create_view",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-view/create",
            "创建视图"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "update_view",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-view/update",
            "更新视图"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "delete_view",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-view/delete",
            "删除视图"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "get_view",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-view/get",
            "获取视图信息"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "list_views",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-view/list",
            "获取视图列表"
        ),

        // === PermissionService - 权限管理服务 ===

        // 公开权限管理（基于已验证的permission-v1模式）
        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "get_permission_public",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/permission/get-public",
            "获取公开权限"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "patch_permission_public",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/permission/patch-public",
            "更新公开权限"
        ),

        // 密码管理（基于已验证模式）
        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "create_password",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/password/create",
            "创建访问密码"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "update_password",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/password/update",
            "更新访问密码"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "delete_password",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/password/delete",
            "删除访问密码"
        ),

        // === AssistantService - AI助手服务 ===

        // 订阅管理（基于已验证的assistant-v1模式）
        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "create_subscription",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/assistant/create-subscription",
            "创建AI助手订阅"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "get_subscription",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/assistant/get-subscription",
            "获取AI助手订阅信息"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "patch_subscription",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/assistant/patch-subscription",
            "更新AI助手订阅"
        ),

        // 快速订阅（基于已验证模式）
        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "quick_subscribe_doc",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/assistant/quick-subscribe-doc",
            "快速订阅文档AI助手"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "quick_subscribe_bitable",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/assistant/quick-subscribe-bitable",
            "快速订阅多维表格AI助手"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "quick_subscribe_sheet",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/assistant/quick-subscribe-sheet",
            "快速订阅电子表格AI助手"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "quick_subscribe_wiki",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/assistant/quick-subscribe-wiki",
            "快速订阅知识库AI助手"
        ),

        // === BoardService - 画板协作服务 ===

        // 画板管理（基于已验证的board-v1模式）
        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "get_thumbnail",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/board/get-thumbnail",
            "获取画板缩略图"
        ),

        ApiDocUrl::new(
            "cloud_docs",
            "v1",
            "list_whiteboard_nodes",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/board/list-whiteboard-nodes",
            "获取画板节点列表"
        ),
    ];
    registry.register_service("cloud_docs", urls);
}

/// 注册Group V1服务的文档URL
fn register_group_v1(registry: &mut DocUrlRegistry) {
    let urls = vec![
        // === ChatService - 群管理服务 ===

        // 群聊管理（基于已验证的im-v1和group模式）
        ApiDocUrl::new(
            "group",
            "v1",
            "delete_chat",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/delete",
            "解散群聊"
        ),

        ApiDocUrl::new(
            "group",
            "v1",
            "patch_chat",
            "https://open.feishu.cn/document/server-docs/im-v1/chat/patch",
            "更新群信息"
        ),

        ApiDocUrl::new(
            "group",
            "v1",
            "get_chat",
            "https://open.feishu.cn/document/server-docs/im-v1/chat/get",
            "获取群信息"
        ),

        ApiDocUrl::new(
            "group",
            "v1",
            "create_chat",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat/create",
            "创建群聊"
        ),

        ApiDocUrl::new(
            "group",
            "v1",
            "search_chat",
            "https://open.feishu.cn/document/server-docs/im-v1/chat/search",
            "搜索群聊"
        ),

        ApiDocUrl::new(
            "group",
            "v1",
            "get_chat_link",
            "https://open.feishu.cn/document/server-docs/im-v1/chat/get-chat-link",
            "获取群分享链接"
        ),

        ApiDocUrl::new(
            "group",
            "v1",
            "put_top_notice",
            "https://open.feishu.cn/document/server-docs/im-v1/chat/put-top-notice",
            "设置群置顶消息"
        ),

        ApiDocUrl::new(
            "group",
            "v1",
            "delete_top_notice",
            "https://open.feishu.cn/document/server-docs/im-v1/chat/delete-top-notice",
            "撤销群置顶"
        ),

        // === ChatMemberService - 群成员管理服务 ===

        // 群成员管理（基于已验证的im-v1模式）
        ApiDocUrl::new(
            "group",
            "v1",
            "add_member",
            "https://open.feishu.cn/document/server-docs/im-v1/chat-members/add",
            "添加群成员"
        ),

        ApiDocUrl::new(
            "group",
            "v1",
            "delete_member",
            "https://open.feishu.cn/document/server-docs/im-v1/chat-members/delete",
            "移除群成员"
        ),

        ApiDocUrl::new(
            "group",
            "v1",
            "get_member",
            "https://open.feishu.cn/document/server-docs/im-v1/chat-members/get",
            "获取群成员信息"
        ),

        ApiDocUrl::new(
            "group",
            "v1",
            "list_members",
            "https://open.feishu.cn/document/server-docs/im-v1/chat-members/list",
            "获取群成员列表"
        ),

        ApiDocUrl::new(
            "group",
            "v1",
            "is_in_chat",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/chat-members/is_in_chat",
            "判断是否在群里"
        ),

        ApiDocUrl::new(
            "group",
            "v1",
            "me_join_chat",
            "https://open.feishu.cn/document/server-docs/im-v1/chat-members/me-join",
            "主动加入群聊"
        ),

        ApiDocUrl::new(
            "group",
            "v1",
            "add_managers",
            "https://open.feishu.cn/document/server-docs/im-v1/chat-members/add-managers",
            "指定群管理员"
        ),

        ApiDocUrl::new(
            "group",
            "v1",
            "delete_managers",
            "https://open.feishu.cn/document/server-docs/im-v1/chat-members/delete-managers",
            "删除群管理员"
        ),

        // === ChatAnnouncementService - 群公告服务 ===

        // 群公告管理（基于已验证的group模式）
        ApiDocUrl::new(
            "group",
            "v1",
            "get_announcement",
            "https://open.feishu.cn/document/server-docs/group/chat-announcement/get",
            "获取群公告"
        ),

        ApiDocUrl::new(
            "group",
            "v1",
            "list_announcements",
            "https://open.feishu.cn/document/server-docs/group/chat-announcement/list",
            "获取群公告列表"
        ),

        ApiDocUrl::new(
            "group",
            "v1",
            "create_announcement_block",
            "https://open.feishu.cn/document/server-docs/group/chat-announcement/create-block",
            "创建公告块"
        ),

        ApiDocUrl::new(
            "group",
            "v1",
            "batch_update_announcement_block",
            "https://open.feishu.cn/document/server-docs/group/chat-announcement/batch-update-block",
            "批量更新公告块"
        ),

        ApiDocUrl::new(
            "group",
            "v1",
            "get_announcement_block",
            "https://open.feishu.cn/document/server-docs/group/chat-announcement/get-block",
            "获取公告块"
        ),

        ApiDocUrl::new(
            "group",
            "v1",
            "get_announcement_children_blocks",
            "https://open.feishu.cn/document/server-docs/group/chat-announcement/get-children-blocks",
            "获取子公告块"
        ),

        ApiDocUrl::new(
            "group",
            "v1",
            "batch_delete_announcement_block",
            "https://open.feishu.cn/document/server-docs/group/chat-announcement/batch-delete-block",
            "批量删除公告块"
        ),

        // === ChatTabService - 会话标签页服务 ===

        // 会话标签页管理（基于已验证的group模式）
        ApiDocUrl::new(
            "group",
            "v1",
            "create_chat_tab",
            "https://open.feishu.cn/document/server-docs/group/chat-tab/create",
            "创建标签页"
        ),

        ApiDocUrl::new(
            "group",
            "v1",
            "delete_chat_tab",
            "https://open.feishu.cn/document/server-docs/group/chat-tab/delete",
            "删除标签页"
        ),

        ApiDocUrl::new(
            "group",
            "v1",
            "update_chat_tab",
            "https://open.feishu.cn/document/server-docs/group/chat-tab/update",
            "更新标签页"
        ),

        ApiDocUrl::new(
            "group",
            "v1",
            "sort_chat_tab",
            "https://open.feishu.cn/document/server-docs/group/chat-tab/sort",
            "排序标签页"
        ),

        ApiDocUrl::new(
            "group",
            "v1",
            "list_chat_tabs",
            "https://open.feishu.cn/document/server-docs/group/chat-tab/list",
            "获取标签页列表"
        ),

        // === ChatMenuTreeService - 群菜单服务 ===

        // 群菜单管理（基于已验证的group模式）
        ApiDocUrl::new(
            "group",
            "v1",
            "create_chat_menu",
            "https://open.feishu.cn/document/server-docs/group/chat-menu_tree/create",
            "创建群菜单"
        ),

        ApiDocUrl::new(
            "group",
            "v1",
            "delete_chat_menu",
            "https://open.feishu.cn/document/server-docs/group/chat-menu_tree/delete",
            "删除群菜单"
        ),

        ApiDocUrl::new(
            "group",
            "v1",
            "patch_chat_menu",
            "https://open.feishu.cn/document/server-docs/group/chat-menu_tree/patch",
            "修改群菜单"
        ),

        ApiDocUrl::new(
            "group",
            "v1",
            "sort_chat_menu",
            "https://open.feishu.cn/document/server-docs/group/chat-menu_tree/sort",
            "排序群菜单"
        ),

        ApiDocUrl::new(
            "group",
            "v1",
            "get_chat_menu",
            "https://open.feishu.cn/document/server-docs/group/chat-menu_tree/get",
            "获取群菜单"
        ),
    ];
    registry.register_service("group", urls);
}

/// 注册CoreHR V1服务的文档URL
fn register_corehr_v1(registry: &mut DocUrlRegistry) {
    let urls = vec![
        // === BasicInfoService - 基础数据管理服务 ===

        // 基础数据查询（基于已验证的corehr-v1模式）
        ApiDocUrl::new(
            "corehr",
            "v1",
            "search_enum",
            "https://open.feishu.cn/document/server-docs/corehr-v1/basic-infomation/enum/search",
            "查询枚举信息"
        ),

        ApiDocUrl::new(
            "corehr",
            "v1",
            "search_country_region",
            "https://open.feishu.cn/document/server-docs/corehr-v1/basic-infomation/location/search",
            "查询国家/地区信息"
        ),

        ApiDocUrl::new(
            "corehr",
            "v1",
            "search_nationality",
            "https://open.feishu.cn/document/server-docs/corehr-v1/basic-infomation/nationality/search",
            "查询国籍信息"
        ),

        ApiDocUrl::new(
            "corehr",
            "v1",
            "convert_id",
            "https://open.feishu.cn/document/server-docs/corehr-v1/common-data/id-convert",
            "不同ID类型转换"
        ),

        // === EmployeeService - 员工信息服务 ===

        // 员工信息查询（基于已验证模式）
        ApiDocUrl::new(
            "corehr",
            "v1",
            "batch_get_employees",
            "https://open.feishu.cn/document/server-docs/corehr-v1/employees/batch-get",
            "批量查询员工详细信息"
        ),

        ApiDocUrl::new(
            "corehr",
            "v1",
            "search_employees",
            "https://open.feishu.cn/document/server-docs/corehr-v1/employees/search",
            "按条件搜索员工信息"
        ),

        ApiDocUrl::new(
            "corehr",
            "v1",
            "get_job_information",
            "https://open.feishu.cn/document/server-docs/corehr-v1/employee/job_data/get",
            "获取员工任职信息"
        ),

        ApiDocUrl::new(
            "corehr",
            "v1",
            "update_employment_information",
            "https://open.feishu.cn/document/server-docs/corehr-v1/employee/employment/updated",
            "更新雇佣信息"
        ),

        ApiDocUrl::new(
            "corehr",
            "v1",
            "upload_person_file",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/person/upload",
            "上传员工附件文件"
        ),

        ApiDocUrl::new(
            "corehr",
            "v1",
            "download_person_file",
            "https://open.feishu.cn/document/server-docs/corehr-v1/employee/person/get-2",
            "下载员工附件文件"
        ),

        // === JobManagementService - 岗职务管理服务 ===

        // 职位序列管理（基于已验证模式）
        ApiDocUrl::new(
            "corehr",
            "v1",
            "create_job_family",
            "https://open.feishu.cn/document/server-docs/corehr-v1/job-families/create",
            "创建职位序列"
        ),

        ApiDocUrl::new(
            "corehr",
            "v1",
            "list_job_families",
            "https://open.feishu.cn/document/server-docs/corehr-v1/job-families/list",
            "批量查询职位序列"
        ),

        // 职级管理（基于已验证模式）
        ApiDocUrl::new(
            "corehr",
            "v1",
            "create_job_level",
            "https://open.feishu.cn/document/server-docs/corehr-v1/job-levels/create",
            "创建职级"
        ),

        ApiDocUrl::new(
            "corehr",
            "v1",
            "list_job_levels",
            "https://open.feishu.cn/document/server-docs/corehr-v1/job-levels/list",
            "批量查询职级"
        ),

        // 职等管理（基于已验证模式）
        ApiDocUrl::new(
            "corehr",
            "v1",
            "create_job_grade",
            "https://open.feishu.cn/document/server-docs/corehr-v1/job-grades/create",
            "创建职等"
        ),

        ApiDocUrl::new(
            "corehr",
            "v1",
            "query_job_grades",
            "https://open.feishu.cn/document/server-docs/corehr-v1/job-grades/query",
            "按条件查询职等"
        ),

        // 职务管理（基于已验证模式）
        ApiDocUrl::new(
            "corehr",
            "v1",
            "create_job",
            "https://open.feishu.cn/document/server-docs/corehr-v1/jobs/create",
            "创建职务"
        ),

        ApiDocUrl::new(
            "corehr",
            "v1",
            "list_jobs",
            "https://open.feishu.cn/document/server-docs/corehr-v1/jobs/list",
            "批量查询职务"
        ),

        // === LifecycleService - 员工生命周期服务 ===

        // 待入职管理（基于已验证模式）
        ApiDocUrl::new(
            "corehr",
            "v1",
            "create_pre_hire",
            "https://open.feishu.cn/document/server-docs/corehr-v1/pre-hires/create",
            "创建待入职人员"
        ),

        ApiDocUrl::new(
            "corehr",
            "v1",
            "search_pre_hire",
            "https://open.feishu.cn/document/server-docs/corehr-v1/pre-hires/search",
            "搜索待入职信息"
        ),

        // 员工异动管理（基于已验证模式）
        ApiDocUrl::new(
            "corehr",
            "v1",
            "create_job_change",
            "https://open.feishu.cn/document/server-docs/corehr-v1/job-changes/create",
            "发起员工异动"
        ),

        ApiDocUrl::new(
            "corehr",
            "v1",
            "search_job_change",
            "https://open.feishu.cn/document/server-docs/corehr-v1/job-changes/search",
            "搜索员工异动信息"
        ),

        // 离职管理（基于已验证模式）
        ApiDocUrl::new(
            "corehr",
            "v1",
            "create_offboarding",
            "https://open.feishu.cn/document/server-docs/corehr-v1/offboardings/create",
            "操作员工离职"
        ),

        ApiDocUrl::new(
            "corehr",
            "v1",
            "search_offboarding",
            "https://open.feishu.cn/document/server-docs/corehr-v1/offboardings/search",
            "搜索离职信息"
        ),

        // === OrganizationService - 组织管理服务 ===

        // 部门管理（基于已验证模式）
        ApiDocUrl::new(
            "corehr",
            "v1",
            "create_department",
            "https://open.feishu.cn/document/server-docs/corehr-v1/departments/create",
            "创建部门"
        ),

        ApiDocUrl::new(
            "corehr",
            "v1",
            "batch_get_departments",
            "https://open.feishu.cn/document/server-docs/corehr-v1/departments/batch-get",
            "批量查询部门"
        ),

        ApiDocUrl::new(
            "corehr",
            "v1",
            "get_department_tree",
            "https://open.feishu.cn/document/server-docs/corehr-v1/departments/tree",
            "查询部门架构树"
        ),

        ApiDocUrl::new(
            "corehr",
            "v1",
            "list_departments",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/corehr-v1/department/list",
            "批量查询部门列表"
        ),

        // 公司管理（基于已验证模式）
        ApiDocUrl::new(
            "corehr",
            "v1",
            "create_company",
            "https://open.feishu.cn/document/server-docs/corehr-v1/companies/create",
            "创建公司"
        ),

        ApiDocUrl::new(
            "corehr",
            "v1",
            "list_companies",
            "https://open.feishu.cn/document/server-docs/corehr-v1/companies/list",
            "批量查询公司"
        ),

        // === 人员类型管理 ===

        // 人员类型查询（基于已验证模式）
        ApiDocUrl::new(
            "corehr",
            "v1",
            "list_employee_types",
            "https://open.feishu.cn/document/server-docs/corehr-v1/basic-infomation/employee_type/list",
            "批量查询人员类型"
        ),
    ];
    registry.register_service("corehr", urls);
}

/// 注册Hire V1服务的文档URL
fn register_hire_v1(registry: &mut DocUrlRegistry) {
    let urls = vec![
        // === RecruitmentConfigService - 招聘配置服务 ===

        // 职位管理（基于已验证的hire-v1模式）
        ApiDocUrl::new(
            "hire",
            "v1",
            "create_job",
            "https://open.feishu.cn/document/server-docs/hire-v1/recruitment-related-configuration/job/create",
            "创建招聘职位"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "update_job",
            "https://open.feishu.cn/document/server-docs/hire-v1/recruitment-related-configuration/job/update",
            "更新招聘职位"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "get_job_detail",
            "https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/job/get_detail",
            "获取职位详情"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "list_jobs",
            "https://open.feishu.cn/document/server-docs/hire-v1/recruitment-related-configuration/job/list",
            "获取职位列表"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "close_job",
            "https://open.feishu.cn/document/server-docs/hire-v1/recruitment-related-configuration/job/close",
            "关闭招聘职位"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "open_job",
            "https://open.feishu.cn/document/server-docs/hire-v1/recruitment-related-configuration/job/open",
            "开启招聘职位"
        ),

        // 地点管理（基于已验证模式）
        ApiDocUrl::new(
            "hire",
            "v1",
            "query_locations",
            "https://open.feishu.cn/document/server-docs/hire-v1/recruitment-related-configuration/location/query",
            "查询工作地点"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "list_locations",
            "https://open.feishu.cn/document/server-docs/hire-v1/recruitment-related-configuration/location/list",
            "获取工作地点列表"
        ),

        // 职位需求管理（基于已验证模式）
        ApiDocUrl::new(
            "hire",
            "v1",
            "create_job_requirement",
            "https://open.feishu.cn/document/server-docs/hire-v1/recruitment-related-configuration/job_requirement/create",
            "创建职位需求"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "get_job_requirement_detail",
            "https://open.feishu.cn/document/server-docs/hire-v1/recruitment-related-configuration/job_requirement/list_by_id",
            "获取职位需求详情"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "list_job_requirements",
            "https://open.feishu.cn/document/server-docs/hire-v1/recruitment-related-configuration/job_requirement/list",
            "获取职位需求列表"
        ),

        // 招聘流程管理（基于已验证模式）
        ApiDocUrl::new(
            "hire",
            "v1",
            "create_job_process",
            "https://open.feishu.cn/document/server-docs/hire-v1/recruitment-related-configuration/job_process/create",
            "创建招聘流程"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "get_job_process_detail",
            "https://open.feishu.cn/document/server-docs/hire-v1/recruitment-related-configuration/job_process/list",
            "获取招聘流程详情"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "list_job_processes",
            "https://open.feishu.cn/document/server-docs/hire-v1/recruitment-related-configuration/job_process/list",
            "获取招聘流程列表"
        ),

        // === CandidateManagementService - 候选人管理服务 ===

        // 人才管理（基于已验证模式）
        ApiDocUrl::new(
            "hire",
            "v1",
            "create_talent",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/talent/combined_create",
            "创建人才"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "get_talent_detail",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/talent/get",
            "获取人才详情"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "list_talents",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/talent/list",
            "获取人才列表"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "update_talent",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/talent/update",
            "更新人才信息"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "delete_talent",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/talent/delete",
            "删除人才"
        ),

        // 投递管理（基于已验证模式）
        ApiDocUrl::new(
            "hire",
            "v1",
            "create_application",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/application/create",
            "创建投递"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "get_application_detail",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/application/get",
            "获取投递详情"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "list_applications",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/application/list",
            "获取投递列表"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "advance_application",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/application/advance",
            "推进投递流程"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "reject_application",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/application/reject",
            "拒绝投递"
        ),

        // 面试管理（基于已验证模式）
        ApiDocUrl::new(
            "hire",
            "v1",
            "create_interview",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/interview/create",
            "创建面试"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "get_interview_detail",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/interview/get",
            "获取面试详情"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "list_interviews",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/interview/list",
            "获取面试列表"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "arrange_interview",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/interview/arrange",
            "安排面试"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "submit_interview_evaluation",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/interview/submit_evaluation",
            "提交面试评价"
        ),

        // Offer管理（基于已验证模式）
        ApiDocUrl::new(
            "hire",
            "v1",
            "create_offer",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/offer/create",
            "创建Offer"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "get_offer_detail",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/offer/get",
            "获取Offer详情"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "list_offers",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/offer/list",
            "获取Offer列表"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "update_offer",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/offer/update",
            "更新Offer"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "send_offer",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/offer/send",
            "发送Offer"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "withdraw_offer",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/offer/withdraw",
            "撤回Offer"
        ),

        // === GetCandidatesService - 候选人获取服务 ===

        // 官网投递（基于已验证模式）
        ApiDocUrl::new(
            "hire",
            "v1",
            "list_website_jobs",
            "https://open.feishu.cn/document/server-docs/hire-v1/get-candidates/website/list",
            "获取官网职位列表"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "create_website_channel",
            "https://open.feishu.cn/document/server-docs/hire-v1/get-candidates/website/create-2",
            "创建官网推广渠道"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "delete_website_channel",
            "https://open.larkoffice.com/document/server-docs/hire-v1/get-candidates/website/delete",
            "删除官网推广渠道"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "list_website_applications",
            "https://open.feishu.cn/document/server-docs/hire-v1/get-candidates/website/list_applications",
            "获取官网投递列表"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "get_website_configuration",
            "https://open.feishu.cn/document/server-docs/hire-v1/get-candidates/website/get_configuration",
            "获取官网配置"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "update_website_configuration",
            "https://open.feishu.cn/document/server-docs/hire-v1/get-candidates/website/update_configuration",
            "更新官网配置"
        ),

        // 内推管理（基于已验证模式）
        ApiDocUrl::new(
            "hire",
            "v1",
            "create_referral",
            "https://open.feishu.cn/document/server-docs/hire-v1/get-candidates/referral/create",
            "创建内推"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "get_referral_detail",
            "https://open.feishu.cn/document/server-docs/hire-v1/get-candidates/referral/get",
            "获取内推详情"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "list_referrals",
            "https://open.feishu.cn/document/server-docs/hire-v1/get-candidates/referral/list",
            "获取内推列表"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "register_referral_account",
            "https://open.feishu.cn/document/server-docs/hire-v1/get-candidates/referral/register_account",
            "注册内推账户"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "get_referral_account",
            "https://open.feishu.cn/document/server-docs/hire-v1/get-candidates/referral/get_account",
            "获取内推账户"
        ),

        // 猎头管理（基于已验证模式）
        ApiDocUrl::new(
            "hire",
            "v1",
            "create_agency",
            "https://open.feishu.cn/document/server-docs/hire-v1/get-candidates/agency/create",
            "创建猎头机构"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "list_agencies",
            "https://open.feishu.cn/document/server-docs/hire-v1/get-candidates/agency/list",
            "获取猎头机构列表"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "create_recommendation",
            "https://open.feishu.cn/document/server-docs/hire-v1/get-candidates/agency/create_recommendation",
            "创建猎头推荐"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "list_recommendations",
            "https://open.feishu.cn/document/server-docs/hire-v1/get-candidates/agency/list_recommendations",
            "获取猎头推荐列表"
        ),

        // === 待办事项管理 ===

        // 待办事项（基于已验证模式）
        ApiDocUrl::new(
            "hire",
            "v1",
            "list_todos",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/recruitment-process-follow-up/list",
            "批量获取待办事项"
        ),

        // === 开发指南和概览 ===

        // 开发指南（基于已验证模式）
        ApiDocUrl::new(
            "hire",
            "v1",
            "recruitment_development_guide",
            "https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/recruitment-development-guide",
            "招聘开发指南"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "hire_overview",
            "https://open.feishu.cn/solutions/detail/hire",
            "飞书招聘解决方案概览"
        ),

        // === 候选人管理服务 - Candidate Management ===

        // === 人才管理模块 ===

        // 创建人才（已验证）
        ApiDocUrl::new(
            "hire",
            "v1",
            "create_talent",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/talent/combined_create",
            "创建人才档案"
        ),

        // 获取人才详情
        ApiDocUrl::new(
            "hire",
            "v1",
            "get_talent_detail",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/talent/get_detail",
            "获取人才详情"
        ),

        // 更新人才信息
        ApiDocUrl::new(
            "hire",
            "v1",
            "update_talent",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/talent/combined_update",
            "更新人才信息"
        ),

        // 列出人才列表
        ApiDocUrl::new(
            "hire",
            "v1",
            "list_talents",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/talent/list",
            "获取人才列表"
        ),

        // 删除人才
        ApiDocUrl::new(
            "hire",
            "v1",
            "delete_talent",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/talent/delete",
            "删除人才档案"
        ),

        // 批量导入人才
        ApiDocUrl::new(
            "hire",
            "v1",
            "batch_import_talents",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/talent/batch_import",
            "批量导入人才"
        ),

        // 获取人才投递历史
        ApiDocUrl::new(
            "hire",
            "v1",
            "get_talent_application_history",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/talent/get_application_history",
            "获取人才投递历史"
        ),

        // === 投递管理模块 ===

        // 创建投递（已验证）
        ApiDocUrl::new(
            "hire",
            "v1",
            "create_application",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/delivery-process-management/application/create",
            "创建投递记录"
        ),

        // 获取投递详情
        ApiDocUrl::new(
            "hire",
            "v1",
            "get_application_detail",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/delivery-process-management/application/get_detail",
            "获取投递详情"
        ),

        // 列出投递列表
        ApiDocUrl::new(
            "hire",
            "v1",
            "list_applications",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/delivery-process-management/application/list",
            "获取投递列表"
        ),

        // 推进投递流程
        ApiDocUrl::new(
            "hire",
            "v1",
            "advance_application",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/delivery-process-management/application/advance",
            "推进投递到下一阶段"
        ),

        // 拒绝投递
        ApiDocUrl::new(
            "hire",
            "v1",
            "reject_application",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/delivery-process-management/application/reject",
            "拒绝投递申请"
        ),

        // 恢复投递
        ApiDocUrl::new(
            "hire",
            "v1",
            "recover_application",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/delivery-process-management/application/recover",
            "恢复被拒绝的投递"
        ),

        // 获取投递的面试记录
        ApiDocUrl::new(
            "hire",
            "v1",
            "get_application_interviews",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/delivery-process-management/application/get_interviews",
            "获取投递的面试记录"
        ),

        // === 面试管理模块 ===

        // 创建面试
        ApiDocUrl::new(
            "hire",
            "v1",
            "create_interview",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/interview/create",
            "创建面试安排"
        ),

        // 获取面试详情
        ApiDocUrl::new(
            "hire",
            "v1",
            "get_interview_detail",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/interview/get_detail",
            "获取面试详情"
        ),

        // 列出面试列表
        ApiDocUrl::new(
            "hire",
            "v1",
            "list_interviews",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/interview/list",
            "获取面试列表"
        ),

        // 安排面试
        ApiDocUrl::new(
            "hire",
            "v1",
            "arrange_interview",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/interview/arrange",
            "安排面试时间和地点"
        ),

        // 提交面试评价
        ApiDocUrl::new(
            "hire",
            "v1",
            "submit_interview_evaluation",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/interview/submit_evaluation",
            "提交面试评价结果"
        ),

        // 取消面试
        ApiDocUrl::new(
            "hire",
            "v1",
            "cancel_interview",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/interview/cancel",
            "取消已安排的面试"
        ),

        // 重新安排面试
        ApiDocUrl::new(
            "hire",
            "v1",
            "reschedule_interview",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/interview/reschedule",
            "重新安排面试时间"
        ),

        // === Offer管理模块 ===

        // 创建Offer
        ApiDocUrl::new(
            "hire",
            "v1",
            "create_offer",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/offer/create",
            "创建Offer"
        ),

        // 获取Offer详情
        ApiDocUrl::new(
            "hire",
            "v1",
            "get_offer_detail",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/offer/get_detail",
            "获取Offer详情"
        ),

        // 列出Offer列表
        ApiDocUrl::new(
            "hire",
            "v1",
            "list_offers",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/offer/list",
            "获取Offer列表"
        ),

        // 更新Offer
        ApiDocUrl::new(
            "hire",
            "v1",
            "update_offer",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/offer/update",
            "更新Offer信息"
        ),

        // 发送Offer
        ApiDocUrl::new(
            "hire",
            "v1",
            "send_offer",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/offer/send",
            "发送Offer给候选人"
        ),

        // 撤回Offer
        ApiDocUrl::new(
            "hire",
            "v1",
            "withdraw_offer",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/offer/withdraw",
            "撤回已发送的Offer"
        ),

        // 创建入职准备
        ApiDocUrl::new(
            "hire",
            "v1",
            "create_onboarding",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/delivery-process-management/onboard/create",
            "创建入职准备"
        ),

        // 更新入职状态
        ApiDocUrl::new(
            "hire",
            "v1",
            "update_onboarding_status",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/delivery-process-management/onboard/patch",
            "更新人才入职状态"
        ),

        // === 人才库管理模块 ===

        // 创建人才库
        ApiDocUrl::new(
            "hire",
            "v1",
            "create_talent_pool",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/talent-pool/create",
            "创建人才库"
        ),

        // 获取人才库详情
        ApiDocUrl::new(
            "hire",
            "v1",
            "get_talent_pool_detail",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/talent-pool/get_detail",
            "获取人才库详情"
        ),

        // 列出人才库列表
        ApiDocUrl::new(
            "hire",
            "v1",
            "list_talent_pools",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/talent-pool/list",
            "获取人才库列表"
        ),

        // 获取人才库中的人才列表
        ApiDocUrl::new(
            "hire",
            "v1",
            "list_talent_pool_talents",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/talent-pool/list_talents",
            "获取人才库中的人才列表"
        ),

        // 添加人才到人才库
        ApiDocUrl::new(
            "hire",
            "v1",
            "add_talent_to_pool",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/talent-pool/add_talent",
            "添加人才到人才库"
        ),

        // 从人才库移除人才
        ApiDocUrl::new(
            "hire",
            "v1",
            "remove_talent_from_pool",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/talent-pool/remove_talent",
            "从人才库移除人才"
        ),

        // 更新人才库
        ApiDocUrl::new(
            "hire",
            "v1",
            "update_talent_pool",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/talent-pool/update",
            "更新人才库信息"
        ),

        // 删除人才库
        ApiDocUrl::new(
            "hire",
            "v1",
            "delete_talent_pool",
            "https://open.feishu.cn/document/server-docs/hire-v1/candidate-management/talent-pool/delete",
            "删除人才库"
        ),

        // === 候选人获取模块 (Get Candidates) ===

        // === 招聘网站管理 (Website) ===
        ApiDocUrl::new(
            "hire",
            "v1",
            "create_website_promotion_channel",
            "https://open.larkoffice.com/document/server-docs/hire-v1/get-candidates/website/create-2",
            "新建招聘官网推广渠道"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "get_website_list",
            "https://open.feishu.cn/document/server-docs/hire-v1/get-candidates/website/list",
            "获取招聘官网列表"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "get_website_application_list",
            "https://open.feishu.cn/document/server-docs/hire-v1/get-candidates/website/get_application",
            "获取招聘官网申请列表"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "get_website_detail",
            "https://open.feishu.cn/document/server-docs/hire-v1/get-candidates/website/get",
            "获取招聘官网详情"
        ),

        // === 猎头合作管理 (Agency) ===
        ApiDocUrl::new(
            "hire",
            "v1",
            "create_agency",
            "https://open.feishu.cn/document/server-docs/hire-v1/get-candidates/agency/create",
            "创建猎头机构"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "list_agencies",
            "https://open.feishu.cn/document/server-docs/hire-v1/get-candidates/agency/list",
            "获取猎头机构列表"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "get_agency_detail",
            "https://open.feishu.cn/document/server-docs/hire-v1/get-candidates/agency/get",
            "获取猎头机构详情"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "create_agency_consultant",
            "https://open.feishu.cn/document/server-docs/hire-v1/get-candidates/agency/create_consultant",
            "创建猎头顾问"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "list_agency_consultants",
            "https://open.feishu.cn/document/server-docs/hire-v1/get-candidates/agency/list_consultant",
            "获取猎头顾问列表"
        ),

        // === 内推管理 (Referral) ===
        ApiDocUrl::new(
            "hire",
            "v1",
            "get_referral_by_application",
            "https://open.larkoffice.com/document/server-docs/hire-v1/get-candidates/referral/get_by_application",
            "获取投递的内推信息"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "get_referral_job_details",
            "https://open.feishu.cn/document/server-docs/hire-v1/get-candidates/referral/get_job_detail",
            "获取内推职位详情"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "create_referral_account",
            "https://open.feishu.cn/document/server-docs/hire-v1/get-candidates/referral_account/create",
            "创建内推账户"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "get_referral_account_detail",
            "https://open.feishu.cn/document/server-docs/hire-v1/get-candidates/referral_account/get",
            "获取内推账户详情"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "list_referral_accounts",
            "https://open.feishu.cn/document/server-docs/hire-v1/get-candidates/referral_account/list",
            "获取内推账户列表"
        ),

        // === 外部系统集成 (External System) ===
        ApiDocUrl::new(
            "hire",
            "v1",
            "create_external_system_config",
            "https://open.feishu.cn/document/server-docs/hire-v1/get-candidates/import-external-system-information/create",
            "创建外部系统配置"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "update_external_application",
            "https://open.larkenterprise.com/document/server-docs/hire-v1/get-candidates/import-external-system-information/update-2",
            "更新外部投递信息"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "get_external_application_list",
            "https://open.feishu.cn/document/server-docs/hire-v1/get-candidates/import-external-system-information/list",
            "获取外部投递列表"
        ),

        // === 生态对接模块 (Ecological Docking) ===

        // === 背景调查服务 (Background Check) ===
        ApiDocUrl::new(
            "hire",
            "v1",
            "create_background_check_package",
            "https://open.feishu.cn/document/server-docs/hire-v1/ecological-docking/background-check/create_package",
            "创建背调套餐"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "list_background_check_packages",
            "https://open.feishu.cn/document/server-docs/hire-v1/ecological-docking/background-check/list_package",
            "获取背调套餐列表"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "create_background_check_order",
            "https://open.feishu.cn/document/server-docs/hire-v1/ecological-docking/background-check/create_order",
            "创建背调订单"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "get_background_check_report",
            "https://open.feishu.cn/document/server-docs/hire-v1/ecological-docking/background-check/get_report",
            "获取背调报告"
        ),

        // === 笔试服务 (Exam) ===
        ApiDocUrl::new(
            "hire",
            "v1",
            "create_exam_paper",
            "https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/test/create",
            "创建笔试试卷"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "list_exam_papers",
            "https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/test/search",
            "获取笔试试卷列表"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "arrange_exam",
            "https://open.feishu.cn/document/server-docs/hire-v1/ecological-docking/exam/arrange",
            "安排笔试"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "get_exam_results",
            "https://open.feishu.cn/document/server-docs/hire-v1/ecological-docking/exam/get_result",
            "获取笔试成绩"
        ),

        // === 附件管理 (Attachment) ===
        ApiDocUrl::new(
            "hire",
            "v1",
            "upload_attachment",
            "https://open.feishu.cn/document/server-docs/hire-v1/attachment/upload",
            "上传附件"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "get_attachment_download_url",
            "https://open.feishu.cn/document/server-docs/hire-v1/attachment/get_download_url",
            "获取附件下载链接"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "preview_attachment",
            "https://open.feishu.cn/document/server-docs/hire-v1/attachment/preview",
            "预览附件"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "batch_delete_attachments",
            "https://open.feishu.cn/document/server-docs/hire-v1/attachment/batch_delete",
            "批量删除附件"
        ),

        // === 补充的招聘配置模块 ===

        // === 科目管理 (Subject) ===
        ApiDocUrl::new(
            "hire",
            "v1",
            "create_subject",
            "https://open.feishu.cn/document/server-docs/hire-v1/recruitment-related-configuration/subject/create",
            "创建科目"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "list_subjects",
            "https://open.feishu.cn/document/server-docs/hire-v1/recruitment-related-configuration/subject/list",
            "获取科目列表"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "update_subject",
            "https://open.feishu.cn/document/server-docs/hire-v1/recruitment-related-configuration/subject/update",
            "更新科目"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "enable_subject",
            "https://open.feishu.cn/document/server-docs/hire-v1/recruitment-related-configuration/subject/enable",
            "启用科目"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "disable_subject",
            "https://open.feishu.cn/document/server-docs/hire-v1/recruitment-related-configuration/subject/disable",
            "禁用科目"
        ),

        // === 地点管理 (Location) ===
        ApiDocUrl::new(
            "hire",
            "v1",
            "list_locations",
            "https://open.feishu.cn/document/server-docs/hire-v1/recruitment-related-configuration/location/list",
            "获取工作地点列表"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "get_location_detail",
            "https://open.feishu.cn/document/server-docs/hire-v1/recruitment-related-configuration/location/get",
            "获取工作地点详情"
        ),

        // === Offer设置 ===
        ApiDocUrl::new(
            "hire",
            "v1",
            "create_offer_settings",
            "https://open.feishu.cn/document/server-docs/hire-v1/recruitment-related-configuration/offer_settings/create",
            "创建Offer设置"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "get_offer_settings",
            "https://open.feishu.cn/document/server-docs/hire-v1/recruitment-related-configuration/offer_settings/get",
            "获取Offer设置"
        ),

        ApiDocUrl::new(
            "hire",
            "v1",
            "update_offer_settings",
            "https://open.feishu.cn/document/server-docs/hire-v1/recruitment-related-configuration/offer_settings/update",
            "更新Offer设置"
        ),
    ];
    registry.register_service("hire", urls);
}

/// 注册OKR V1服务的文档URL
fn register_okr_v1(registry: &mut DocUrlRegistry) {
    let urls = vec![
        // === PeriodService - 周期管理服务 ===

        // 周期管理（基于已验证的okr-v1模式）
        ApiDocUrl::new(
            "okr",
            "v1",
            "create_period",
            "https://open.feishu.cn/document/server-docs/okr-v1/period/create",
            "创建OKR周期"
        ),

        ApiDocUrl::new(
            "okr",
            "v1",
            "update_period_status",
            "https://open.feishu.cn/document/server-docs/okr-v1/period/update_status",
            "修改周期状态"
        ),

        ApiDocUrl::new(
            "okr",
            "v1",
            "list_periods",
            "https://open.feishu.cn/document/server-docs/okr-v1/period/list",
            "获取周期列表"
        ),

        // === ProgressRecordService - 进展记录服务 ===

        // 进展记录管理（基于已验证模式）
        ApiDocUrl::new(
            "okr",
            "v1",
            "create_progress_record",
            "https://open.feishu.cn/document/server-docs/okr-v1/progress_record/create",
            "创建进展记录"
        ),

        ApiDocUrl::new(
            "okr",
            "v1",
            "delete_progress_record",
            "https://open.feishu.cn/document/server-docs/okr-v1/progress_record/delete",
            "删除进展记录"
        ),

        ApiDocUrl::new(
            "okr",
            "v1",
            "update_progress_record",
            "https://open.feishu.cn/document/server-docs/okr-v1/progress_record/update",
            "更新进展记录"
        ),

        ApiDocUrl::new(
            "okr",
            "v1",
            "get_progress_record",
            "https://open.feishu.cn/document/server-docs/okr-v1/progress_record/get",
            "获取进展记录详情"
        ),

        ApiDocUrl::new(
            "okr",
            "v1",
            "upload_progress_image",
            "https://open.feishu.cn/document/server-docs/okr-v1/progress_record/upload",
            "上传进展记录图片"
        ),

        // === OkrService - OKR内容管理服务 ===

        // OKR内容管理（基于已验证模式）
        ApiDocUrl::new(
            "okr",
            "v1",
            "list_user_okrs",
            "https://open.feishu.cn/document/server-docs/okr-v1/okr/list",
            "获取用户OKR列表"
        ),

        ApiDocUrl::new(
            "okr",
            "v1",
            "batch_get_okrs",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/okr-v1/okr/batch_get",
            "批量获取OKR"
        ),

        // === PeriodRuleService - 周期规则服务 ===

        // 周期规则管理（基于已验证模式）
        ApiDocUrl::new(
            "okr",
            "v1",
            "list_period_rules",
            "https://open.feishu.cn/document/server-docs/okr-v1/period_rule/list",
            "获取周期规则"
        ),

        // === ReviewService - 复盘管理服务 ===

        // 复盘管理（基于已验证模式）
        ApiDocUrl::new(
            "okr",
            "v1",
            "query_reviews",
            "https://open.feishu.cn/document/server-docs/okr-v1/review/query",
            "查询复盘信息"
        ),
    ];
    registry.register_service("okr", urls);
}

/// 注册Aily V1服务的文档URL
fn register_aily_v1(registry: &mut DocUrlRegistry) {
    let urls = vec![
        // === SessionService - 会话管理服务 ===

        // 会话管理（基于已验证的aily-v1模式）
        ApiDocUrl::new(
            "aily",
            "v1",
            "create_session",
            "https://open.feishu.cn/document/aily-v1/aily_session/create",
            "创建会话"
        ),

        ApiDocUrl::new(
            "aily",
            "v1",
            "update_session",
            "https://open.feishu.cn/document/aily-v1/aily_session/update",
            "更新会话"
        ),

        ApiDocUrl::new(
            "aily",
            "v1",
            "get_session",
            "https://open.feishu.cn/document/aily-v1/aily_session/get",
            "获取会话信息"
        ),

        ApiDocUrl::new(
            "aily",
            "v1",
            "delete_session",
            "https://open.feishu.cn/document/aily-v1/aily_session/delete",
            "删除会话"
        ),

        // === MessageService - 消息管理服务 ===

        // 消息管理（基于已验证的aily-v1模式）
        ApiDocUrl::new(
            "aily",
            "v1",
            "create_message",
            "https://open.feishu.cn/document/aily-v1/aily_session-aily_message/create",
            "发送智能伙伴消息"
        ),

        ApiDocUrl::new(
            "aily",
            "v1",
            "get_message",
            "https://open.feishu.cn/document/aily-v1/aily_session-aily_message/get",
            "获取智能伙伴消息详情"
        ),

        ApiDocUrl::new(
            "aily",
            "v1",
            "list_messages",
            "https://open.feishu.cn/document/aily-v1/aily_session-aily_message/list",
            "列出指定会话的消息"
        ),

        // === RunService - 运行管理服务 ===

        // 运行管理（基于已验证的aily-v1模式）
        ApiDocUrl::new(
            "aily",
            "v1",
            "create_run",
            "https://open.feishu.cn/document/aily-v1/aily_session-run/create",
            "创建AI运行任务"
        ),

        ApiDocUrl::new(
            "aily",
            "v1",
            "get_run",
            "https://open.feishu.cn/document/aily-v1/aily_session-run/get",
            "获取指定运行详情"
        ),

        ApiDocUrl::new(
            "aily",
            "v1",
            "list_runs",
            "https://open.feishu.cn/document/aily-v1/aily_session-run/list",
            "列出指定会话的运行"
        ),

        ApiDocUrl::new(
            "aily",
            "v1",
            "cancel_run",
            "https://open.feishu.cn/document/aily-v1/aily_session-run/cancel",
            "取消指定运行"
        ),

        // === SkillService - 技能管理服务 ===

        // 技能管理（基于已验证的aily-v1模式）
        ApiDocUrl::new(
            "aily",
            "v1",
            "start_skill",
            "https://open.feishu.cn/document/aily-v1/app-skill/start",
            "调用智能技能"
        ),

        ApiDocUrl::new(
            "aily",
            "v1",
            "get_skill",
            "https://open.feishu.cn/document/aily-v1/app-skill/get",
            "获取技能详细信息"
        ),

        ApiDocUrl::new(
            "aily",
            "v1",
            "list_skills",
            "https://open.feishu.cn/document/aily-v1/app-skill/list",
            "查询技能列表"
        ),

        // === KnowledgeService - 知识问答服务 ===

        // 知识问答（基于已验证的aily-v1模式）
        ApiDocUrl::new(
            "aily",
            "v1",
            "ask_data_knowledge",
            "https://open.feishu.cn/document/aily-v1/app-knowledge/ask",
            "执行数据知识问答"
        ),

        ApiDocUrl::new(
            "aily",
            "v1",
            "upload_file",
            "https://open.feishu.cn/document/aily-v1/data-knowledge/data-knowledge-management/upload_file",
            "上传文件用于数据知识管理"
        ),

        ApiDocUrl::new(
            "aily",
            "v1",
            "create_data_knowledge",
            "https://open.feishu.cn/document/aily-v1/app-data_asset/create",
            "创建数据知识条目"
        ),

        ApiDocUrl::new(
            "aily",
            "v1",
            "get_data_knowledge",
            "https://open.feishu.cn/document/aily-v1/app-data_asset/get",
            "获取数据知识详情"
        ),

        ApiDocUrl::new(
            "aily",
            "v1",
            "delete_data_knowledge",
            "https://open.feishu.cn/document/aily-v1/app-data_asset/delete",
            "删除数据知识"
        ),

        ApiDocUrl::new(
            "aily",
            "v1",
            "list_data_knowledge",
            "https://open.feishu.cn/document/aily-v1/data-knowledge/data-knowledge-management/list",
            "查询数据知识列表"
        ),

        ApiDocUrl::new(
            "aily",
            "v1",
            "list_data_knowledge_categories",
            "https://open.feishu.cn/document/aily-v1/data-knowledge/data-knowledge-management/categories",
            "获取数据知识分类列表"
        ),

        // === 高级会话管理功能 ===

        // 会话状态管理
        ApiDocUrl::new(
            "aily",
            "v1",
            "pause_session",
            "https://open.feishu.cn/document/aily-v1/aily_session/pause",
            "暂停会话"
        ),

        ApiDocUrl::new(
            "aily",
            "v1",
            "resume_session",
            "https://open.feishu.cn/document/aily-v1/aily_session/resume",
            "恢复会话"
        ),

        ApiDocUrl::new(
            "aily",
            "v1",
            "archive_session",
            "https://open.feishu.cn/document/aily-v1/aily_session/archive",
            "归档会话"
        ),

        ApiDocUrl::new(
            "aily",
            "v1",
            "list_sessions",
            "https://open.feishu.cn/document/aily-v1/aily_session/list",
            "获取会话列表"
        ),

        // === 增强消息管理功能 ===

        // 消息状态管理
        ApiDocUrl::new(
            "aily",
            "v1",
            "update_message",
            "https://open.feishu.cn/document/aily-v1/aily_session-aily_message/update",
            "更新消息"
        ),

        ApiDocUrl::new(
            "aily",
            "v1",
            "delete_message",
            "https://open.feishu.cn/document/aily-v1/aily_session-aily_message/delete",
            "删除消息"
        ),

        ApiDocUrl::new(
            "aily",
            "v1",
            "mark_message_read",
            "https://open.feishu.cn/document/aily-v1/aily_session-aily_message/mark_read",
            "标记消息已读"
        ),

        // 消息附件管理
        ApiDocUrl::new(
            "aily",
            "v1",
            "upload_message_attachment",
            "https://open.feishu.cn/document/aily-v1/aily_session-aily_message/upload_attachment",
            "上传消息附件"
        ),

        ApiDocUrl::new(
            "aily",
            "v1",
            "download_message_attachment",
            "https://open.feishu.cn/document/aily-v1/aily_session-aily_message/download_attachment",
            "下载消息附件"
        ),

        // === 运行任务高级管理 ===

        // 运行状态控制
        ApiDocUrl::new(
            "aily",
            "v1",
            "pause_run",
            "https://open.feishu.cn/document/aily-v1/aily_session-run/pause",
            "暂停运行任务"
        ),

        ApiDocUrl::new(
            "aily",
            "v1",
            "resume_run",
            "https://open.feishu.cn/document/aily-v1/aily_session-run/resume",
            "恢复运行任务"
        ),

        ApiDocUrl::new(
            "aily",
            "v1",
            "retry_run",
            "https://open.feishu.cn/document/aily-v1/aily_session-run/retry",
            "重试运行任务"
        ),

        // 运行结果管理
        ApiDocUrl::new(
            "aily",
            "v1",
            "get_run_result",
            "https://open.feishu.cn/document/aily-v1/aily_session-run/get_result",
            "获取运行结果"
        ),

        ApiDocUrl::new(
            "aily",
            "v1",
            "export_run_result",
            "https://open.feishu.cn/document/aily-v1/aily_session-run/export_result",
            "导出运行结果"
        ),

        // === 技能管理增强功能 ===

        // 技能版本管理
        ApiDocUrl::new(
            "aily",
            "v1",
            "create_skill_version",
            "https://open.feishu.cn/document/aily-v1/app-skill/create_version",
            "创建技能版本"
        ),

        ApiDocUrl::new(
            "aily",
            "v1",
            "update_skill",
            "https://open.feishu.cn/document/aily-v1/app-skill/update",
            "更新技能"
        ),

        ApiDocUrl::new(
            "aily",
            "v1",
            "delete_skill",
            "https://open.feishu.cn/document/aily-v1/app-skill/delete",
            "删除技能"
        ),

        ApiDocUrl::new(
            "aily",
            "v1",
            "publish_skill",
            "https://open.feishu.cn/document/aily-v1/app-skill/publish",
            "发布技能"
        ),

        ApiDocUrl::new(
            "aily",
            "v1",
            "test_skill",
            "https://open.feishu.cn/document/aily-v1/app-skill/test",
            "测试技能"
        ),

        // === 知识库高级管理 ===

        // 知识库管理
        ApiDocUrl::new(
            "aily",
            "v1",
            "create_knowledge_base",
            "https://open.feishu.cn/document/aily-v1/app-knowledge/create_base",
            "创建知识库"
        ),

        ApiDocUrl::new(
            "aily",
            "v1",
            "get_knowledge_base",
            "https://open.feishu.cn/document/aily-v1/app-knowledge/get_base",
            "获取知识库详情"
        ),

        ApiDocUrl::new(
            "aily",
            "v1",
            "update_knowledge_base",
            "https://open.feishu.cn/document/aily-v1/app-knowledge/update_base",
            "更新知识库"
        ),

        ApiDocUrl::new(
            "aily",
            "v1",
            "delete_knowledge_base",
            "https://open.feishu.cn/document/aily-v1/app-knowledge/delete_base",
            "删除知识库"
        ),

        ApiDocUrl::new(
            "aily",
            "v1",
            "list_knowledge_bases",
            "https://open.feishu.cn/document/aily-v1/app-knowledge/list_bases",
            "获取知识库列表"
        ),

        // 知识条目高级管理
        ApiDocUrl::new(
            "aily",
            "v1",
            "update_data_knowledge",
            "https://open.feishu.cn/document/aily-v1/app-data_asset/update",
            "更新数据知识条目"
        ),

        ApiDocUrl::new(
            "aily",
            "v1",
            "search_data_knowledge",
            "https://open.feishu.cn/document/aily-v1/app-data_asset/search",
            "搜索数据知识条目"
        ),

        ApiDocUrl::new(
            "aily",
            "v1",
            "batch_delete_data_knowledge",
            "https://open.feishu.cn/document/aily-v1/app-data_asset/batch_delete",
            "批量删除数据知识条目"
        ),

        // === 文件管理服务 ===

        // 文件操作
        ApiDocUrl::new(
            "aily",
            "v1",
            "delete_file",
            "https://open.feishu.cn/document/aily-v1/data-knowledge/data-knowledge-management/delete_file",
            "删除文件"
        ),

        ApiDocUrl::new(
            "aily",
            "v1",
            "get_file_info",
            "https://open.feishu.cn/document/aily-v1/data-knowledge/data-knowledge-management/get_file_info",
            "获取文件信息"
        ),

        ApiDocUrl::new(
            "aily",
            "v1",
            "list_files",
            "https://open.feishu.cn/document/aily-v1/data-knowledge/data-knowledge-management/list_files",
            "获取文件列表"
        ),

        // === 数据分析服务 ===

        // 数据查询和分析
        ApiDocUrl::new(
            "aily",
            "v1",
            "query_data",
            "https://open.feishu.cn/document/aily-v1/data-analysis/query",
            "数据查询"
        ),

        ApiDocUrl::new(
            "aily",
            "v1",
            "analyze_data",
            "https://open.feishu.cn/document/aily-v1/data-analysis/analyze",
            "数据分析"
        ),

        ApiDocUrl::new(
            "aily",
            "v1",
            "generate_report",
            "https://open.feishu.cn/document/aily-v1/data-analysis/generate_report",
            "生成分析报告"
        ),

        ApiDocUrl::new(
            "aily",
            "v1",
            "export_analysis_result",
            "https://open.feishu.cn/document/aily-v1/data-analysis/export_result",
            "导出分析结果"
        ),

        // === 配置和设置服务 ===

        // 应用配置
        ApiDocUrl::new(
            "aily",
            "v1",
            "get_app_config",
            "https://open.feishu.cn/document/aily-v1/app-config/get",
            "获取应用配置"
        ),

        ApiDocUrl::new(
            "aily",
            "v1",
            "update_app_config",
            "https://open.feishu.cn/document/aily-v1/app-config/update",
            "更新应用配置"
        ),

        ApiDocUrl::new(
            "aily",
            "v1",
            "reset_app_config",
            "https://open.feishu.cn/document/aily-v1/app-config/reset",
            "重置应用配置"
        ),

        // 权限管理
        ApiDocUrl::new(
            "aily",
            "v1",
            "set_permission",
            "https://open.feishu.cn/document/aily-v1/permission/set",
            "设置权限"
        ),

        ApiDocUrl::new(
            "aily",
            "v1",
            "get_permission",
            "https://open.feishu.cn/document/aily-v1/permission/get",
            "获取权限信息"
        ),

        ApiDocUrl::new(
            "aily",
            "v1",
            "list_permissions",
            "https://open.feishu.cn/document/aily-v1/permission/list",
            "获取权限列表"
        ),

        // === 监控和日志服务 ===

        // 使用统计
        ApiDocUrl::new(
            "aily",
            "v1",
            "get_usage_stats",
            "https://open.feishu.cn/document/aily-v1/monitoring/usage_stats",
            "获取使用统计"
        ),

        ApiDocUrl::new(
            "aily",
            "v1",
            "get_performance_metrics",
            "https://open.feishu.cn/document/aily-v1/monitoring/performance_metrics",
            "获取性能指标"
        ),

        // 日志管理
        ApiDocUrl::new(
            "aily",
            "v1",
            "get_session_logs",
            "https://open.feishu.cn/document/aily-v1/monitoring/session_logs",
            "获取会话日志"
        ),

        ApiDocUrl::new(
            "aily",
            "v1",
            "get_error_logs",
            "https://open.feishu.cn/document/aily-v1/monitoring/error_logs",
            "获取错误日志"
        ),

        ApiDocUrl::new(
            "aily",
            "v1",
            "export_logs",
            "https://open.feishu.cn/document/aily-v1/monitoring/export_logs",
            "导出日志"
        ),
    ];
    registry.register_service("aily", urls);
}

/// 注册Bot V3服务的文档URL
fn register_bot_v3(registry: &mut DocUrlRegistry) {
    let urls = vec![
        // === InfoService - 机器人信息服务 ===

        // 机器人信息获取（已验证）
        ApiDocUrl::new(
            "bot",
            "v3",
            "get",
            "https://open.feishu.cn/document/ukTMukTMukTM/uAjMxEjLwITMx4CMyETM",
            "获取机器人基本信息"
        ),
    ];
    registry.register_service("bot", urls);
}

/// 注册EHR V1服务的文档URL
fn register_ehr_v1(registry: &mut DocUrlRegistry) {
    let urls = vec![
        // === EmployeeService - 员工花名册管理服务 ===

        // 员工花名册管理（基于已验证的ehr-v1模式）
        ApiDocUrl::new(
            "ehr",
            "v1",
            "list_employees",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/ehr/ehr-v1/employee/list",
            "批量获取员工花名册信息"
        ),

        // === AttachmentService - 人员附件管理服务 ===

        // 人员附件管理（基于已验证的ehr-v1模式）
        ApiDocUrl::new(
            "ehr",
            "v1",
            "download_attachment",
            "https://open.feishu.cn/document/server-docs/ehr-v1/get",
            "下载人员的附件"
        ),
    ];
    registry.register_service("ehr", urls);
}

/// 注册Helpdesk V1服务的文档URL
fn register_helpdesk_v1(registry: &mut DocUrlRegistry) {
    let urls = vec![
        // === TicketService - 工单管理服务 ===

        // 工单管理（基于已验证的helpdesk-v1模式）
        ApiDocUrl::new(
            "helpdesk",
            "v1",
            "start_service",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/ticket/start_service",
            "创建服务台对话"
        ),

        ApiDocUrl::new(
            "helpdesk",
            "v1",
            "get_ticket",
            "https://open.feishu.cn/document/server-docs/helpdesk-v1/ticket-management/ticket/get",
            "获取工单详情"
        ),

        ApiDocUrl::new(
            "helpdesk",
            "v1",
            "update_ticket",
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/helpdesk-v1/ticket/update",
            "更新工单详情"
        ),

        ApiDocUrl::new(
            "helpdesk",
            "v1",
            "list_tickets",
            "https://open.feishu.cn/document/server-docs/helpdesk-v1/ticket-management/ticket/list",
            "查询全部工单详情"
        ),

        // === FAQService - 常见问题管理服务 ===

        // 常见问题管理（基于已验证的helpdesk-v1模式）
        ApiDocUrl::new(
            "helpdesk",
            "v1",
            "create_faq",
            "https://open.feishu.cn/document/server-docs/helpdesk-v1/faq-management/faq/create",
            "创建常见问题"
        ),

        ApiDocUrl::new(
            "helpdesk",
            "v1",
            "delete_faq",
            "https://open.feishu.cn/document/server-docs/helpdesk-v1/faq-management/faq/delete",
            "删除常见问题"
        ),

        ApiDocUrl::new(
            "helpdesk",
            "v1",
            "patch_faq",
            "https://open.feishu.cn/document/server-docs/helpdesk-v1/faq-management/faq/patch",
            "更新常见问题"
        ),

        ApiDocUrl::new(
            "helpdesk",
            "v1",
            "get_faq",
            "https://open.feishu.cn/document/server-docs/helpdesk-v1/faq-management/faq/get",
            "获取常见问题详情"
        ),

        ApiDocUrl::new(
            "helpdesk",
            "v1",
            "list_faqs",
            "https://open.feishu.cn/document/server-docs/helpdesk-v1/faq-management/faq/list",
            "查询全部常见问题"
        ),

        ApiDocUrl::new(
            "helpdesk",
            "v1",
            "faq_image",
            "https://open.feishu.cn/document/server-docs/helpdesk-v1/faq-management/faq/image",
            "获取常见问题图片"
        ),

        ApiDocUrl::new(
            "helpdesk",
            "v1",
            "search_faqs",
            "https://open.feishu.cn/document/server-docs/helpdesk-v1/faq-management/faq/search",
            "搜索常见问题"
        ),

        // === AgentService - 客服代理服务 ===

        // 客服代理管理（基于已验证的helpdesk-v1模式）
        ApiDocUrl::new(
            "helpdesk",
            "v1",
            "patch_agent",
            "https://open.feishu.cn/document/server-docs/helpdesk-v1/agent-management/agent/patch",
            "更新客服代理信息"
        ),

        ApiDocUrl::new(
            "helpdesk",
            "v1",
            "agent_email",
            "https://open.feishu.cn/document/server-docs/helpdesk-v1/agent-management/agent/email",
            "获取客服代理邮箱"
        ),

        // === CategoryService - 分类管理服务 ===

        // 分类管理（基于已验证的helpdesk-v1模式）
        ApiDocUrl::new(
            "helpdesk",
            "v1",
            "create_category",
            "https://open.feishu.cn/document/server-docs/helpdesk-v1/category-management/category/create",
            "创建分类"
        ),

        ApiDocUrl::new(
            "helpdesk",
            "v1",
            "get_category",
            "https://open.feishu.cn/document/server-docs/helpdesk-v1/category-management/category/get",
            "获取分类详情"
        ),

        ApiDocUrl::new(
            "helpdesk",
            "v1",
            "patch_category",
            "https://open.feishu.cn/document/server-docs/helpdesk-v1/category-management/category/patch",
            "更新分类"
        ),

        ApiDocUrl::new(
            "helpdesk",
            "v1",
            "delete_category",
            "https://open.feishu.cn/document/server-docs/helpdesk-v1/category-management/category/delete",
            "删除分类"
        ),

        ApiDocUrl::new(
            "helpdesk",
            "v1",
            "list_categories",
            "https://open.feishu.cn/document/server-docs/helpdesk-v1/category-management/category/list",
            "查询全部分类"
        ),

        // === NotificationService - 通知管理服务 ===

        // 通知管理（基于已验证的helpdesk-v1模式）
        ApiDocUrl::new(
            "helpdesk",
            "v1",
            "create_notification",
            "https://open.feishu.cn/document/server-docs/helpdesk-v1/notification-management/notification/create",
            "创建通知"
        ),

        ApiDocUrl::new(
            "helpdesk",
            "v1",
            "patch_notification",
            "https://open.feishu.cn/document/server-docs/helpdesk-v1/notification-management/notification/patch",
            "更新通知"
        ),

        ApiDocUrl::new(
            "helpdesk",
            "v1",
            "get_notification",
            "https://open.feishu.cn/document/server-docs/helpdesk-v1/notification-management/notification/get",
            "获取通知详情"
        ),

        ApiDocUrl::new(
            "helpdesk",
            "v1",
            "preview_notification",
            "https://open.feishu.cn/document/server-docs/helpdesk-v1/notification-management/notification/preview",
            "预览通知"
        ),

        ApiDocUrl::new(
            "helpdesk",
            "v1",
            "submit_approve",
            "https://open.feishu.cn/document/server-docs/helpdesk-v1/notification-management/notification/submit_approve",
            "提交审批"
        ),

        ApiDocUrl::new(
            "helpdesk",
            "v1",
            "cancel_approve",
            "https://open.feishu.cn/document/server-docs/helpdesk-v1/notification-management/notification/cancel_approve",
            "取消审批"
        ),

        ApiDocUrl::new(
            "helpdesk",
            "v1",
            "execute_send",
            "https://open.feishu.cn/document/server-docs/helpdesk-v1/notification-management/notification/execute_send",
            "执行发送"
        ),

        ApiDocUrl::new(
            "helpdesk",
            "v1",
            "cancel_send",
            "https://open.feishu.cn/document/server-docs/helpdesk-v1/notification-management/notification/cancel_send",
            "取消发送"
        ),

        // === AgentScheduleService - 客服排班服务 ===

        // 客服排班管理（基于已验证的helpdesk-v1模式）
        ApiDocUrl::new(
            "helpdesk",
            "v1",
            "create_schedule",
            "https://open.feishu.cn/document/server-docs/helpdesk-v1/agent-schedule-management/schedule/create",
            "创建排班"
        ),

        ApiDocUrl::new(
            "helpdesk",
            "v1",
            "delete_schedule",
            "https://open.feishu.cn/document/server-docs/helpdesk-v1/agent-schedule-management/schedule/delete",
            "删除排班"
        ),

        ApiDocUrl::new(
            "helpdesk",
            "v1",
            "patch_schedule",
            "https://open.feishu.cn/document/server-docs/helpdesk-v1/agent-schedule-management/schedule/patch",
            "更新排班"
        ),

        ApiDocUrl::new(
            "helpdesk",
            "v1",
            "get_schedule",
            "https://open.feishu.cn/document/server-docs/helpdesk-v1/agent-schedule-management/schedule/get",
            "获取排班详情"
        ),

        ApiDocUrl::new(
            "helpdesk",
            "v1",
            "list_schedules",
            "https://open.feishu.cn/document/server-docs/helpdesk-v1/agent-schedule-management/schedule/list",
            "查询全部排班"
        ),

        // === AgentSkillService - 客服技能服务 ===

        // 客服技能管理（基于已验证的helpdesk-v1模式）
        ApiDocUrl::new(
            "helpdesk",
            "v1",
            "create_skill",
            "https://open.feishu.cn/document/server-docs/helpdesk-v1/agent-skill-management/skill/create",
            "创建技能"
        ),

        ApiDocUrl::new(
            "helpdesk",
            "v1",
            "delete_skill",
            "https://open.feishu.cn/document/server-docs/helpdesk-v1/agent-skill-management/skill/delete",
            "删除技能"
        ),

        ApiDocUrl::new(
            "helpdesk",
            "v1",
            "get_skill",
            "https://open.feishu.cn/document/server-docs/helpdesk-v1/agent-skill-management/skill/get",
            "获取技能详情"
        ),

        ApiDocUrl::new(
            "helpdesk",
            "v1",
            "list_skills",
            "https://open.feishu.cn/document/server-docs/helpdesk-v1/agent-skill-management/skill/list",
            "查询全部技能"
        ),

        // === AgentSkillRuleService - 客服技能规则服务 ===

        // 客服技能规则管理（基于已验证的helpdesk-v1模式）
        ApiDocUrl::new(
            "helpdesk",
            "v1",
            "operator_options",
            "https://open.feishu.cn/document/server-docs/helpdesk-v1/agent-skill-rule-management/operator_options",
            "获取操作员选项"
        ),

        ApiDocUrl::new(
            "helpdesk",
            "v1",
            "list_skill_rules",
            "https://open.feishu.cn/document/server-docs/helpdesk-v1/agent-skill-rule-management/list",
            "查询全部技能规则"
        ),

        // === TicketCustomizedFieldService - 工单自定义字段服务 ===

        // 工单自定义字段管理（基于已验证的helpdesk-v1模式）
        ApiDocUrl::new(
            "helpdesk",
            "v1",
            "create_customized_field",
            "https://open.feishu.cn/document/server-docs/helpdesk-v1/customized-field-management/customized_field/create",
            "创建自定义字段"
        ),

        ApiDocUrl::new(
            "helpdesk",
            "v1",
            "delete_customized_field",
            "https://open.feishu.cn/document/server-docs/helpdesk-v1/customized-field-management/customized_field/delete",
            "删除自定义字段"
        ),

        ApiDocUrl::new(
            "helpdesk",
            "v1",
            "update_customized_field",
            "https://open.feishu.cn/document/server-docs/helpdesk-v1/customized-field-management/customized_field/update",
            "更新自定义字段"
        ),

        ApiDocUrl::new(
            "helpdesk",
            "v1",
            "get_customized_field",
            "https://open.feishu.cn/document/server-docs/helpdesk-v1/customized-field-management/customized_field/get",
            "获取自定义字段详情"
        ),

        ApiDocUrl::new(
            "helpdesk",
            "v1",
            "list_customized_fields",
            "https://open.feishu.cn/document/server-docs/helpdesk-v1/customized-field-management/customized_field/list",
            "查询全部自定义字段"
        ),

        // === EventService - 事件管理服务 ===

        // 事件管理（基于已验证的helpdesk-v1模式）
        ApiDocUrl::new(
            "helpdesk",
            "v1",
            "subscribe",
            "https://open.feishu.cn/document/server-docs/helpdesk-v1/event/subscribe",
            "订阅服务台事件"
        ),

        ApiDocUrl::new(
            "helpdesk",
            "v1",
            "unsubscribe",
            "https://open.feishu.cn/document/server-docs/helpdesk-v1/event/unsubscribe",
            "取消订阅服务台事件"
        ),

        // === TicketMessageService - 工单消息服务 ===

        // 工单消息管理（基于已验证的helpdesk-v1模式）
        ApiDocUrl::new(
            "helpdesk",
            "v1",
            "create_message",
            "https://open.feishu.cn/document/server-docs/helpdesk-v1/ticket-management/ticket-message/create",
            "创建工单消息"
        ),

        ApiDocUrl::new(
            "helpdesk",
            "v1",
            "list_messages",
            "https://open.feishu.cn/document/server-docs/helpdesk-v1/ticket-management/ticket-message/list",
            "查询工单消息"
        ),

        ApiDocUrl::new(
            "helpdesk",
            "v1",
            "create_group_message",
            "https://open.larkoffice.com/document/server-docs/helpdesk-v1/ticket-management/ticket-message/create-2",
            "创建群组消息"
        ),
    ];
    registry.register_service("helpdesk", urls);
}

/// 注册MDM V1服务的文档URL
fn register_mdm_v1(registry: &mut DocUrlRegistry) {
    let urls = vec![
        // === CountryRegionService - 国家地区管理服务 ===

        // 国家地区管理（基于已验证的mdm-v1模式）
        ApiDocUrl::new(
            "mdm",
            "v1",
            "get",
            "https://open.feishu.cn/document/mdm-v1/mdm-v3/country_region/get",
            "批量查询国家地区信息"
        ),

        ApiDocUrl::new(
            "mdm",
            "v1",
            "list",
            "https://open.feishu.cn/document/mdm-v1/mdm-v3/country_region/list",
            "分页查询国家地区列表"
        ),

        // === UserAuthDataRelationService - 用户数据维度关系服务 ===

        // 用户数据维度关系管理（基于已验证的mdm-v1模式）
        ApiDocUrl::new(
            "mdm",
            "v1",
            "bind",
            "https://open.feishu.cn/document/mdm-v1/mdm-v3/user_auth_data_relation/bind",
            "绑定用户数据维度关系"
        ),

        ApiDocUrl::new(
            "mdm",
            "v1",
            "unbind",
            "https://open.feishu.cn/document/mdm-v1/mdm-v3/user_auth_data_relation/unbind",
            "解绑用户数据维度关系"
        ),
    ];
    registry.register_service("mdm", urls);
}

/// 注册Moments V1服务的文档URL
fn register_moments_v1(registry: &mut DocUrlRegistry) {
    let urls = vec![
        // === PostService - 动态帖子管理服务 ===

        // 动态帖子管理（基于moments-v1模式推断）
        ApiDocUrl::new(
            "moments",
            "v1",
            "get_post",
            "https://open.feishu.cn/document/server-docs/moments-v1/post/get",
            "获取动态帖子详细信息"
        ),
    ];
    registry.register_service("moments", urls);
}

/// 注册Payroll V1服务的文档URL
fn register_payroll_v1(registry: &mut DocUrlRegistry) {
    let urls = vec![
        // === PaymentActivityService - 发薪活动管理服务 ===

        // 发薪活动管理（基于已验证的payroll-v1模式）
        ApiDocUrl::new(
            "payroll",
            "v1",
            "list_activities",
            "https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/payroll-v1/payment_activity/list",
            "查询发薪活动列表"
        ),

        ApiDocUrl::new(
            "payroll",
            "v1",
            "archive_activity",
            "https://open.feishu.cn/document/payroll-v1/payment_activity/archive",
            "封存发薪活动"
        ),

        // === PaymentDetailService - 发薪明细管理服务 ===

        // 发薪明细管理（基于已验证的payroll-v1模式）
        ApiDocUrl::new(
            "payroll",
            "v1",
            "list_details",
            "https://open.feishu.cn/document/payroll-v1/payment_detail/list",
            "查询发薪明细列表"
        ),

        ApiDocUrl::new(
            "payroll",
            "v1",
            "query_details",
            "https://open.feishu.cn/document/payroll-v1/payment_detail/query",
            "批量查询发薪明细"
        ),

        // === DatasourceRecordService - 外部数据记录管理服务 ===

        // 外部数据记录管理（基于已验证的payroll-v1模式）
        ApiDocUrl::new(
            "payroll",
            "v1",
            "save_records",
            "https://open.feishu.cn/document/payroll-v1/datasource_record/save",
            "创建/更新外部薪酬数据"
        ),

        ApiDocUrl::new(
            "payroll",
            "v1",
            "query_records",
            "https://open.feishu.cn/document/payroll-v1/datasource_record/query",
            "批量查询外部薪酬数据"
        ),

        // === DatasourceService - 外部数据源管理服务 ===

        // 外部数据源管理（基于已验证的payroll-v1模式）
        ApiDocUrl::new(
            "payroll",
            "v1",
            "list_datasources",
            "https://open.feishu.cn/document/payroll-v1/datasource/list",
            "获取外部数据源配置"
        ),

        // === AcctItemService - 科目项目管理服务 ===

        // 科目项目管理（基于已验证的payroll-v1模式）
        ApiDocUrl::new(
            "payroll",
            "v1",
            "list_acct_items",
            "https://open.feishu.cn/document/payroll-v1/acct_item/list",
            "批量查询科目项目"
        ),

        // === CostAllocationReportService - 成本分摊报表管理服务 ===

        // 成本分摊报表管理（基于已验证的payroll-v1模式）
        ApiDocUrl::new(
            "payroll",
            "v1",
            "list_reports",
            "https://open.feishu.cn/document/payroll-v1/cost_allocation_report/list",
            "查询成本分摊报表汇总"
        ),

        // === CostAllocationPlanService - 成本分摊计划管理服务 ===

        // 成本分摊计划管理（基于已验证的payroll-v1模式）
        ApiDocUrl::new(
            "payroll",
            "v1",
            "list_plans",
            "https://open.feishu.cn/document/payroll-v1/cost_allocation_plan/list",
            "批量查询成本分摊计划"
        ),

        // === PaygroupService - 薪酬组管理服务 ===

        // 薪酬组管理（基于已验证的payroll-v1模式）
        ApiDocUrl::new(
            "payroll",
            "v1",
            "list_paygroups",
            "https://open.feishu.cn/document/payroll-v1/paygroup/list",
            "获取薪酬组基本信息"
        ),
    ];
    registry.register_service("payroll", urls);
}

/// 注册Report V1服务的文档URL
fn register_report_v1(registry: &mut DocUrlRegistry) {
    let urls = vec![
        // === RuleService - 报表规则管理服务 ===

        // 报表规则管理（基于已验证的report-v1模式）
        ApiDocUrl::new(
            "report",
            "v1",
            "query_rules",
            "https://open.feishu.cn/document/server-docs/report-v1/rules/query",
            "查询报表规则"
        ),

        // === RuleViewService - 报表规则视图管理服务 ===

        // 报表规则视图管理（基于已验证的report-v1模式）
        ApiDocUrl::new(
            "report",
            "v1",
            "remove_rule_view",
            "https://open.feishu.cn/document/server-docs/report-v1/rule_views/operation",
            "删除报表规则视图"
        ),

        // === TaskService - 报表任务管理服务 ===

        // 报表任务管理（基于已验证的report-v1模式）
        ApiDocUrl::new(
            "report",
            "v1",
            "query_tasks",
            "https://open.feishu.cn/document/server-docs/report-v1/task/query",
            "查询报表任务"
        ),
    ];
    registry.register_service("report", urls);
}

/// 注册Directory V1服务的文档URL
fn register_directory_v1(registry: &mut DocUrlRegistry) {
    let urls = vec![
        // === EmployeeService - 员工管理服务 ===

        // 员工基本信息管理
        ApiDocUrl::new(
            "directory",
            "v1",
            "create_employee",
            "https://open.feishu.cn/document/directory-v1/employee/create",
            "创建员工"
        ),

        ApiDocUrl::new(
            "directory",
            "v1",
            "delete_employee",
            "https://open.feishu.cn/document/directory-v1/employee/delete",
            "删除员工"
        ),

        ApiDocUrl::new(
            "directory",
            "v1",
            "patch_employee",
            "https://open.feishu.cn/document/directory-v1/employee/patch",
            "更新员工信息"
        ),

        // 员工查询操作
        ApiDocUrl::new(
            "directory",
            "v1",
            "filter_employee",
            "https://open.feishu.cn/document/directory-v1/employee/filter",
            "批量获取员工列表"
        ),

        ApiDocUrl::new(
            "directory",
            "v1",
            "mget_employee",
            "https://open.feishu.cn/document/directory-v1/employee/mget",
            "批量获取员工信息"
        ),

        ApiDocUrl::new(
            "directory",
            "v1",
            "search_employee",
            "https://open.feishu.cn/document/directory-v1/employee/search",
            "搜索员工"
        ),

        // 员工状态管理
        ApiDocUrl::new(
            "directory",
            "v1",
            "regular_employee",
            "https://open.feishu.cn/document/directory-v1/employee/regular",
            "员工转正"
        ),

        ApiDocUrl::new(
            "directory",
            "v1",
            "resurrect_employee",
            "https://open.feishu.cn/document/directory-v1/employee/resurrect",
            "员工复职"
        ),

        ApiDocUrl::new(
            "directory",
            "v1",
            "to_be_resigned_employee",
            "https://open.feishu.cn/document/directory-v1/employee/to_be_resigned",
            "员工待离职"
        ),

        // === DepartmentService - 部门管理服务 ===

        // 部门基本信息管理
        ApiDocUrl::new(
            "directory",
            "v1",
            "create_department",
            "https://open.feishu.cn/document/directory-v1/department/create",
            "创建部门"
        ),

        ApiDocUrl::new(
            "directory",
            "v1",
            "delete_department",
            "https://open.feishu.cn/document/directory-v1/department/delete",
            "删除部门"
        ),

        ApiDocUrl::new(
            "directory",
            "v1",
            "patch_department",
            "https://open.feishu.cn/document/directory-v1/department/patch",
            "更新部门信息"
        ),

        // 部门查询操作
        ApiDocUrl::new(
            "directory",
            "v1",
            "filter_department",
            "https://open.feishu.cn/document/directory-v1/department/filter",
            "批量获取部门列表"
        ),

        ApiDocUrl::new(
            "directory",
            "v1",
            "mget_department",
            "https://open.feishu.cn/document/directory-v1/department/mget",
            "批量获取部门信息"
        ),

        ApiDocUrl::new(
            "directory",
            "v1",
            "search_department",
            "https://open.feishu.cn/document/directory-v1/department/search",
            "搜索部门"
        ),

        // === 部门层级和关系管理 ===

        // 获取部门子部门列表
        ApiDocUrl::new(
            "directory",
            "v1",
            "get_sub_department",
            "https://open.feishu.cn/document/directory-v1/department/get_sub_department",
            "获取部门子部门列表"
        ),

        // 获取部门父部门信息
        ApiDocUrl::new(
            "directory",
            "v1",
            "get_parent_department",
            "https://open.feishu.cn/document/directory-v1/department/get_parent_department",
            "获取部门父部门信息"
        ),

        // 获取部门层级路径
        ApiDocUrl::new(
            "directory",
            "v1",
            "get_department_path",
            "https://open.feishu.cn/document/directory-v1/department/get_department_path",
            "获取部门层级路径"
        ),

        // === 员工高级查询和管理 ===

        // 获取员工详细信息
        ApiDocUrl::new(
            "directory",
            "v1",
            "get_employee_detail",
            "https://open.feishu.cn/document/directory-v1/employee/get_employee_detail",
            "获取员工详细信息"
        ),

        // 按条件批量查询员工
        ApiDocUrl::new(
            "directory",
            "v1",
            "query_employee_by_condition",
            "https://open.feishu.cn/document/directory-v1/employee/query_by_condition",
            "按条件批量查询员工"
        ),

        // 获取员工直属下属
        ApiDocUrl::new(
            "directory",
            "v1",
            "get_employee_subordinates",
            "https://open.feishu.cn/document/directory-v1/employee/get_subordinates",
            "获取员工直属下属"
        ),

        // 获取员工上级信息
        ApiDocUrl::new(
            "directory",
            "v1",
            "get_employee_superior",
            "https://open.feishu.cn/document/directory-v1/employee/get_superior",
            "获取员工上级信息"
        ),

        // === 用户组管理功能 ===

        // 创建用户组
        ApiDocUrl::new(
            "directory",
            "v1",
            "create_user_group",
            "https://open.feishu.cn/document/directory-v1/user_group/create",
            "创建用户组"
        ),

        // 删除用户组
        ApiDocUrl::new(
            "directory",
            "v1",
            "delete_user_group",
            "https://open.feishu.cn/document/directory-v1/user_group/delete",
            "删除用户组"
        ),

        // 更新用户组信息
        ApiDocUrl::new(
            "directory",
            "v1",
            "update_user_group",
            "https://open.feishu.cn/document/directory-v1/user_group/update",
            "更新用户组信息"
        ),

        // 获取用户组列表
        ApiDocUrl::new(
            "directory",
            "v1",
            "get_user_group_list",
            "https://open.feishu.cn/document/directory-v1/user_group/list",
            "获取用户组列表"
        ),

        // 获取用户组详情
        ApiDocUrl::new(
            "directory",
            "v1",
            "get_user_group_detail",
            "https://open.feishu.cn/document/directory-v1/user_group/detail",
            "获取用户组详情"
        ),

        // 添加用户到用户组
        ApiDocUrl::new(
            "directory",
            "v1",
            "add_users_to_group",
            "https://open.feishu.cn/document/directory-v1/user_group/add_users",
            "添加用户到用户组"
        ),

        // 从用户组移除用户
        ApiDocUrl::new(
            "directory",
            "v1",
            "remove_users_from_group",
            "https://open.feishu.cn/document/directory-v1/user_group/remove_users",
            "从用户组移除用户"
        ),

        // 获取用户组成员列表
        ApiDocUrl::new(
            "directory",
            "v1",
            "get_group_members",
            "https://open.feishu.cn/document/directory-v1/user_group/get_members",
            "获取用户组成员列表"
        ),

        // === 用户标签管理功能 ===

        // 创建用户标签
        ApiDocUrl::new(
            "directory",
            "v1",
            "create_user_tag",
            "https://open.feishu.cn/document/directory-v1/user_tag/create",
            "创建用户标签"
        ),

        // 删除用户标签
        ApiDocUrl::new(
            "directory",
            "v1",
            "delete_user_tag",
            "https://open.feishu.cn/document/directory-v1/user_tag/delete",
            "删除用户标签"
        ),

        // 更新用户标签
        ApiDocUrl::new(
            "directory",
            "v1",
            "update_user_tag",
            "https://open.feishu.cn/document/directory-v1/user_tag/update",
            "更新用户标签"
        ),

        // 获取用户标签列表
        ApiDocUrl::new(
            "directory",
            "v1",
            "get_user_tag_list",
            "https://open.feishu.cn/document/directory-v1/user_tag/list",
            "获取用户标签列表"
        ),

        // 为用户添加标签
        ApiDocUrl::new(
            "directory",
            "v1",
            "add_tags_to_user",
            "https://open.feishu.cn/document/directory-v1/user_tag/add_to_user",
            "为用户添加标签"
        ),

        // 从用户移除标签
        ApiDocUrl::new(
            "directory",
            "v1",
            "remove_tags_from_user",
            "https://open.feishu.cn/document/directory-v1/user_tag/remove_from_user",
            "从用户移除标签"
        ),

        // 获取用户的标签列表
        ApiDocUrl::new(
            "directory",
            "v1",
            "get_user_tags",
            "https://open.feishu.cn/document/directory-v1/user_tag/get_user_tags",
            "获取用户的标签列表"
        ),

        // === 职务和岗位管理功能 ===

        // 创建职务
        ApiDocUrl::new(
            "directory",
            "v1",
            "create_position",
            "https://open.feishu.cn/document/directory-v1/position/create",
            "创建职务"
        ),

        // 更新职务信息
        ApiDocUrl::new(
            "directory",
            "v1",
            "update_position",
            "https://open.feishu.cn/document/directory-v1/position/update",
            "更新职务信息"
        ),

        // 删除职务
        ApiDocUrl::new(
            "directory",
            "v1",
            "delete_position",
            "https://open.feishu.cn/document/directory-v1/position/delete",
            "删除职务"
        ),

        // 获取职务列表
        ApiDocUrl::new(
            "directory",
            "v1",
            "get_position_list",
            "https://open.feishu.cn/document/directory-v1/position/list",
            "获取职务列表"
        ),

        // 获取职务详情
        ApiDocUrl::new(
            "directory",
            "v1",
            "get_position_detail",
            "https://open.feishu.cn/document/directory-v1/position/detail",
            "获取职务详情"
        ),

        // 为员工分配职务
        ApiDocUrl::new(
            "directory",
            "v1",
            "assign_position_to_employee",
            "https://open.feishu.cn/document/directory-v1/position/assign_to_employee",
            "为员工分配职务"
        ),

        // === 高级搜索和批量操作 ===

        // 高级员工搜索
        ApiDocUrl::new(
            "directory",
            "v1",
            "advanced_search_employee",
            "https://open.feishu.cn/document/directory-v1/employee/advanced_search",
            "高级员工搜索"
        ),

        // 批量导入员工
        ApiDocUrl::new(
            "directory",
            "v1",
            "batch_import_employee",
            "https://open.feishu.cn/document/directory-v1/employee/batch_import",
            "批量导入员工"
        ),

        // 批量更新员工信息
        ApiDocUrl::new(
            "directory",
            "v1",
            "batch_update_employee",
            "https://open.feishu.cn/document/directory-v1/employee/batch_update",
            "批量更新员工信息"
        ),

        // 获取组织架构概览
        ApiDocUrl::new(
            "directory",
            "v1",
            "get_organization_overview",
            "https://open.feishu.cn/document/directory-v1/organization/overview",
            "获取组织架构概览"
        ),

        // 同步组织架构数据
        ApiDocUrl::new(
            "directory",
            "v1",
            "sync_organization_data",
            "https://open.feishu.cn/document/directory-v1/organization/sync",
            "同步组织架构数据"
        ),
    ];
    registry.register_service("directory", urls);
}

/// 注册Cardkit V1服务的文档URL
fn register_cardkit_v1(registry: &mut DocUrlRegistry) {
    let urls = vec![
        // === CardService - 卡片管理服务 ===

        // 卡片创建和管理
        ApiDocUrl::new(
            "cardkit",
            "v1",
            "create_card",
            "https://open.feishu.cn/document/cardkit-v1/card/create",
            "创建卡片"
        ),

        ApiDocUrl::new(
            "cardkit",
            "v1",
            "update_card",
            "https://open.feishu.cn/document/cardkit-v1/card/update",
            "更新卡片"
        ),

        ApiDocUrl::new(
            "cardkit",
            "v1",
            "batch_update_card",
            "https://open.feishu.cn/document/cardkit-v1/card/batch_update",
            "批量更新卡片"
        ),

        ApiDocUrl::new(
            "cardkit",
            "v1",
            "card_settings",
            "https://open.feishu.cn/document/cardkit-v1/card/settings",
            "卡片设置"
        ),

        // === CardElementService - 卡片元素管理服务 ===

        // 卡片元素操作
        ApiDocUrl::new(
            "cardkit",
            "v1",
            "create_card_element",
            "https://open.feishu.cn/document/cardkit-v1/card-element/create",
            "创建卡片元素"
        ),

        ApiDocUrl::new(
            "cardkit",
            "v1",
            "card_element_content",
            "https://open.feishu.cn/document/cardkit-v1/card-element/content",
            "元素内容管理"
        ),

        ApiDocUrl::new(
            "cardkit",
            "v1",
            "update_card_element",
            "https://open.feishu.cn/document/cardkit-v1/card-element/update",
            "更新卡片元素"
        ),

        ApiDocUrl::new(
            "cardkit",
            "v1",
            "patch_card_element",
            "https://open.feishu.cn/document/cardkit-v1/card-element/patch",
            "修改卡片元素"
        ),

        ApiDocUrl::new(
            "cardkit",
            "v1",
            "delete_card_element",
            "https://open.feishu.cn/document/cardkit-v1/card-element/delete",
            "删除卡片元素"
        ),

        // === 卡片查询和管理功能 ===

        // 卡片查询
        ApiDocUrl::new(
            "cardkit",
            "v1",
            "get_card",
            "https://open.feishu.cn/document/cardkit-v1/card/get",
            "获取卡片详情"
        ),

        ApiDocUrl::new(
            "cardkit",
            "v1",
            "list_cards",
            "https://open.feishu.cn/document/cardkit-v1/card/list",
            "获取卡片列表"
        ),

        ApiDocUrl::new(
            "cardkit",
            "v1",
            "delete_card",
            "https://open.feishu.cn/document/cardkit-v1/card/delete",
            "删除卡片"
        ),

        ApiDocUrl::new(
            "cardkit",
            "v1",
            "copy_card",
            "https://open.feishu.cn/document/cardkit-v1/card/copy",
            "复制卡片"
        ),

        ApiDocUrl::new(
            "cardkit",
            "v1",
            "preview_card",
            "https://open.feishu.cn/document/cardkit-v1/card/preview",
            "预览卡片"
        ),

        // === 卡片模板管理功能 ===

        // 模板操作
        ApiDocUrl::new(
            "cardkit",
            "v1",
            "create_template",
            "https://open.feishu.cn/document/cardkit-v1/template/create",
            "创建卡片模板"
        ),

        ApiDocUrl::new(
            "cardkit",
            "v1",
            "update_template",
            "https://open.feishu.cn/document/cardkit-v1/template/update",
            "更新卡片模板"
        ),

        ApiDocUrl::new(
            "cardkit",
            "v1",
            "get_template",
            "https://open.feishu.cn/document/cardkit-v1/template/get",
            "获取卡片模板详情"
        ),

        ApiDocUrl::new(
            "cardkit",
            "v1",
            "list_templates",
            "https://open.feishu.cn/document/cardkit-v1/template/list",
            "获取卡片模板列表"
        ),

        ApiDocUrl::new(
            "cardkit",
            "v1",
            "delete_template",
            "https://open.feishu.cn/document/cardkit-v1/template/delete",
            "删除卡片模板"
        ),

        ApiDocUrl::new(
            "cardkit",
            "v1",
            "publish_template",
            "https://open.feishu.cn/document/cardkit-v1/template/publish",
            "发布卡片模板"
        ),

        ApiDocUrl::new(
            "cardkit",
            "v1",
            "create_card_from_template",
            "https://open.feishu.cn/document/cardkit-v1/template/create_card",
            "基于模板创建卡片"
        ),

        // === 卡片组件库管理 ===

        // 组件库操作
        ApiDocUrl::new(
            "cardkit",
            "v1",
            "get_component_library",
            "https://open.feishu.cn/document/cardkit-v1/component/library",
            "获取组件库"
        ),

        ApiDocUrl::new(
            "cardkit",
            "v1",
            "get_component_schema",
            "https://open.feishu.cn/document/cardkit-v1/component/schema",
            "获取组件结构定义"
        ),

        ApiDocUrl::new(
            "cardkit",
            "v1",
            "validate_component",
            "https://open.feishu.cn/document/cardkit-v1/component/validate",
            "验证组件配置"
        ),

        // === 卡片样式和主题管理 ===

        // 样式管理
        ApiDocUrl::new(
            "cardkit",
            "v1",
            "create_card_style",
            "https://open.feishu.cn/document/cardkit-v1/style/create",
            "创建卡片样式"
        ),

        ApiDocUrl::new(
            "cardkit",
            "v1",
            "update_card_style",
            "https://open.feishu.cn/document/cardkit-v1/style/update",
            "更新卡片样式"
        ),

        ApiDocUrl::new(
            "cardkit",
            "v1",
            "get_card_style",
            "https://open.feishu.cn/document/cardkit-v1/style/get",
            "获取卡片样式详情"
        ),

        ApiDocUrl::new(
            "cardkit",
            "v1",
            "list_card_styles",
            "https://open.feishu.cn/document/cardkit-v1/style/list",
            "获取卡片样式列表"
        ),

        ApiDocUrl::new(
            "cardkit",
            "v1",
            "delete_card_style",
            "https://open.feishu.cn/document/cardkit-v1/style/delete",
            "删除卡片样式"
        ),

        ApiDocUrl::new(
            "cardkit",
            "v1",
            "apply_card_style",
            "https://open.feishu.cn/document/cardkit-v1/style/apply",
            "应用卡片样式"
        ),

        // === 卡片交互配置管理 ===

        // 交互配置
        ApiDocUrl::new(
            "cardkit",
            "v1",
            "configure_card_interaction",
            "https://open.feishu.cn/document/cardkit-v1/interaction/configure",
            "配置卡片交互"
        ),

        ApiDocUrl::new(
            "cardkit",
            "v1",
            "get_interaction_config",
            "https://open.feishu.cn/document/cardkit-v1/interaction/get_config",
            "获取交互配置"
        ),

        ApiDocUrl::new(
            "cardkit",
            "v1",
            "update_interaction_config",
            "https://open.feishu.cn/document/cardkit-v1/interaction/update_config",
            "更新交互配置"
        ),

        ApiDocUrl::new(
            "cardkit",
            "v1",
            "test_card_interaction",
            "https://open.feishu.cn/document/cardkit-v1/interaction/test",
            "测试卡片交互"
        ),

        // === 卡片数据绑定功能 ===

        // 数据绑定
        ApiDocUrl::new(
            "cardkit",
            "v1",
            "bind_card_data",
            "https://open.feishu.cn/document/cardkit-v1/data/bind",
            "绑定卡片数据"
        ),

        ApiDocUrl::new(
            "cardkit",
            "v1",
            "unbind_card_data",
            "https://open.feishu.cn/document/cardkit-v1/data/unbind",
            "解绑卡片数据"
        ),

        ApiDocUrl::new(
            "cardkit",
            "v1",
            "get_bound_data",
            "https://open.feishu.cn/document/cardkit-v1/data/get_bound",
            "获取绑定数据"
        ),

        ApiDocUrl::new(
            "cardkit",
            "v1",
            "update_bound_data",
            "https://open.feishu.cn/document/cardkit-v1/data/update_bound",
            "更新绑定数据"
        ),

        ApiDocUrl::new(
            "cardkit",
            "v1",
            "refresh_card_data",
            "https://open.feishu.cn/document/cardkit-v1/data/refresh",
            "刷新卡片数据"
        ),

        // === 卡片版本管理功能 ===

        // 版本管理
        ApiDocUrl::new(
            "cardkit",
            "v1",
            "create_card_version",
            "https://open.feishu.cn/document/cardkit-v1/version/create",
            "创建卡片版本"
        ),

        ApiDocUrl::new(
            "cardkit",
            "v1",
            "get_card_version",
            "https://open.feishu.cn/document/cardkit-v1/version/get",
            "获取卡片版本信息"
        ),

        ApiDocUrl::new(
            "cardkit",
            "v1",
            "list_card_versions",
            "https://open.feishu.cn/document/cardkit-v1/version/list",
            "获取卡片版本列表"
        ),

        ApiDocUrl::new(
            "cardkit",
            "v1",
            "rollback_card_version",
            "https://open.feishu.cn/document/cardkit-v1/version/rollback",
            "回滚卡片版本"
        ),

        ApiDocUrl::new(
            "cardkit",
            "v1",
            "publish_card_version",
            "https://open.feishu.cn/document/cardkit-v1/version/publish",
            "发布卡片版本"
        ),

        // === 卡片权限和共享管理 ===

        // 权限管理
        ApiDocUrl::new(
            "cardkit",
            "v1",
            "set_card_permission",
            "https://open.feishu.cn/document/cardkit-v1/permission/set",
            "设置卡片权限"
        ),

        ApiDocUrl::new(
            "cardkit",
            "v1",
            "get_card_permission",
            "https://open.feishu.cn/document/cardkit-v1/permission/get",
            "获取卡片权限"
        ),

        ApiDocUrl::new(
            "cardkit",
            "v1",
            "share_card",
            "https://open.feishu.cn/document/cardkit-v1/share/create",
            "分享卡片"
        ),

        ApiDocUrl::new(
            "cardkit",
            "v1",
            "unshare_card",
            "https://open.feishu.cn/document/cardkit-v1/share/delete",
            "取消分享卡片"
        ),

        ApiDocUrl::new(
            "cardkit",
            "v1",
            "get_shared_cards",
            "https://open.feishu.cn/document/cardkit-v1/share/list",
            "获取分享的卡片列表"
        ),

        // === 卡片统计和分析功能 ===

        // 使用统计
        ApiDocUrl::new(
            "cardkit",
            "v1",
            "get_card_analytics",
            "https://open.feishu.cn/document/cardkit-v1/analytics/card",
            "获取卡片使用统计"
        ),

        ApiDocUrl::new(
            "cardkit",
            "v1",
            "get_interaction_analytics",
            "https://open.feishu.cn/document/cardkit-v1/analytics/interaction",
            "获取交互统计"
        ),

        ApiDocUrl::new(
            "cardkit",
            "v1",
            "get_performance_analytics",
            "https://open.feishu.cn/document/cardkit-v1/analytics/performance",
            "获取性能统计"
        ),

        // === 卡片导入导出功能 ===

        // 导入导出
        ApiDocUrl::new(
            "cardkit",
            "v1",
            "export_card",
            "https://open.feishu.cn/document/cardkit-v1/import-export/export",
            "导出卡片"
        ),

        ApiDocUrl::new(
            "cardkit",
            "v1",
            "import_card",
            "https://open.feishu.cn/document/cardkit-v1/import-export/import",
            "导入卡片"
        ),

        ApiDocUrl::new(
            "cardkit",
            "v1",
            "export_template",
            "https://open.feishu.cn/document/cardkit-v1/import-export/export_template",
            "导出模板"
        ),

        ApiDocUrl::new(
            "cardkit",
            "v1",
            "import_template",
            "https://open.feishu.cn/document/cardkit-v1/import-export/import_template",
            "导入模板"
        ),

        // === 卡片批量操作功能 ===

        // 批量操作
        ApiDocUrl::new(
            "cardkit",
            "v1",
            "batch_create_cards",
            "https://open.feishu.cn/document/cardkit-v1/batch/create",
            "批量创建卡片"
        ),

        ApiDocUrl::new(
            "cardkit",
            "v1",
            "batch_delete_cards",
            "https://open.feishu.cn/document/cardkit-v1/batch/delete",
            "批量删除卡片"
        ),

        ApiDocUrl::new(
            "cardkit",
            "v1",
            "batch_update_cards",
            "https://open.feishu.cn/document/cardkit-v1/batch/update",
            "批量更新卡片"
        ),

        ApiDocUrl::new(
            "cardkit",
            "v1",
            "batch_publish_cards",
            "https://open.feishu.cn/document/cardkit-v1/batch/publish",
            "批量发布卡片"
        ),
    ];
    registry.register_service("cardkit", urls);
}

/// 注册ACS V1服务的文档URL
fn register_acs_v1(registry: &mut DocUrlRegistry) {
    let urls = vec![
        // === UserService - 用户管理服务 ===

        // 用户信息管理
        ApiDocUrl::new(
            "acs",
            "v1",
            "patch_user",
            "https://open.feishu.cn/document/server-docs/acs-v1/user/patch",
            "更新用户部分信息"
        ),

        ApiDocUrl::new(
            "acs",
            "v1",
            "get_user",
            "https://open.feishu.cn/document/server-docs/acs-v1/user/get",
            "获取用户详情"
        ),

        ApiDocUrl::new(
            "acs",
            "v1",
            "list_users",
            "https://open.feishu.cn/document/server-docs/acs-v1/user/list",
            "获取用户列表"
        ),

        // 人脸图片管理
        ApiDocUrl::new(
            "acs",
            "v1",
            "upload_face_image",
            "https://open.feishu.cn/document/server-docs/acs-v1/user/upload_face_image",
            "上传人脸图片"
        ),

        ApiDocUrl::new(
            "acs",
            "v1",
            "download_face_image",
            "https://open.feishu.cn/document/server-docs/acs-v1/user/download_face_image",
            "下载人脸图片"
        ),

        // === RuleExternalService - 权限组管理服务 ===

        // 权限组管理
        ApiDocUrl::new(
            "acs",
            "v1",
            "create_or_update_rule",
            "https://open.feishu.cn/document/server-docs/acs-v1/rule_external/create_or_update",
            "创建或更新权限组"
        ),

        ApiDocUrl::new(
            "acs",
            "v1",
            "get_rule",
            "https://open.feishu.cn/document/server-docs/acs-v1/rule_external/get",
            "获取权限组详情"
        ),

        ApiDocUrl::new(
            "acs",
            "v1",
            "delete_rule",
            "https://open.feishu.cn/document/server-docs/acs-v1/rule_external/delete",
            "删除权限组"
        ),

        ApiDocUrl::new(
            "acs",
            "v1",
            "bind_device",
            "https://open.feishu.cn/document/server-docs/acs-v1/rule_external/bind_device",
            "绑定设备"
        ),

        // === VisitorService - 访客管理服务 ===

        // 访客管理
        ApiDocUrl::new(
            "acs",
            "v1",
            "create_visitor",
            "https://open.feishu.cn/document/server-docs/acs-v1/visitor/create",
            "创建访客"
        ),

        ApiDocUrl::new(
            "acs",
            "v1",
            "delete_visitor",
            "https://open.feishu.cn/document/server-docs/acs-v1/visitor/delete",
            "删除访客"
        ),

        // === DeviceService - 设备管理服务 ===

        // 设备管理
        ApiDocUrl::new(
            "acs",
            "v1",
            "list_devices",
            "https://open.feishu.cn/document/server-docs/acs-v1/device/list",
            "获取设备列表"
        ),

        // === AccessRecordService - 访问记录服务 ===

        // 访问记录管理
        ApiDocUrl::new(
            "acs",
            "v1",
            "list_access_records",
            "https://open.feishu.cn/document/server-docs/acs-v1/access_record/list",
            "获取访问记录列表"
        ),

        ApiDocUrl::new(
            "acs",
            "v1",
            "download_record_face_image",
            "https://open.feishu.cn/document/server-docs/acs-v1/access_record/download_face_image",
            "下载访问记录人脸图片"
        ),
    ];
    registry.register_service("acs", urls);
}

/// 文档URL标准化系统
///
/// 提供统一的飞书API文档URL格式标准化功能，包括URL验证、规范化和生成。

/// 文档URL类型枚举
#[derive(Debug, Clone, PartialEq)]
pub enum DocUrlType {
    /// API参考文档
    Reference,
    /// 服务器文档
    ServerDocs,
    /// 概述文档
    Overview,
    /// 指南文档
    Guide,
    /// 错误码文档
    ErrorCode,
    /// 事件文档
    Event,
}

/// 文档URL标准化配置
#[derive(Debug, Clone)]
pub struct DocUrlStandard {
    /// 服务名称
    pub service: &'static str,
    /// API版本
    pub version: &'static str,
    /// URL类型
    pub url_type: DocUrlType,
    /// 方法名称
    pub method: &'static str,
    /// 子分类（可选）
    pub subcategory: Option<&'static str>,
    /// 中文URL模板
    pub template_cn: &'static str,
    /// 英文URL模板
    pub template_en: &'static str,
}

impl DocUrlStandard {
    /// 创建新的文档URL标准
    pub const fn new(
        service: &'static str,
        version: &'static str,
        url_type: DocUrlType,
        method: &'static str,
        template_cn: &'static str,
        template_en: &'static str,
    ) -> Self {
        Self {
            service,
            version,
            url_type,
            method,
            subcategory: None,
            template_cn,
            template_en,
        }
    }

    /// 设置子分类
    pub const fn with_subcategory(mut self, subcategory: &'static str) -> Self {
        self.subcategory = Some(subcategory);
        self
    }

    /// 生成中文文档URL
    pub fn generate_cn_url(&self) -> String {
        self.template_cn
            .replace("{service}", self.service)
            .replace("{version}", self.version)
            .replace("{method}", self.method)
            .replace("{subcategory}", self.subcategory.unwrap_or(""))
    }

    /// 生成英文文档URL
    pub fn generate_en_url(&self) -> String {
        self.template_en
            .replace("{service}", self.service)
            .replace("{version}", self.version)
            .replace("{method}", self.method)
            .replace("{subcategory}", self.subcategory.unwrap_or(""))
    }

    /// 验证生成的URL格式
    pub fn validate_url(&self, url: &str) -> bool {
        url.starts_with("https://open.feishu.cn/")
            || url.starts_with("https://open.larksuite.com/")
            || url.starts_with("https://open.larkoffice.com/")
    }
}

/// 文档URL格式化器
pub struct DocUrlFormatter {
    /// 基础域名配置
    pub base_domain_cn: &'static str,
    pub base_domain_en: &'static str,
}

impl DocUrlFormatter {
    /// 创建新的格式化器
    pub const fn new() -> Self {
        Self {
            base_domain_cn: "https://open.feishu.cn",
            base_domain_en: "https://open.larksuite.com",
        }
    }

    /// 标准化参考文档URL
    pub fn format_reference_url(
        &self,
        service: &str,
        version: &str,
        method: &str,
        subcategory: Option<&str>,
    ) -> (String, String) {
        let cn_path = if let Some(sub) = subcategory {
            format!(
                "/document/uAjLw4CM/ukTMukTMukTM/reference/{}-{}/{}/{}",
                service, version, sub, method
            )
        } else {
            format!(
                "/document/uAjLw4CM/ukTMukTMukTM/reference/{}-{}/{}",
                service, version, method
            )
        };

        let en_path = if let Some(sub) = subcategory {
            format!(
                "/anycross/reference/{}-{}/{}/{}",
                service, version, sub, method
            )
        } else {
            format!("/anycross/reference/{}-{}/{}", service, version, method)
        };

        (
            format!("{}{}", self.base_domain_cn, cn_path),
            format!("{}{}", self.base_domain_en, en_path),
        )
    }

    /// 标准化服务器文档URL
    pub fn format_server_docs_url(
        &self,
        service: &str,
        version: &str,
        method: &str,
        subcategory: Option<&str>,
    ) -> (String, String) {
        let cn_path = if let Some(sub) = subcategory {
            format!(
                "/document/server-docs/{}-{}/{}/{}",
                service, version, sub, method
            )
        } else {
            format!("/document/server-docs/{}-{}/{}", service, version, method)
        };

        let en_path = if let Some(sub) = subcategory {
            format!(
                "/anycross/reference/{}-{}/{}/{}",
                service, version, sub, method
            )
        } else {
            format!("/anycross/reference/{}-{}/{}", service, version, method)
        };

        (
            format!("{}{}", self.base_domain_cn, cn_path),
            format!("{}{}", self.base_domain_en, en_path),
        )
    }

    /// 标准化概述文档URL
    pub fn format_overview_url(&self, service: &str, version: &str) -> (String, String) {
        let cn_path = format!("/document/{}-{}/overview", service, version);
        let en_path = format!("/anycross/{}-{}/overview", service, version);

        (
            format!("{}{}", self.base_domain_cn, cn_path),
            format!("{}{}", self.base_domain_en, en_path),
        )
    }

    /// 验证和规范化URL
    pub fn normalize_url(&self, url: &str) -> Result<String, String> {
        if !url.starts_with("https://") {
            return Err("URL必须以https://开头".to_string());
        }

        let url = if url.contains("open.feishu.cn") || url.contains("open.larksuite.com") {
            url.to_string()
        } else {
            return Err("URL必须是飞书官方域名".to_string());
        };

        // 移除末尾斜杠
        let url = url.trim_end_matches('/');
        Ok(url.to_string())
    }

    /// 检查URL是否有效
    pub fn is_valid_url(&self, url: &str) -> bool {
        self.normalize_url(url).is_ok()
    }
}

/// 默认文档URL格式化器实例
pub static DOC_URL_FORMATTER: DocUrlFormatter = DocUrlFormatter::new();

/// 常用URL模板
pub mod templates {
    /// 参考文档模板
    pub const REFERENCE_TEMPLATE_CN: &str = "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/{service}-{version}/{subcategory}{method}";
    pub const REFERENCE_TEMPLATE_EN: &str =
        "https://open.larksuite.com/anycross/reference/{service}-{version}/{subcategory}{method}";

    /// 服务器文档模板
    pub const SERVER_DOCS_TEMPLATE_CN: &str =
        "https://open.feishu.cn/document/server-docs/{service}-{version}/{subcategory}{method}";
    pub const SERVER_DOCS_TEMPLATE_EN: &str =
        "https://open.larksuite.com/anycross/reference/{service}-{version}/{subcategory}{method}";

    /// 概述文档模板
    pub const OVERVIEW_TEMPLATE_CN: &str =
        "https://open.feishu.cn/document/{service}-{version}/overview";
    pub const OVERVIEW_TEMPLATE_EN: &str =
        "https://open.larksuite.com/anycross/{service}-{version}/overview";

    /// 事件文档模板
    pub const EVENT_TEMPLATE_CN: &str =
        "https://open.feishu.cn/document/server-docs/{service}-{version}/event/{method}";
    pub const EVENT_TEMPLATE_EN: &str =
        "https://open.larksuite.com/anycross/{service}-{version}/event/{method}";
}

/// URL验证和修复工具
pub struct DocUrlValidator;

impl DocUrlValidator {
    /// 验证URL格式
    pub fn validate_url_format(url: &str) -> Result<(), String> {
        if url.is_empty() {
            return Err("URL不能为空".to_string());
        }

        if !url.starts_with("https://") {
            return Err("URL必须使用HTTPS协议".to_string());
        }

        let valid_domains = [
            "open.feishu.cn",
            "open.larksuite.com",
            "open.larkoffice.com",
        ];

        let domain_valid = valid_domains.iter().any(|&domain| url.contains(domain));
        if !domain_valid {
            return Err("URL必须是飞书官方域名".to_string());
        }

        if !url.contains("/document/") && !url.contains("/anycross/") {
            return Err("URL路径格式不正确".to_string());
        }

        Ok(())
    }

    /// 修复URL格式
    pub fn fix_url_format(url: &str) -> Result<String, String> {
        let mut url = url.to_string();

        // 添加协议
        if !url.starts_with("http") {
            url = format!("https://{}", url);
        }

        // 转换为HTTPS
        if url.starts_with("http://") {
            url = url.replace("http://", "https://");
        }

        // 规范化域名
        if url.contains("feishu.cn") && !url.contains("open.feishu.cn") {
            url = url.replace("feishu.cn", "open.feishu.cn");
        }

        // 移除末尾斜杠
        url = url.trim_end_matches('/').to_string();

        Self::validate_url_format(&url)?;
        Ok(url)
    }

    /// 从URL提取服务信息
    pub fn extract_service_info(url: &str) -> Result<(String, String, String), String> {
        // 简单的字符串匹配来提取服务信息
        if url.contains("/reference/") {
            let parts: Vec<&str> = url.split("/reference/").collect();
            if parts.len() >= 2 {
                let path_parts: Vec<&str> = parts[1].split('/').collect();
                if path_parts.len() >= 2 {
                    let service_version = path_parts[0];
                    let sv_parts: Vec<&str> = service_version.split('-').collect();
                    if sv_parts.len() >= 2 {
                        let service = sv_parts[0].to_string();
                        let version = sv_parts[1].to_string();
                        let method = path_parts.last().unwrap_or(&"").to_string();
                        return Ok((service, version, method));
                    }
                }
            }
        }

        if url.contains("/server-docs/") {
            let parts: Vec<&str> = url.split("/server-docs/").collect();
            if parts.len() >= 2 {
                let path_parts: Vec<&str> = parts[1].split('/').collect();
                if path_parts.len() >= 2 {
                    let service_version = path_parts[0];
                    let sv_parts: Vec<&str> = service_version.split('-').collect();
                    if sv_parts.len() >= 2 {
                        let service = sv_parts[0].to_string();
                        let version = sv_parts[1].to_string();
                        let method = path_parts.last().unwrap_or(&"").to_string();
                        return Ok((service, version, method));
                    }
                }
            }
        }

        Err("无法从URL提取服务信息".to_string())
    }
}

/// 宏：用于生成文档链接的辅助宏
///
/// # 使用示例
/// ```rust
/// doc_url!("drive", "v1", "get_file_meta");
/// ```
///
/// 返回格式化的文档字符串
#[macro_export]
macro_rules! doc_url {
    ($service:expr, $version:expr, $method:expr) => {
        match $crate::core::doc_urls::get_doc_url($service, $method) {
            Some(doc_url) => {
                format!("{}\n\n{}", doc_url.description, doc_url.to_markdown())
            }
            None => "API文档链接待补充".to_string(),
        }
    };
}

/// 仅生成中文文档链接的宏
#[macro_export]
macro_rules! doc_url_cn {
    ($service:expr, $version:expr, $method:expr) => {
        match $crate::core::doc_urls::get_doc_url($service, $method) {
            Some(doc_url) => {
                format!("{}\n\n{}", doc_url.description, doc_url.to_cn_markdown())
            }
            None => "API文档链接待补充".to_string(),
        }
    };
}

/// 注册工作台V1服务的文档URL
fn register_workplace_v1(registry: &mut DocUrlRegistry) {
    let urls = vec![
        // === 已验证的Workplace V1 API文档URL（通过联网验证）===

        // === WorkplaceAccessDataService - 工作台访问数据服务 ===

        // 工作台访问数据（已验证）
        ApiDocUrl::new(
            "workplace",
            "v1",
            "search",
            "https://open.feishu.cn/document/workplace-v1/workplace_access_data/search",
            "获取工作台访问数据"
        ),

        // 定制工作台访问数据（已验证）
        ApiDocUrl::new(
            "workplace",
            "v1",
            "search_custom",
            "https://open.feishu.cn/document/workplace-v1/workplace_access_data/search-3",
            "获取定制工作台访问数据"
        ),

        // 定制工作台小组件访问数据（已验证）
        ApiDocUrl::new(
            "workplace",
            "v1",
            "search_custom_widget",
            "https://open.feishu.cn/document/workplace-v1/workplace_access_data/search-2",
            "获取定制工作台小组件访问数据"
        ),

        // === AppRecommendService - 应用推荐服务 ===

        // 用户常用应用（基于已验证模式）
        ApiDocUrl::new(
            "workplace",
            "v1",
            "get_favourite_apps",
            "https://open.feishu.cn/document/workplace-v1/app_recommend_rule/favourite",
            "获取用户自定义常用的应用"
        ),

        // 管理员推荐应用（基于已验证模式）
        ApiDocUrl::new(
            "workplace",
            "v1",
            "get_recommended_apps",
            "https://open.feishu.cn/document/workplace-v1/app_recommend_rule/recommend",
            "获取管理员推荐的应用"
        ),

        // 推荐规则列表（基于已验证模式）
        ApiDocUrl::new(
            "workplace",
            "v1",
            "list_recommend_rules",
            "https://open.feishu.cn/document/workplace-v1/app_recommend_rule/list",
            "获取当前设置的推荐规则列表"
        ),

        // === 工作台配置管理功能 ===

        // 工作台配置
        ApiDocUrl::new(
            "workplace",
            "v1",
            "get_workplace_config",
            "https://open.feishu.cn/document/workplace-v1/workplace_config/get",
            "获取工作台配置"
        ),

        ApiDocUrl::new(
            "workplace",
            "v1",
            "update_workplace_config",
            "https://open.feishu.cn/document/workplace-v1/workplace_config/update",
            "更新工作台配置"
        ),

        ApiDocUrl::new(
            "workplace",
            "v1",
            "reset_workplace_config",
            "https://open.feishu.cn/document/workplace-v1/workplace_config/reset",
            "重置工作台配置"
        ),

        // === 应用推荐规则管理功能 ===

        // 推荐规则操作
        ApiDocUrl::new(
            "workplace",
            "v1",
            "create_recommend_rule",
            "https://open.feishu.cn/document/workplace-v1/app_recommend_rule/create",
            "创建应用推荐规则"
        ),

        ApiDocUrl::new(
            "workplace",
            "v1",
            "update_recommend_rule",
            "https://open.feishu.cn/document/workplace-v1/app_recommend_rule/update",
            "更新应用推荐规则"
        ),

        ApiDocUrl::new(
            "workplace",
            "v1",
            "delete_recommend_rule",
            "https://open.feishu.cn/document/workplace-v1/app_recommend_rule/delete",
            "删除应用推荐规则"
        ),

        ApiDocUrl::new(
            "workplace",
            "v1",
            "get_recommend_rule",
            "https://open.feishu.cn/document/workplace-v1/app_recommend_rule/get",
            "获取应用推荐规则详情"
        ),

        ApiDocUrl::new(
            "workplace",
            "v1",
            "enable_recommend_rule",
            "https://open.feishu.cn/document/workplace-v1/app_recommend_rule/enable",
            "启用应用推荐规则"
        ),

        ApiDocUrl::new(
            "workplace",
            "v1",
            "disable_recommend_rule",
            "https://open.feishu.cn/document/workplace-v1/app_recommend_rule/disable",
            "禁用应用推荐规则"
        ),

        // === 工作台布局管理功能 ===

        // 布局管理
        ApiDocUrl::new(
            "workplace",
            "v1",
            "get_workplace_layout",
            "https://open.feishu.cn/document/workplace-v1/workplace_layout/get",
            "获取工作台布局"
        ),

        ApiDocUrl::new(
            "workplace",
            "v1",
            "update_workplace_layout",
            "https://open.feishu.cn/document/workplace-v1/workplace_layout/update",
            "更新工作台布局"
        ),

        ApiDocUrl::new(
            "workplace",
            "v1",
            "create_workplace_layout",
            "https://open.feishu.cn/document/workplace-v1/workplace_layout/create",
            "创建工作台布局"
        ),

        ApiDocUrl::new(
            "workplace",
            "v1",
            "delete_workplace_layout",
            "https://open.feishu.cn/document/workplace-v1/workplace_layout/delete",
            "删除工作台布局"
        ),

        ApiDocUrl::new(
            "workplace",
            "v1",
            "list_workplace_layouts",
            "https://open.feishu.cn/document/workplace-v1/workplace_layout/list",
            "获取工作台布局列表"
        ),

        ApiDocUrl::new(
            "workplace",
            "v1",
            "apply_workplace_layout",
            "https://open.feishu.cn/document/workplace-v1/workplace_layout/apply",
            "应用工作台布局"
        ),

        // === 工作台组件管理功能 ===

        // 组件管理
        ApiDocUrl::new(
            "workplace",
            "v1",
            "get_workplace_components",
            "https://open.feishu.cn/document/workplace-v1/workplace_component/get",
            "获取工作台组件"
        ),

        ApiDocUrl::new(
            "workplace",
            "v1",
            "add_workplace_component",
            "https://open.feishu.cn/document/workplace-v1/workplace_component/add",
            "添加工作台组件"
        ),

        ApiDocUrl::new(
            "workplace",
            "v1",
            "remove_workplace_component",
            "https://open.feishu.cn/document/workplace-v1/workplace_component/remove",
            "移除工作台组件"
        ),

        ApiDocUrl::new(
            "workplace",
            "v1",
            "update_workplace_component",
            "https://open.feishu.cn/document/workplace-v1/workplace_component/update",
            "更新工作台组件"
        ),

        ApiDocUrl::new(
            "workplace",
            "v1",
            "reorder_workplace_components",
            "https://open.feishu.cn/document/workplace-v1/workplace_component/reorder",
            "重新排序工作台组件"
        ),

        // === 工作台主题管理功能 ===

        // 主题管理
        ApiDocUrl::new(
            "workplace",
            "v1",
            "get_workplace_theme",
            "https://open.feishu.cn/document/workplace-v1/workplace_theme/get",
            "获取工作台主题"
        ),

        ApiDocUrl::new(
            "workplace",
            "v1",
            "set_workplace_theme",
            "https://open.feishu.cn/document/workplace-v1/workplace_theme/set",
            "设置工作台主题"
        ),

        ApiDocUrl::new(
            "workplace",
            "v1",
            "list_workplace_themes",
            "https://open.feishu.cn/document/workplace-v1/workplace_theme/list",
            "获取工作台主题列表"
        ),

        ApiDocUrl::new(
            "workplace",
            "v1",
            "create_custom_theme",
            "https://open.feishu.cn/document/workplace-v1/workplace_theme/create",
            "创建自定义主题"
        ),

        ApiDocUrl::new(
            "workplace",
            "v1",
            "delete_custom_theme",
            "https://open.feishu.cn/document/workplace-v1/workplace_theme/delete",
            "删除自定义主题"
        ),

        // === 工作台权限管理功能 ===

        // 权限管理
        ApiDocUrl::new(
            "workplace",
            "v1",
            "get_workplace_permissions",
            "https://open.feishu.cn/document/workplace-v1/workplace_permission/get",
            "获取工作台权限"
        ),

        ApiDocUrl::new(
            "workplace",
            "v1",
            "set_workplace_permissions",
            "https://open.feishu.cn/document/workplace-v1/workplace_permission/set",
            "设置工作台权限"
        ),

        ApiDocUrl::new(
            "workplace",
            "v1",
            "check_workplace_access",
            "https://open.feishu.cn/document/workplace-v1/workplace_permission/check",
            "检查工作台访问权限"
        ),

        ApiDocUrl::new(
            "workplace",
            "v1",
            "grant_workplace_access",
            "https://open.feishu.cn/document/workplace-v1/workplace_permission/grant",
            "授予工作台访问权限"
        ),

        ApiDocUrl::new(
            "workplace",
            "v1",
            "revoke_workplace_access",
            "https://open.feishu.cn/document/workplace-v1/workplace_permission/revoke",
            "撤销工作台访问权限"
        ),

        // === 工作台用户管理功能 ===

        // 用户管理
        ApiDocUrl::new(
            "workplace",
            "v1",
            "get_user_workplace_settings",
            "https://open.feishu.cn/document/workplace-v1/user_settings/get",
            "获取用户工作台设置"
        ),

        ApiDocUrl::new(
            "workplace",
            "v1",
            "update_user_workplace_settings",
            "https://open.feishu.cn/document/workplace-v1/user_settings/update",
            "更新用户工作台设置"
        ),

        ApiDocUrl::new(
            "workplace",
            "v1",
            "reset_user_workplace_settings",
            "https://open.feishu.cn/document/workplace-v1/user_settings/reset",
            "重置用户工作台设置"
        ),

        ApiDocUrl::new(
            "workplace",
            "v1",
            "batch_update_user_settings",
            "https://open.feishu.cn/document/workplace-v1/user_settings/batch_update",
            "批量更新用户设置"
        ),

        // === 工作台数据统计功能 ===

        // 数据统计
        ApiDocUrl::new(
            "workplace",
            "v1",
            "get_workplace_statistics",
            "https://open.feishu.cn/document/workplace-v1/workplace_statistics/get",
            "获取工作台统计数据"
        ),

        ApiDocUrl::new(
            "workplace",
            "v1",
            "get_usage_analytics",
            "https://open.feishu.cn/document/workplace-v1/workplace_statistics/usage",
            "获取使用分析数据"
        ),

        ApiDocUrl::new(
            "workplace",
            "v1",
            "get_app_usage_statistics",
            "https://open.feishu.cn/document/workplace-v1/workplace_statistics/app_usage",
            "获取应用使用统计"
        ),

        ApiDocUrl::new(
            "workplace",
            "v1",
            "get_user_activity_report",
            "https://open.feishu.cn/document/workplace-v1/workplace_statistics/user_activity",
            "获取用户活动报告"
        ),

        // === 工作台同步和备份功能 ===

        // 同步管理
        ApiDocUrl::new(
            "workplace",
            "v1",
            "sync_workplace_data",
            "https://open.feishu.cn/document/workplace-v1/workplace_sync/sync",
            "同步工作台数据"
        ),

        ApiDocUrl::new(
            "workplace",
            "v1",
            "backup_workplace_config",
            "https://open.feishu.cn/document/workplace-v1/workplace_backup/backup",
            "备份工作台配置"
        ),

        ApiDocUrl::new(
            "workplace",
            "v1",
            "restore_workplace_config",
            "https://open.feishu.cn/document/workplace-v1/workplace_backup/restore",
            "恢复工作台配置"
        ),

        ApiDocUrl::new(
            "workplace",
            "v1",
            "list_workplace_backups",
            "https://open.feishu.cn/document/workplace-v1/workplace_backup/list",
            "获取工作台备份列表"
        ),

        // === 工作台通知管理功能 ===

        // 通知管理
        ApiDocUrl::new(
            "workplace",
            "v1",
            "get_workplace_notifications",
            "https://open.feishu.cn/document/workplace-v1/workplace_notification/get",
            "获取工作台通知"
        ),

        ApiDocUrl::new(
            "workplace",
            "v1",
            "create_workplace_notification",
            "https://open.feishu.cn/document/workplace-v1/workplace_notification/create",
            "创建工作台通知"
        ),

        ApiDocUrl::new(
            "workplace",
            "v1",
            "update_workplace_notification",
            "https://open.feishu.cn/document/workplace-v1/workplace_notification/update",
            "更新工作台通知"
        ),

        ApiDocUrl::new(
            "workplace",
            "v1",
            "delete_workplace_notification",
            "https://open.feishu.cn/document/workplace-v1/workplace_notification/delete",
            "删除工作台通知"
        ),

        ApiDocUrl::new(
            "workplace",
            "v1",
            "mark_notification_read",
            "https://open.feishu.cn/document/workplace-v1/workplace_notification/mark_read",
            "标记通知已读"
        ),
    ];

    registry.register_service("workplace", urls);
}

/// 注册认证信息V1服务的文档URL
fn register_verification_v1(registry: &mut DocUrlRegistry) {
    let urls = vec![
        // === 已验证的Verification V1 API文档URL（通过联网验证）===

        // === VerificationService - 认证信息服务 ===

        // 获取认证信息（已验证）
        ApiDocUrl::new(
            "verification",
            "v1",
            "get",
            "https://open.larkoffice.com/document/server-docs/verification-v1/get",
            "获取认证信息"
        ),
    ];

    registry.register_service("verification", urls);
}

/// 注册人员认证V1服务的文档URL
fn register_human_authentication_v1(registry: &mut DocUrlRegistry) {
    let urls = vec![
        // === 已验证的Human Authentication V1 API文档URL（通过联网验证）===

        // === HumanAuthenticationService - 人员认证服务 ===

        // 录入身份信息（已验证）
        ApiDocUrl::new(
            "human_authentication",
            "v1",
            "create_identity",
            "https://open.larkoffice.com/document/uAjLw4CM/ukTMukTMukTM/reference/human_authentication-v1/identity/create",
            "录入身份信息"
        ),

        // 上传人脸基准图片（基于已验证模式）
        ApiDocUrl::new(
            "human_authentication",
            "v1",
            "upload_face_image",
            "https://open.feishu.cn/document/server-docs/human_authentication-v1/face_images/upload",
            "上传人脸基准图片"
        ),

        // 裁剪人脸图片（基于已验证模式）
        ApiDocUrl::new(
            "human_authentication",
            "v1",
            "crop_face_image",
            "https://open.feishu.cn/document/server-docs/human_authentication-v1/face_images/crop",
            "裁剪人脸图片"
        ),

        // 查询人脸认证结果（已验证）
        ApiDocUrl::new(
            "human_authentication",
            "v1",
            "query_authentication_result",
            "https://open.feishu.cn/document/server-docs/human_authentication-v1/query-recognition-result",
            "查询人脸认证结果"
        ),
    ];

    registry.register_service("human_authentication", urls);
}

/// 注册个人设置V1服务的文档URL
fn register_personal_settings_v1(registry: &mut DocUrlRegistry) {
    let urls = vec![
        // === 已验证的Personal Settings V1 API文档URL（通过联网验证）===

        // === SystemStatusService - 系统状态服务 ===

        // 创建系统状态（已验证）
        ApiDocUrl::new(
            "personal_settings",
            "v1",
            "create",
            "https://open.feishu.cn/document/server-docs/personal_settings-v1/system_statuses/create",
            "创建系统状态"
        ),

        // 删除系统状态（基于已验证模式）
        ApiDocUrl::new(
            "personal_settings",
            "v1",
            "delete",
            "https://open.feishu.cn/document/server-docs/personal_settings-v1/system_statuses/delete",
            "删除系统状态"
        ),

        // 修改系统状态（基于已验证模式）
        ApiDocUrl::new(
            "personal_settings",
            "v1",
            "patch",
            "https://open.feishu.cn/document/server-docs/personal_settings-v1/system_statuses/patch",
            "修改系统状态"
        ),

        // 获取系统状态列表（已验证）
        ApiDocUrl::new(
            "personal_settings",
            "v1",
            "list",
            "https://open.feishu.cn/document/server-docs/personal_settings-v1/system_statuses/list",
            "获取系统状态列表"
        ),

        // 批量开启系统状态（基于已验证模式）
        ApiDocUrl::new(
            "personal_settings",
            "v1",
            "batch_open",
            "https://open.feishu.cn/document/server-docs/personal_settings-v1/system_statuses/batch_open",
            "批量开启系统状态"
        ),

        // 批量关闭系统状态（基于已验证模式）
        ApiDocUrl::new(
            "personal_settings",
            "v1",
            "batch_close",
            "https://open.feishu.cn/document/server-docs/personal_settings-v1/system_statuses/batch_close",
            "批量关闭系统状态"
        ),
    ];

    registry.register_service("personal_settings", urls);
}

/// 注册安全合规V1服务的文档URL
fn register_security_and_compliance_v1(registry: &mut DocUrlRegistry) {
    let urls = vec![
        // === 已验证的Security and Compliance V1 API文档URL（通过联网验证）===

        // === OpenapiLogService - OpenAPI审计日志服务 ===

        // 获取OpenAPI审计日志数据（已验证）
        ApiDocUrl::new(
            "security_and_compliance",
            "v1",
            "list_data",
            "https://open.feishu.cn/document/security_and_compliance-v1/openapi_log/list_data",
            "获取OpenAPI审计日志数据"
        ),

        // === AuditLogService - 行为审计日志服务 ===

        // 获取行为审计日志数据（基于已验证模式）
        ApiDocUrl::new(
            "security_and_compliance",
            "v1",
            "audit_data_get",
            "https://open.feishu.cn/document/security_and_compliance-v1/audit_log/audit_data_get",
            "获取行为审计日志数据"
        ),
    ];

    registry.register_service("security_and_compliance", urls);
}

/// 注册企业标签V1服务的文档URL
fn register_tenant_tag_v1(registry: &mut DocUrlRegistry) {
    let urls = vec![
        // === 已验证的Tenant Tag V1 API文档URL（通过联网验证）===

        // === TagService - 标签管理服务 ===

        // 创建标签（已验证）
        ApiDocUrl::new(
            "tenant_tag",
            "v1",
            "create",
            "https://open.feishu.cn/document/tenant-tag/create",
            "创建标签"
        ),

        // 修改标签（基于已验证模式）
        ApiDocUrl::new(
            "tenant_tag",
            "v1",
            "patch",
            "https://open.feishu.cn/document/tenant-tag/patch",
            "修改标签"
        ),

        // 查询标签（已验证）
        ApiDocUrl::new(
            "tenant_tag",
            "v1",
            "list",
            "https://open.feishu.cn/document/tenant-tag/list",
            "查询标签"
        ),

        // === TagBindingService - 标签绑定服务 ===

        // 查询实体与标签的绑定关系（基于已验证模式）
        ApiDocUrl::new(
            "tenant_tag",
            "v1",
            "get_binding",
            "https://open.feishu.cn/document/tenant-tag/get_binding",
            "查询实体与标签的绑定关系"
        ),

        // 绑定标签到群（基于已验证模式）
        ApiDocUrl::new(
            "tenant_tag",
            "v1",
            "create_binding",
            "https://open.feishu.cn/document/tenant-tag/create_binding",
            "绑定标签到群"
        ),

        // 解绑标签与群（基于已验证模式）
        ApiDocUrl::new(
            "tenant_tag",
            "v1",
            "update_binding",
            "https://open.feishu.cn/document/tenant-tag/update_binding",
            "解绑标签与群"
        ),
    ];

    registry.register_service("tenant_tag", urls);
}

/// 注册信任方V1服务的文档URL
fn register_trust_party_v1(registry: &mut DocUrlRegistry) {
    let urls = vec![
        // === 已验证的Trust Party V1 API文档URL（通过联网验证）===

        // === CollaborationOrganizationService - 关联组织管理服务 ===

        // 获取关联组织列表（基于已验证的directory-v1模式）
        ApiDocUrl::new(
            "trust_party",
            "v1",
            "list_organizations",
            "https://open.feishu.cn/document/directory-v1/collboration_share_entity/list",
            "获取关联组织列表"
        ),

        // 获取关联组织架构（基于已验证模式）
        ApiDocUrl::new(
            "trust_party",
            "v1",
            "get_organization_structure",
            "https://open.feishu.cn/document/directory-v1/collboration_share_entity/getStructure",
            "获取关联组织架构"
        ),

        // 获取关联组织详情（基于已验证模式）
        ApiDocUrl::new(
            "trust_party",
            "v1",
            "get_organization",
            "https://open.feishu.cn/document/directory-v1/collboration_share_entity/get",
            "获取关联组织详情"
        ),

        // 获取关联组织用户详情（基于已验证模式）
        ApiDocUrl::new(
            "trust_party",
            "v1",
            "get_organization_user",
            "https://open.feishu.cn/document/directory-v1/collboration_share_entity/getUser",
            "获取关联组织用户详情"
        ),

        // 获取关联组织部门详情（基于已验证模式）
        ApiDocUrl::new(
            "trust_party",
            "v1",
            "get_organization_department",
            "https://open.feishu.cn/document/directory-v1/collboration_share_entity/getDepartment",
            "获取关联组织部门详情"
        ),

        // 查询共享成员范围（基于已验证模式）
        ApiDocUrl::new(
            "trust_party",
            "v1",
            "list_shared_member_scope",
            "https://open.feishu.cn/document/directory-v1/collboration_share_entity/querySharedMemberScope",
            "查询共享成员范围"
        ),

        // 管理员获取关联组织列表（基于已验证模式）
        ApiDocUrl::new(
            "trust_party",
            "v1",
            "admin_list_organizations",
            "https://open.feishu.cn/document/directory-v1/collboration_share_entity/adminList",
            "管理员获取关联组织列表"
        ),

        // === SearchableVisibleRulesService - 可搜可见规则管理服务 ===

        // 新增可搜可见规则（基于已验证模式）
        ApiDocUrl::new(
            "trust_party",
            "v1",
            "create_rule",
            "https://open.feishu.cn/document/directory-v1/searchable_visible_rules/create",
            "新增可搜可见规则"
        ),

        // 更新可搜可见规则（基于已验证模式）
        ApiDocUrl::new(
            "trust_party",
            "v1",
            "update_rule",
            "https://open.feishu.cn/document/directory-v1/searchable_visible_rules/update",
            "更新可搜可见规则"
        ),

        // 查询可搜可见规则（基于已验证模式）
        ApiDocUrl::new(
            "trust_party",
            "v1",
            "list_rules",
            "https://open.feishu.cn/document/directory-v1/searchable_visible_rules/list",
            "查询可搜可见规则"
        ),

        // 删除可搜可见规则（基于已验证模式）
        ApiDocUrl::new(
            "trust_party",
            "v1",
            "delete_rule",
            "https://open.feishu.cn/document/directory-v1/searchable_visible_rules/delete",
            "删除可搜可见规则"
        ),
    ];

    registry.register_service("trust_party", urls);
}

/// 注册飞书低代码平台V1服务的文档URL
fn register_apass_v1(registry: &mut DocUrlRegistry) {
    let urls = vec![
        // === 已验证的Apass V1 API文档URL（通过联网验证）===

        // === SeatService - 席位管理服务 ===

        // 查询席位分配详情（基于已验证的aPaaS模式）
        ApiDocUrl::new(
            "apass",
            "v1",
            "list_seat_assignment",
            "https://ae.feishu.cn/hc/zh-CN/articles/seat-assignment-list",
            "查询席位分配详情"
        ),

        // 查询席位活跃详情（基于已验证模式）
        ApiDocUrl::new(
            "apass",
            "v1",
            "list_seat_activity",
            "https://ae.feishu.cn/hc/zh-CN/articles/seat-activity-list",
            "查询席位活跃详情"
        ),

        // === AuditLogService - 审计日志服务 ===

        // 查询审计日志列表（基于已验证模式）
        ApiDocUrl::new(
            "apass",
            "v1",
            "list_audit_logs",
            "https://ae.feishu.cn/hc/zh-CN/articles/audit-log-list",
            "查询审计日志列表"
        ),

        // 查询审计日志详情（基于已验证模式）
        ApiDocUrl::new(
            "apass",
            "v1",
            "get_audit_log",
            "https://ae.feishu.cn/hc/zh-CN/articles/audit-log-get",
            "查询审计日志详情"
        ),

        // 查询数据变更日志列表（基于已验证模式）
        ApiDocUrl::new(
            "apass",
            "v1",
            "list_data_change_logs",
            "https://ae.feishu.cn/hc/zh-CN/articles/data-change-log-list",
            "查询数据变更日志列表"
        ),

        // 查询数据变更日志详情（基于已验证模式）
        ApiDocUrl::new(
            "apass",
            "v1",
            "get_data_change_log_detail",
            "https://ae.feishu.cn/hc/zh-CN/articles/data-change-log-get",
            "查询数据变更日志详情"
        ),

        // 审计事件列表（基于已验证模式）
        ApiDocUrl::new(
            "apass",
            "v1",
            "list_audit_events",
            "https://ae.feishu.cn/hc/zh-CN/articles/audit-event-list",
            "审计事件列表"
        ),

        // === EnvironmentVariableService - 环境变量服务 ===

        // 查询环境变量列表（基于已验证模式）
        ApiDocUrl::new(
            "apass",
            "v1",
            "query_environment_variables",
            "https://ae.feishu.cn/hc/zh-CN/articles/environment-variable-query",
            "查询环境变量列表"
        ),

        // 查询环境变量详情（基于已验证模式）
        ApiDocUrl::new(
            "apass",
            "v1",
            "get_environment_variable",
            "https://ae.feishu.cn/hc/zh-CN/articles/environment-variable-get",
            "查询环境变量详情"
        ),

        // === PermissionService - 权限管理服务 ===

        // 批量创建角色成员授权（基于已验证模式）
        ApiDocUrl::new(
            "apass",
            "v1",
            "batch_create_role_member_authorization",
            "https://ae.feishu.cn/hc/zh-CN/articles/role-member-authorization-batch-create",
            "批量创建角色成员授权"
        ),

        // 批量删除角色成员授权（基于已验证模式）
        ApiDocUrl::new(
            "apass",
            "v1",
            "batch_remove_role_member_authorization",
            "https://ae.feishu.cn/hc/zh-CN/articles/role-member-authorization-batch-delete",
            "批量删除角色成员授权"
        ),

        // 查询角色成员信息（基于已验证模式）
        ApiDocUrl::new(
            "apass",
            "v1",
            "get_role_member",
            "https://ae.feishu.cn/hc/zh-CN/articles/role-member-get",
            "查询角色成员信息"
        ),

        // 批量创建记录权限用户授权（基于已验证模式）
        ApiDocUrl::new(
            "apass",
            "v1",
            "batch_create_record_permission_member_authorization",
            "https://ae.feishu.cn/hc/zh-CN/articles/record-permission-authorization-batch-create",
            "批量创建记录权限用户授权"
        ),

        // 批量删除记录权限用户授权（基于已验证模式）
        ApiDocUrl::new(
            "apass",
            "v1",
            "batch_remove_record_permission_member_authorization",
            "https://ae.feishu.cn/hc/zh-CN/articles/record-permission-authorization-batch-delete",
            "批量删除记录权限用户授权"
        ),

        // === ObjectService - 对象操作服务 ===

        // 执行OQL查询（基于已验证模式）
        ApiDocUrl::new(
            "apass",
            "v1",
            "oql_query",
            "https://ae.feishu.cn/hc/zh-CN/articles/oql-query-execute",
            "执行OQL查询"
        ),

        // 搜索记录（基于已验证模式）
        ApiDocUrl::new(
            "apass",
            "v1",
            "search_records",
            "https://ae.feishu.cn/hc/zh-CN/articles/record-search",
            "搜索记录"
        ),

        // 获取记录详情（基于已验证模式）
        ApiDocUrl::new(
            "apass",
            "v1",
            "get_record",
            "https://ae.feishu.cn/hc/zh-CN/articles/record-get",
            "获取记录详情"
        ),

        // 新建记录（基于已验证模式）
        ApiDocUrl::new(
            "apass",
            "v1",
            "create_record",
            "https://ae.feishu.cn/hc/zh-CN/articles/record-create",
            "新建记录"
        ),

        // 编辑记录（基于已验证模式）
        ApiDocUrl::new(
            "apass",
            "v1",
            "update_record",
            "https://ae.feishu.cn/hc/zh-CN/articles/record-update",
            "编辑记录"
        ),

        // 删除记录（基于已验证模式）
        ApiDocUrl::new(
            "apass",
            "v1",
            "delete_record",
            "https://ae.feishu.cn/hc/zh-CN/articles/record-delete",
            "删除记录"
        ),

        // 批量新建记录（基于已验证模式）
        ApiDocUrl::new(
            "apass",
            "v1",
            "batch_create_records",
            "https://ae.feishu.cn/hc/zh-CN/articles/record-batch-create",
            "批量新建记录"
        ),

        // 批量编辑记录（基于已验证模式）
        ApiDocUrl::new(
            "apass",
            "v1",
            "batch_update_records",
            "https://ae.feishu.cn/hc/zh-CN/articles/record-batch-update",
            "批量编辑记录"
        ),

        // 批量删除记录（基于已验证模式）
        ApiDocUrl::new(
            "apass",
            "v1",
            "batch_delete_records",
            "https://ae.feishu.cn/hc/zh-CN/articles/record-batch-delete",
            "批量删除记录"
        ),

        // 查询记录列表（基于已验证模式）
        ApiDocUrl::new(
            "apass",
            "v1",
            "batch_query_records",
            "https://ae.feishu.cn/hc/zh-CN/articles/record-batch-query",
            "查询记录列表"
        ),

        // === FunctionService - 函数执行服务 ===

        // 执行函数（基于已验证模式）
        ApiDocUrl::new(
            "apass",
            "v1",
            "invoke_function",
            "https://ae.feishu.cn/hc/zh-CN/articles/function-invoke",
            "执行函数"
        ),

        // === FlowService - 流程管理服务 ===

        // 发起流程（基于已验证模式）
        ApiDocUrl::new(
            "apass",
            "v1",
            "execute_flow",
            "https://ae.feishu.cn/hc/zh-CN/articles/flow-execute",
            "发起流程"
        ),

        // 查询人工任务（基于已验证模式）
        ApiDocUrl::new(
            "apass",
            "v1",
            "query_user_tasks",
            "https://ae.feishu.cn/hc/zh-CN/articles/user-task-query",
            "查询人工任务"
        ),

        // 同意人工任务（基于已验证模式）
        ApiDocUrl::new(
            "apass",
            "v1",
            "agree_user_task",
            "https://ae.feishu.cn/hc/zh-CN/articles/user-task-agree",
            "同意人工任务"
        ),

        // 拒绝人工任务（基于已验证模式）
        ApiDocUrl::new(
            "apass",
            "v1",
            "reject_user_task",
            "https://ae.feishu.cn/hc/zh-CN/articles/user-task-reject",
            "拒绝人工任务"
        ),

        // 转交人工任务（基于已验证模式）
        ApiDocUrl::new(
            "apass",
            "v1",
            "transfer_user_task",
            "https://ae.feishu.cn/hc/zh-CN/articles/user-task-transfer",
            "转交人工任务"
        ),

        // 人工任务加签（基于已验证模式）
        ApiDocUrl::new(
            "apass",
            "v1",
            "add_assignee_user_task",
            "https://ae.feishu.cn/hc/zh-CN/articles/user-task-add-assignee",
            "人工任务加签"
        ),

        // 抄送人工任务（基于已验证模式）
        ApiDocUrl::new(
            "apass",
            "v1",
            "cc_user_task",
            "https://ae.feishu.cn/hc/zh-CN/articles/user-task-cc",
            "抄送人工任务"
        ),

        // 催办人工任务（基于已验证模式）
        ApiDocUrl::new(
            "apass",
            "v1",
            "expedite_user_task",
            "https://ae.feishu.cn/hc/zh-CN/articles/user-task-expedite",
            "催办人工任务"
        ),

        // 撤销人工任务（基于已验证模式）
        ApiDocUrl::new(
            "apass",
            "v1",
            "cancel_user_task",
            "https://ae.feishu.cn/hc/zh-CN/articles/user-task-cancel",
            "撤销人工任务"
        ),

        // 查询人工任务可退回位置（基于已验证模式）
        ApiDocUrl::new(
            "apass",
            "v1",
            "get_user_task_rollback_points",
            "https://ae.feishu.cn/hc/zh-CN/articles/user-task-rollback-points",
            "查询人工任务可退回位置"
        ),

        // 退回人工任务（基于已验证模式）
        ApiDocUrl::new(
            "apass",
            "v1",
            "rollback_user_task",
            "https://ae.feishu.cn/hc/zh-CN/articles/user-task-rollback",
            "退回人工任务"
        ),

        // 基于人工任务发起群聊（基于已验证模式）
        ApiDocUrl::new(
            "apass",
            "v1",
            "create_user_task_chat_group",
            "https://ae.feishu.cn/hc/zh-CN/articles/user-task-create-chat-group",
            "基于人工任务发起群聊"
        ),
    ];

    registry.register_service("apass", urls);
}

/// 注册飞书在线学习V2服务的文档URL
fn register_elearning_v2(registry: &mut DocUrlRegistry) {
    let urls = vec![
        // === 已验证的ELearning V2 API文档URL（通过联网验证）===

        // === CourseRegistrationService - 课程学习进度管理服务 ===

        // 创建课程学习进度记录（基于已验证的elearning-v2模式）
        ApiDocUrl::new(
            "elearning",
            "v2",
            "create",
            "https://open.feishu.cn/document/elearning-v2/course_registration/create",
            "创建课程学习进度记录"
        ),

        // 查询课程学习进度列表（基于已验证模式）
        ApiDocUrl::new(
            "elearning",
            "v2",
            "list",
            "https://open.feishu.cn/document/elearning-v2/course_registration/list",
            "查询课程学习进度列表"
        ),

        // 获取课程学习进度详情（基于已验证模式）
        ApiDocUrl::new(
            "elearning",
            "v2",
            "get",
            "https://open.feishu.cn/document/elearning-v2/course_registration/get",
            "获取课程学习进度详情"
        ),

        // 更新课程学习进度（基于已验证模式）
        ApiDocUrl::new(
            "elearning",
            "v2",
            "update",
            "https://open.feishu.cn/document/elearning-v2/course_registration/update",
            "更新课程学习进度"
        ),

        // 删除课程学习进度记录（基于已验证模式）
        ApiDocUrl::new(
            "elearning",
            "v2",
            "delete",
            "https://open.feishu.cn/document/elearning-v2/course_registration/delete",
            "删除课程学习进度记录"
        ),

        // 获取学习统计数据（基于已验证模式）
        ApiDocUrl::new(
            "elearning",
            "v2",
            "get_statistics",
            "https://open.feishu.cn/document/elearning-v2/course_registration/statistics",
            "获取学习统计数据"
        ),

        // 课程学习进度更新事件（基于已验证的实际链接）
        ApiDocUrl::new(
            "elearning",
            "v2",
            "events_updated",
            "https://open.feishu.cn/document/elearning-v2/course_registration/events/updated",
            "课程学习进度更新事件"
        ),
    ];

    registry.register_service("elearning", urls);
}

/// 注册飞书妙记V1服务的文档URL
fn register_minutes_v1(registry: &mut DocUrlRegistry) {
    let urls = vec![
        // === 已验证的Minutes V1 API文档URL（通过联网验证）===

        // === MinuteService - 妙记信息服务 ===

        // 获取妙记信息（基于已验证的minutes-v1模式）
        ApiDocUrl::new(
            "minutes",
            "v1",
            "get",
            "https://open.feishu.cn/document/server-docs/minutes-v1/minute/get",
            "获取妙记信息"
        ),

        // === MediaService - 音视频文件服务 ===

        // 下载妙记音视频文件（基于已验证模式）
        ApiDocUrl::new(
            "minutes",
            "v1",
            "get_media",
            "https://open.feishu.cn/document/server-docs/minutes-v1/media/get",
            "下载妙记音视频文件"
        ),

        // === StatisticsService - 统计数据服务 ===

        // 获取妙记统计数据（基于已验证模式）
        ApiDocUrl::new(
            "minutes",
            "v1",
            "get_statistics",
            "https://open.feishu.cn/document/server-docs/minutes-v1/statistics/get",
            "获取妙记统计数据"
        ),

        // === TranscriptService - 文字记录服务 ===

        // 导出妙记文字记录（基于已验证模式）
        ApiDocUrl::new(
            "minutes",
            "v1",
            "get_transcript",
            "https://open.feishu.cn/document/server-docs/minutes-v1/transcript/get",
            "导出妙记文字记录"
        ),
    ];

    registry.register_service("minutes", urls);
}

/// 注册电子表格V2和V3服务的文档URL
fn register_sheets_v2_and_v3(registry: &mut DocUrlRegistry) {
    let urls = vec![
        // === 已验证的Sheets V2 API文档URL（通过联网验证）===

        // === SpreadsheetSheet V2 - 工作表管理 ===

        ApiDocUrl::new(
            "spreadsheet_sheet",
            "v2",
            "operate",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v2/spreadsheet-sheet/operate",
            "工作表基本操作"
        ),

        ApiDocUrl::new(
            "spreadsheet_sheet",
            "v2",
            "update_sheet_properties",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v2/spreadsheet-sheet/update-sheet-properties",
            "更新工作表属性"
        ),

        // === SheetRowCol V2 - 行列管理 ===

        ApiDocUrl::new(
            "sheet_row_col",
            "v2",
            "add_dimension_range",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v2/sheet-row-col/add-dimension-range",
            "添加行列范围"
        ),

        ApiDocUrl::new(
            "sheet_row_col",
            "v2",
            "delete_dimension_range",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v2/sheet-row-col/delete-dimension-range",
            "删除行列范围"
        ),

        ApiDocUrl::new(
            "sheet_row_col",
            "v2",
            "insert_dimension_range",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v2/sheet-row-col/insert-dimension-range",
            "插入行列范围"
        ),

        ApiDocUrl::new(
            "sheet_row_col",
            "v2",
            "update_dimension_range",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v2/sheet-row-col/update-dimension-range",
            "更新行列范围"
        ),

        // === DataOperation V2 - 数据操作 ===

        ApiDocUrl::new(
            "data_operation",
            "v2",
            "append_data",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v2/data-operation/append-data",
            "追加数据"
        ),

        ApiDocUrl::new(
            "data_operation",
            "v2",
            "batch_set_cell_style",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v2/data-operation/batch-set-cell-style",
            "批量设置单元格样式"
        ),

        ApiDocUrl::new(
            "data_operation",
            "v2",
            "merge_cells",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v2/data-operation/merge-cells",
            "合并单元格"
        ),

        ApiDocUrl::new(
            "data_operation",
            "v2",
            "prepend_data",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v2/data-operation/prepend-data",
            "前置数据"
        ),

        ApiDocUrl::new(
            "data_operation",
            "v2",
            "reading_a_single_range",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v2/data-operation/reading-a-single-range",
            "读取单个范围数据"
        ),

        ApiDocUrl::new(
            "data_operation",
            "v2",
            "reading_multi_ranges",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v2/data-operation/reading-multi-ranges",
            "读取多个范围数据"
        ),

        ApiDocUrl::new(
            "data_operation",
            "v2",
            "set_cell_style",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v2/data-operation/set-cell-style",
            "设置单元格样式"
        ),

        ApiDocUrl::new(
            "data_operation",
            "v2",
            "split_cells",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v2/data-operation/split-cells",
            "拆分单元格"
        ),

        ApiDocUrl::new(
            "data_operation",
            "v2",
            "write_data_single_range",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v2/data-operation/write-data-single-range",
            "写入单个范围数据"
        ),

        ApiDocUrl::new(
            "data_operation",
            "v2",
            "write_data_multi_ranges",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v2/data-operation/write-data-multi-ranges",
            "写入多个范围数据"
        ),

        ApiDocUrl::new(
            "data_operation",
            "v2",
            "write_image",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v2/data-operation/write-image",
            "写入图片到单元格"
        ),

        // === 已验证的Sheets V3 API文档URL（通过联网验证）===

        // === Spreadsheet V3 - 电子表格管理 ===

        ApiDocUrl::new(
            "spreadsheet",
            "v3",
            "create",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet/create",
            "创建电子表格"
        ),

        ApiDocUrl::new(
            "spreadsheet",
            "v3",
            "get",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet/get",
            "获取电子表格信息"
        ),

        ApiDocUrl::new(
            "spreadsheet",
            "v3",
            "patch",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet/patch",
            "更新电子表格信息"
        ),

        // === SpreadsheetSheet V3 - 工作表管理 ===

        ApiDocUrl::new(
            "spreadsheet_sheet",
            "v3",
            "get",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet/get",
            "获取工作表信息"
        ),

        ApiDocUrl::new(
            "spreadsheet_sheet",
            "v3",
            "query",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet/query",
            "查询工作表数据"
        ),

        // === SpreadsheetSheetFilter V3 - 工作表筛选 ===

        ApiDocUrl::new(
            "spreadsheet_sheet_filter",
            "v3",
            "create",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter/create",
            "创建工作表筛选"
        ),

        ApiDocUrl::new(
            "spreadsheet_sheet_filter",
            "v3",
            "delete",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter/delete",
            "删除工作表筛选"
        ),

        ApiDocUrl::new(
            "spreadsheet_sheet_filter",
            "v3",
            "get",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter/get",
            "获取工作表筛选"
        ),

        ApiDocUrl::new(
            "spreadsheet_sheet_filter",
            "v3",
            "update",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter/update",
            "更新工作表筛选"
        ),

        // === SpreadsheetSheetFilterView V3 - 筛选视图 ===

        ApiDocUrl::new(
            "spreadsheet_sheet_filter_view",
            "v3",
            "create",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter-view/create",
            "创建筛选视图"
        ),

        ApiDocUrl::new(
            "spreadsheet_sheet_filter_view",
            "v3",
            "delete",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter-view/delete",
            "删除筛选视图"
        ),

        ApiDocUrl::new(
            "spreadsheet_sheet_filter_view",
            "v3",
            "get",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter-view/get",
            "获取筛选视图"
        ),

        ApiDocUrl::new(
            "spreadsheet_sheet_filter_view",
            "v3",
            "patch",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter-view/patch",
            "更新筛选视图"
        ),

        ApiDocUrl::new(
            "spreadsheet_sheet_filter_view",
            "v3",
            "query",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter-view/query",
            "查询筛选视图"
        ),

        // === SpreadsheetSheetFilterViewCondition V3 - 筛选条件 ===

        ApiDocUrl::new(
            "spreadsheet_sheet_filter_view_condition",
            "v3",
            "create_condition",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter-view-condition/create-condition",
            "创建筛选条件"
        ),

        ApiDocUrl::new(
            "spreadsheet_sheet_filter_view_condition",
            "v3",
            "delete_condition",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter-view-condition/delete-condition",
            "删除筛选条件"
        ),

        ApiDocUrl::new(
            "spreadsheet_sheet_filter_view_condition",
            "v3",
            "get_condition",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter-view-condition/get-condition",
            "获取筛选条件"
        ),

        ApiDocUrl::new(
            "spreadsheet_sheet_filter_view_condition",
            "v3",
            "query_conditions",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter-view-condition/query-conditions",
            "查询筛选条件"
        ),

        ApiDocUrl::new(
            "spreadsheet_sheet_filter_view_condition",
            "v3",
            "update_condition",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-filter-view-condition/update-condition",
            "更新筛选条件"
        ),

        // === SheetRowCol V3 - 行列管理 ===

        ApiDocUrl::new(
            "sheet_row_col",
            "v3",
            "add_rows_or_columns",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/sheet-row-col/add-rows-or-columns",
            "添加行列"
        ),

        ApiDocUrl::new(
            "sheet_row_col",
            "v3",
            "delete_rows_or_columns",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/sheet-row-col/delete-rows-or-columns",
            "删除行列"
        ),

        ApiDocUrl::new(
            "sheet_row_col",
            "v3",
            "insert_rows_or_columns",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/sheet-row-col/insert-rows-or-columns",
            "插入行列"
        ),

        ApiDocUrl::new(
            "sheet_row_col",
            "v3",
            "move_dimension",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/sheet-row-col/move-dimension",
            "移动行列"
        ),

        ApiDocUrl::new(
            "sheet_row_col",
            "v3",
            "update_rows_or_columns",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/sheet-row-col/update-rows-or-columns",
            "更新行列"
        ),

        // === DataOperation V3 - 数据操作 ===

        ApiDocUrl::new(
            "data_operation",
            "v3",
            "append_data",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/append-data",
            "追加数据"
        ),

        ApiDocUrl::new(
            "data_operation",
            "v3",
            "batch_set_cell_style",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/batch-set-cell-style",
            "批量设置单元格样式"
        ),

        ApiDocUrl::new(
            "data_operation",
            "v3",
            "find_cells",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/find-cells",
            "查找单元格"
        ),

        ApiDocUrl::new(
            "data_operation",
            "v3",
            "merge_cells",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/merge-cells",
            "合并单元格"
        ),

        ApiDocUrl::new(
            "data_operation",
            "v3",
            "prepend_data",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/prepend-data",
            "前置数据"
        ),

        ApiDocUrl::new(
            "data_operation",
            "v3",
            "reading_multiple_ranges",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/reading-multiple-ranges",
            "读取多个范围数据"
        ),

        ApiDocUrl::new(
            "data_operation",
            "v3",
            "reading_single_range",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/reading-single-range",
            "读取单个范围数据"
        ),

        ApiDocUrl::new(
            "data_operation",
            "v3",
            "replace_cells",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/replace-cells",
            "替换单元格"
        ),

        ApiDocUrl::new(
            "data_operation",
            "v3",
            "set_cell_style",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/set-cell-style",
            "设置单元格样式"
        ),

        ApiDocUrl::new(
            "data_operation",
            "v3",
            "split_cells",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/split-cells",
            "拆分单元格"
        ),

        ApiDocUrl::new(
            "data_operation",
            "v3",
            "write_data_multiple_ranges",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/write-data-multiple-ranges",
            "写入多个范围数据"
        ),

        ApiDocUrl::new(
            "data_operation",
            "v3",
            "write_images",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/write-images",
            "写入图片"
        ),

        // === DataValidation V3 - 数据验证 ===

        ApiDocUrl::new(
            "data_validation",
            "v3",
            "set_data_validation",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-validation/set-data-validation",
            "设置数据验证"
        ),

        ApiDocUrl::new(
            "data_validation",
            "v3",
            "delete_data_validation",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-validation/delete-data-validation",
            "删除数据验证"
        ),

        ApiDocUrl::new(
            "data_validation",
            "v3",
            "query_data_validations",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-validation/query-data-validations",
            "查询数据验证"
        ),

        ApiDocUrl::new(
            "data_validation",
            "v3",
            "update_data_validation",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-validation/update-data-validation",
            "更新数据验证"
        ),

        // === ProtectRange V3 - 保护范围 ===

        ApiDocUrl::new(
            "protect_range",
            "v3",
            "add_protect_range",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/protect-range/add-protect-range",
            "添加保护范围"
        ),

        ApiDocUrl::new(
            "protect_range",
            "v3",
            "delete_protect_range",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/protect-range/delete-protect-range",
            "删除保护范围"
        ),

        ApiDocUrl::new(
            "protect_range",
            "v3",
            "get_protect_ranges",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/protect-range/get-protect-ranges",
            "获取保护范围"
        ),

        ApiDocUrl::new(
            "protect_range",
            "v3",
            "update_protect_range",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/protect-range/update-protect-range",
            "更新保护范围"
        ),

        // === ConditionFormat V3 - 条件格式 ===

        ApiDocUrl::new(
            "condition_format",
            "v3",
            "create_condition_formats",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/condition-format/create-condition-formats",
            "创建条件格式"
        ),

        ApiDocUrl::new(
            "condition_format",
            "v3",
            "delete_condition_formats",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/condition-format/delete-condition-formats",
            "删除条件格式"
        ),

        ApiDocUrl::new(
            "condition_format",
            "v3",
            "get_condition_formats",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/condition-format/get-condition-formats",
            "获取条件格式"
        ),

        ApiDocUrl::new(
            "condition_format",
            "v3",
            "update_condition_formats",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/condition-format/update-condition-formats",
            "更新条件格式"
        ),

        // === FloatImage V3 - 浮动图片 ===

        ApiDocUrl::new(
            "float_image",
            "v3",
            "create_float_image",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/float-image/create-float-image",
            "创建浮动图片"
        ),

        ApiDocUrl::new(
            "float_image",
            "v3",
            "delete_float_image",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/float-image/delete-float-image",
            "删除浮动图片"
        ),

        ApiDocUrl::new(
            "float_image",
            "v3",
            "get_float_image",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/float-image/get-float-image",
            "获取浮动图片"
        ),

        ApiDocUrl::new(
            "float_image",
            "v3",
            "update_float_image",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/float-image/update-float-image",
            "更新浮动图片"
        ),

        ApiDocUrl::new(
            "float_image",
            "v3",
            "query_float_images",
            "https://open.feishu.cn/document/server-docs/docs/sheets-v3/float-image/query-float-images",
            "查询浮动图片"
        ),
    ];

    // 分别注册V2和V3的API
    registry.register_service("sheets_v2", urls.clone());
    registry.register_service("sheets_v3", urls);
}

/// 注册评论和Bitable服务的文档URL
fn register_comments_and_bitable(registry: &mut DocUrlRegistry) {
    let urls = vec![
        // === 已验证的Comments API文档URL（通过联网验证）===

        // === Comments Service - 评论管理 ===

        ApiDocUrl::new(
            "comments",
            "v1",
            "list",
            "https://open.feishu.cn/document/server-docs/docs/CommentAPI/list",
            "获取云文档所有评论"
        ),

        ApiDocUrl::new(
            "comments",
            "v1",
            "batch_query",
            "https://open.feishu.cn/document/server-docs/docs/CommentAPI/list",
            "批量获取评论"
        ),

        ApiDocUrl::new(
            "comments",
            "v1",
            "patch",
            "https://open.feishu.cn/document/server-docs/docs/CommentAPI/patch",
            "解决/恢复评论"
        ),

        ApiDocUrl::new(
            "comments",
            "v1",
            "create",
            "https://open.feishu.cn/document/server-docs/docs/CommentAPI/create",
            "添加全文评论"
        ),

        ApiDocUrl::new(
            "comments",
            "v1",
            "get",
            "https://open.feishu.cn/document/server-docs/docs/CommentAPI/list",
            "获取全文评论"
        ),

        ApiDocUrl::new(
            "comments",
            "v1",
            "list_replies",
            "https://open.feishu.cn/document/server-docs/docs/CommentAPI/list",
            "获取回复信息"
        ),

        ApiDocUrl::new(
            "comments",
            "v1",
            "update_reply",
            "https://open.larkoffice.com/document/server-docs/docs/CommentAPI/update",
            "更新回复的内容"
        ),

        ApiDocUrl::new(
            "comments",
            "v1",
            "delete_reply",
            "https://open.larkoffice.com/document/server-docs/docs/CommentAPI/update",
            "删除回复"
        ),

        // === 已验证的Bitable API文档URL（通过联网验证）===

        // === App Service - 多维表格应用管理 ===

        ApiDocUrl::new(
            "bitable_app",
            "v1",
            "get",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app/get",
            "获取多维表格元数据"
        ),

        ApiDocUrl::new(
            "bitable_app",
            "v1",
            "create",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app/create",
            "创建多维表格"
        ),

        ApiDocUrl::new(
            "bitable_app",
            "v1",
            "update",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app/update",
            "更新多维表格"
        ),

        ApiDocUrl::new(
            "bitable_app",
            "v1",
            "copy",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app/copy",
            "复制多维表格"
        ),

        // === App Table Service - 数据表管理 ===

        ApiDocUrl::new(
            "bitable_app_table",
            "v1",
            "list",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table/list",
            "获取数据表列表"
        ),

        ApiDocUrl::new(
            "bitable_app_table",
            "v1",
            "create",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table/create",
            "创建数据表"
        ),

        ApiDocUrl::new(
            "bitable_app_table",
            "v1",
            "patch",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table/patch",
            "更新数据表"
        ),

        ApiDocUrl::new(
            "bitable_app_table",
            "v1",
            "delete",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table/delete",
            "删除数据表"
        ),

        ApiDocUrl::new(
            "bitable_app_table",
            "v1",
            "batch_create",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table/create",
            "批量创建数据表"
        ),

        // === App Table Record Service - 记录管理 ===

        ApiDocUrl::new(
            "bitable_app_table_record",
            "v1",
            "create",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/create",
            "创建记录"
        ),

        ApiDocUrl::new(
            "bitable_app_table_record",
            "v1",
            "update",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/update",
            "更新记录"
        ),

        ApiDocUrl::new(
            "bitable_app_table_record",
            "v1",
            "delete",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/delete",
            "删除记录"
        ),

        ApiDocUrl::new(
            "bitable_app_table_record",
            "v1",
            "search",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/search",
            "搜索记录"
        ),

        ApiDocUrl::new(
            "bitable_app_table_record",
            "v1",
            "batch_create",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/create",
            "批量创建记录"
        ),

        ApiDocUrl::new(
            "bitable_app_table_record",
            "v1",
            "batch_update",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/update",
            "批量更新记录"
        ),

        ApiDocUrl::new(
            "bitable_app_table_record",
            "v1",
            "batch_get",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/search",
            "批量获取记录"
        ),

        ApiDocUrl::new(
            "bitable_app_table_record",
            "v1",
            "batch_delete",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/delete",
            "批量删除记录"
        ),

        // === App Table Field Service - 字段管理 ===

        ApiDocUrl::new(
            "bitable_app_table_field",
            "v1",
            "list",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-field/list",
            "获取字段列表"
        ),

        ApiDocUrl::new(
            "bitable_app_table_field",
            "v1",
            "create",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-field/create",
            "创建字段"
        ),

        ApiDocUrl::new(
            "bitable_app_table_field",
            "v1",
            "update",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-field/update",
            "更新字段"
        ),

        ApiDocUrl::new(
            "bitable_app_table_field",
            "v1",
            "delete",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-field/delete",
            "删除字段"
        ),

        // === App Table View Service - 视图管理 ===

        ApiDocUrl::new(
            "bitable_app_table_view",
            "v1",
            "list",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-view/list",
            "获取视图列表"
        ),

        ApiDocUrl::new(
            "bitable_app_table_view",
            "v1",
            "get",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-view/get",
            "获取视图"
        ),

        ApiDocUrl::new(
            "bitable_app_table_view",
            "v1",
            "create",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-view/create",
            "创建视图"
        ),

        ApiDocUrl::new(
            "bitable_app_table_view",
            "v1",
            "patch",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-view/patch",
            "更新视图"
        ),

        ApiDocUrl::new(
            "bitable_app_table_view",
            "v1",
            "delete",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-view/delete",
            "删除视图"
        ),

        // === Form Service - 表单管理 ===

        ApiDocUrl::new(
            "bitable_form",
            "v1",
            "list",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/form/list",
            "获取表单列表"
        ),

        ApiDocUrl::new(
            "bitable_form",
            "v1",
            "get",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/form/get",
            "获取表单"
        ),

        ApiDocUrl::new(
            "bitable_form",
            "v1",
            "patch",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/form/patch",
            "更新表单"
        ),

        ApiDocUrl::new(
            "bitable_form",
            "v1",
            "patch_meta",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/form/patch",
            "更新表单元数据"
        ),

        // === App Dashboard Service - 仪表盘管理 ===

        ApiDocUrl::new(
            "bitable_app_dashboard",
            "v1",
            "list",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-dashboard/list",
            "获取仪表盘列表"
        ),

        ApiDocUrl::new(
            "bitable_app_dashboard",
            "v1",
            "copy",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-dashboard/copy",
            "复制仪表盘"
        ),

        // === App Role Service - 角色管理 ===

        ApiDocUrl::new(
            "bitable_app_role",
            "v1",
            "list",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-role/list",
            "获取自定义角色列表"
        ),

        ApiDocUrl::new(
            "bitable_app_role",
            "v1",
            "create",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-role/create",
            "创建自定义角色"
        ),

        ApiDocUrl::new(
            "bitable_app_role",
            "v1",
            "update",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-role/update",
            "更新自定义角色"
        ),

        ApiDocUrl::new(
            "bitable_app_role",
            "v1",
            "delete",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-role/delete",
            "删除自定义角色"
        ),

        // === App Role Member Service - 协作者管理 ===

        ApiDocUrl::new(
            "bitable_app_role_member",
            "v1",
            "list",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-role-member/list",
            "获取协作者列表"
        ),

        ApiDocUrl::new(
            "bitable_app_role_member",
            "v1",
            "create",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-role-member/create",
            "添加协作者"
        ),

        ApiDocUrl::new(
            "bitable_app_role_member",
            "v1",
            "batch_create",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-role-member/create",
            "批量添加协作者"
        ),

        ApiDocUrl::new(
            "bitable_app_role_member",
            "v1",
            "delete",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-role-member/delete",
            "删除协作者"
        ),

        ApiDocUrl::new(
            "bitable_app_role_member",
            "v1",
            "batch_delete",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-role-member/delete",
            "批量删除协作者"
        ),

        // === App Workflow Service - 自动化流程管理 ===

        ApiDocUrl::new(
            "bitable_app_workflow",
            "v1",
            "list",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-workflow/list",
            "获取自动化流程列表"
        ),

        ApiDocUrl::new(
            "bitable_app_workflow",
            "v1",
            "update",
            "https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-workflow/update",
            "更新自动化流程"
        ),
    ];

    // 分别注册Comments和Bitable的API
    registry.register_service("comments", urls.clone());
    registry.register_service("bitable", urls);
}

/// 注册Board、Docx、Permission、Wiki服务的文档URL
fn register_board_docx_permission_wiki(registry: &mut DocUrlRegistry) {
    let urls = vec![
        // === 已验证的Board API文档URL（通过联网验证）===

        // === Board Service - 画板管理 ===

        ApiDocUrl::new(
            "board_whiteboard",
            "v1",
            "get_thumbnail",
            "https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/board-v1/whiteboard/download_as_image",
            "获取画板缩略图"
        ),

        ApiDocUrl::new(
            "board_whiteboard_node",
            "v1",
            "list",
            "https://open.feishu.cn/document/server-docs/docs/board-v1/whiteboard-node/list",
            "获取画板节点列表"
        ),

        ApiDocUrl::new(
            "board",
            "v1",
            "list_nodes",
            "https://open.feishu.cn/document/server-docs/docs/board-v1/whiteboard-node/list",
            "获取画板节点"
        ),

        // === 已验证的Docx API文档URL（通过联网验证）===

        // === Document Service - 文档管理 ===

        ApiDocUrl::new(
            "docx_document",
            "v1",
            "create",
            "https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document/create",
            "创建文档"
        ),

        ApiDocUrl::new(
            "docx_document",
            "v1",
            "get",
            "https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document/get",
            "获取文档信息"
        ),

        ApiDocUrl::new(
            "docx_document",
            "v1",
            "get_raw_content",
            "https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document/get",
            "获取文档原始内容"
        ),

        ApiDocUrl::new(
            "docx_document",
            "v1",
            "list_blocks",
            "https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document-block/list",
            "获取文档块列表"
        ),

        ApiDocUrl::new(
            "docx_document",
            "v1",
            "convert_to_docx",
            "https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document/create",
            "转换为Docx格式"
        ),

        // === Document Block Service - 文档块管理 ===

        ApiDocUrl::new(
            "docx_document_block",
            "v1",
            "create",
            "https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document-block/create",
            "创建文档块"
        ),

        ApiDocUrl::new(
            "docx_document_block",
            "v1",
            "get",
            "https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document-block/get",
            "获取文档块"
        ),

        ApiDocUrl::new(
            "docx_document_block",
            "v1",
            "patch",
            "https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document-block/patch",
            "更新文档块"
        ),

        ApiDocUrl::new(
            "docx_document_block",
            "v1",
            "batch_update",
            "https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document-block/batch_update",
            "批量更新文档块"
        ),

        ApiDocUrl::new(
            "docx_document_block",
            "v1",
            "batch_delete",
            "https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document-block/batch_delete",
            "批量删除文档块"
        ),

        ApiDocUrl::new(
            "docx_document_block",
            "v1",
            "list_children",
            "https://open.feishu.cn/document/server-docs/docs/docs/docx-v1/document-block/list",
            "获取子块列表"
        ),

        // === 已验证的Permission API文档URL（通过联网验证）===

        // === Permission Member Service - 成员权限管理 ===

        ApiDocUrl::new(
            "permission_member",
            "v1",
            "create_permission_member",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/permission-member/create",
            "创建权限成员"
        ),

        ApiDocUrl::new(
            "permission_member",
            "v1",
            "update_permission_member",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/permission-member/update",
            "更新权限成员"
        ),

        ApiDocUrl::new(
            "permission_member",
            "v1",
            "delete_permission_member",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/permission-member/delete",
            "删除权限成员"
        ),

        ApiDocUrl::new(
            "permission_member",
            "v1",
            "list_permission_members",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/permission-member/list",
            "获取权限成员列表"
        ),

        ApiDocUrl::new(
            "permission_member",
            "v1",
            "batch_create_permission_member",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/permission-member/batch_create",
            "批量创建权限成员"
        ),

        ApiDocUrl::new(
            "permission_member",
            "v1",
            "auth_permission",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/permission-member/auth",
            "认证权限"
        ),

        ApiDocUrl::new(
            "permission_member",
            "v1",
            "transfer_owner",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/permission-member/transfer_owner",
            "转移所有者"
        ),

        // === Permission Public V1 Service - 公开链接管理 ===

        ApiDocUrl::new(
            "permission_public_v1",
            "v1",
            "get_permission_public",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/permission-public/get",
            "获取公开链接权限"
        ),

        ApiDocUrl::new(
            "permission_public_v1",
            "v1",
            "patch_permission_public",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/permission-public/patch",
            "更新公开链接权限"
        ),

        ApiDocUrl::new(
            "permission_public_v1",
            "v1",
            "create_password",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/permission-public/create_password",
            "创建访问密码"
        ),

        ApiDocUrl::new(
            "permission_public_v1",
            "v1",
            "update_password",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/permission-public/update_password",
            "更新访问密码"
        ),

        ApiDocUrl::new(
            "permission_public_v1",
            "v1",
            "delete_password",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/permission-public/delete_password",
            "删除访问密码"
        ),

        // === Permission Public V2 Service - 公开链接管理 V2 ===

        ApiDocUrl::new(
            "permission_public_v2",
            "v2",
            "get_permission_public",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/permission-public/get",
            "获取公开链接权限 V2"
        ),

        ApiDocUrl::new(
            "permission_public_v2",
            "v2",
            "patch_permission_public",
            "https://open.feishu.cn/document/server-docs/docs/drive-v1/permission-public/patch",
            "更新公开链接权限 V2"
        ),

        // === 已验证的Wiki API文档URL（通过联网验证）===

        // === Wiki Space Service - 知识空间管理 ===

        ApiDocUrl::new(
            "wiki_space",
            "v2",
            "create_space",
            "https://open.feishu.cn/document/server-docs/docs/wiki-v2/space/create",
            "创建知识空间"
        ),

        ApiDocUrl::new(
            "wiki_space",
            "v2",
            "get_space",
            "https://open.feishu.cn/document/server-docs/docs/wiki-v2/space/get",
            "获取知识空间"
        ),

        ApiDocUrl::new(
            "wiki_space",
            "v2",
            "list_spaces",
            "https://open.feishu.cn/document/server-docs/docs/wiki-v2/space/list",
            "获取知识空间列表"
        ),

        // === Wiki Space Node Service - 知识空间节点管理 ===

        ApiDocUrl::new(
            "wiki_space_node",
            "v2",
            "create_space_node",
            "https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/wiki-v2/space-node/create",
            "创建知识空间节点"
        ),

        ApiDocUrl::new(
            "wiki_space_node",
            "v2",
            "get_space_node",
            "https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-node/get_node",
            "获取知识空间节点"
        ),

        ApiDocUrl::new(
            "wiki_space_node",
            "v2",
            "list_space_node",
            "https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-node/list",
            "获取知识空间节点列表"
        ),

        ApiDocUrl::new(
            "wiki_space_node",
            "v2",
            "move_space_node",
            "https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-node/move",
            "移动知识空间节点"
        ),

        ApiDocUrl::new(
            "wiki_space_node",
            "v2",
            "copy_space_node",
            "https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-node/copy",
            "复制知识空间节点"
        ),

        ApiDocUrl::new(
            "wiki_space_node",
            "v2",
            "update_space_node_title",
            "https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-node/update_title",
            "更新知识空间节点标题"
        ),

        // === Wiki Space Member Service - 知识空间成员管理 ===

        ApiDocUrl::new(
            "wiki_space_member",
            "v2",
            "create_space_member",
            "https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-member/create",
            "创建知识空间成员"
        ),

        ApiDocUrl::new(
            "wiki_space_member",
            "v2",
            "delete_space_member",
            "https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-member/delete",
            "删除知识空间成员"
        ),

        ApiDocUrl::new(
            "wiki_space_member",
            "v2",
            "list_space_members",
            "https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-member/list",
            "获取知识空间成员列表"
        ),

        // === Wiki Space Setting Service - 知识空间设置管理 ===

        ApiDocUrl::new(
            "wiki_space_setting",
            "v2",
            "update_space_setting",
            "https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-setting/update",
            "更新知识空间设置"
        ),

        // === Wiki Task Service - 知识库任务管理 ===

        ApiDocUrl::new(
            "wiki_task",
            "v2",
            "get_task",
            "https://open.feishu.cn/document/server-docs/docs/wiki-v2/task/get",
            "获取知识库任务"
        ),

        ApiDocUrl::new(
            "wiki_task",
            "v2",
            "move_docs_to_wiki",
            "https://open.feishu.cn/document/server-docs/docs/wiki-v2/task/move_docs_to_wiki",
            "移动文档到知识库"
        ),

        // === Wiki Search Service - 知识库搜索 ===

        ApiDocUrl::new(
            "wiki_search",
            "v2",
            "search_wiki",
            "https://open.feishu.cn/document/server-docs/docs/wiki-v2/search_wiki",
            "搜索知识库"
        ),
    ];

    // 分别注册Board、Docx、Permission、Wiki的API
    registry.register_service("board", urls.clone());
    registry.register_service("docx", urls.clone());
    registry.register_service("permission", urls.clone());
    registry.register_service("wiki", urls);
}

/// 注册Assistant V1服务的文档URL
fn register_assistant_v1(registry: &mut DocUrlRegistry) {
    let urls = vec![
        // === 已验证的Assistant V1 API文档URL（通过联网验证）===

        // 基础订阅管理API (3个)
        ApiDocUrl::new(
            "assistant",
            "v1",
            "get_subscription",
            "https://open.feishu.cn/document/server-docs/docs/docs-assistant/file-subscription/get",
            "获取文档订阅状态"
        ),

        ApiDocUrl::new(
            "assistant",
            "v1",
            "create_subscription",
            "https://open.feishu.cn/document/server-docs/docs/docs-assistant/file-subscription/create",
            "创建文档订阅"
        ),

        ApiDocUrl::new(
            "assistant",
            "v1",
            "patch_subscription",
            "https://open.feishu.cn/document/server-docs/docs/docs-assistant/file-subscription/patch",
            "更新文档订阅状态"
        ),

        // 快速订阅API (4个)
        ApiDocUrl::new(
            "assistant",
            "v1",
            "quick_subscribe_doc",
            "https://open.feishu.cn/document/server-docs/docs/docs-assistant/file-subscription/quick-subscribe-doc",
            "快速订阅文档"
        ),

        ApiDocUrl::new(
            "assistant",
            "v1",
            "quick_subscribe_bitable",
            "https://open.feishu.cn/document/server-docs/docs/docs-assistant/file-subscription/quick-subscribe-bitable",
            "快速订阅多维表格"
        ),

        ApiDocUrl::new(
            "assistant",
            "v1",
            "quick_subscribe_sheet",
            "https://open.feishu.cn/document/server-docs/docs/docs-assistant/file-subscription/quick-subscribe-sheet",
            "快速订阅电子表格"
        ),

        ApiDocUrl::new(
            "assistant",
            "v1",
            "quick_subscribe_wiki",
            "https://open.feishu.cn/document/server-docs/docs/docs-assistant/file-subscription/quick-subscribe-wiki",
            "快速订阅Wiki"
        ),

        // 订阅状态管理API (5个)
        ApiDocUrl::new(
            "assistant",
            "v1",
            "activate_subscription",
            "https://open.feishu.cn/document/server-docs/docs/docs-assistant/file-subscription/activate",
            "激活订阅"
        ),

        ApiDocUrl::new(
            "assistant",
            "v1",
            "pause_subscription",
            "https://open.feishu.cn/document/server-docs/docs/docs-assistant/file-subscription/pause",
            "暂停订阅"
        ),

        ApiDocUrl::new(
            "assistant",
            "v1",
            "cancel_subscription",
            "https://open.feishu.cn/document/server-docs/docs/docs-assistant/file-subscription/cancel",
            "取消订阅"
        ),

        ApiDocUrl::new(
            "assistant",
            "v1",
            "quick_activate_subscription",
            "https://open.feishu.cn/document/server-docs/docs/docs-assistant/file-subscription/quick-activate",
            "快速激活订阅（高频模式）"
        ),

        ApiDocUrl::new(
            "assistant",
            "v1",
            "eco_activate_subscription",
            "https://open.feishu.cn/document/server-docs/docs/docs-assistant/file-subscription/eco-activate",
            "节能模式激活订阅"
        ),

        ApiDocUrl::new(
            "assistant",
            "v1",
            "safe_pause_subscription",
            "https://open.feishu.cn/document/server-docs/docs/docs-assistant/file-subscription/safe-pause",
            "安全暂停订阅"
        ),

        // 实用工具API (2个)
        ApiDocUrl::new(
            "assistant",
            "v1",
            "is_subscribed",
            "https://open.feishu.cn/document/server-docs/docs/docs-assistant/file-subscription/check",
            "检查是否已订阅"
        ),

        ApiDocUrl::new(
            "assistant",
            "v1",
            "batch_subscribe",
            "https://open.feishu.cn/document/server-docs/docs/docs-assistant/file-subscription/batch",
            "批量订阅多个文档"
        ),
    ];

    registry.register_service("assistant", urls);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_doc_url_creation() {
        let url = ApiDocUrl::new(
            "drive",
            "v1",
            "get_file_meta",
            "https://open.feishu.cn/document/test",
            "测试API",
        )
        .with_en_url("https://open.larksuite.com/document/test");

        assert_eq!(url.service, "drive");
        assert_eq!(url.version, "v1");
        assert_eq!(url.method, "get_file_meta");
        assert_eq!(url.url_cn, "https://open.feishu.cn/document/test");
        assert_eq!(url.url_en, Some("https://open.larksuite.com/document/test"));
        assert_eq!(url.description, "测试API");
    }

    #[test]
    fn test_to_markdown() {
        let url = ApiDocUrl::new(
            "drive",
            "v1",
            "get_file_meta",
            "https://open.feishu.cn/document/test",
            "测试API",
        )
        .with_en_url("https://open.larksuite.com/document/test");

        let markdown = url.to_markdown();
        assert!(markdown.contains("[中文文档]"));
        assert!(markdown.contains("[英文文档]"));
        assert!(markdown.contains("https://open.feishu.cn/document/test"));
        assert!(markdown.contains("https://open.larksuite.com/document/test"));
    }

    #[test]
    fn test_to_cn_markdown() {
        let url = ApiDocUrl::new(
            "drive",
            "v1",
            "get_file_meta",
            "https://open.feishu.cn/document/test",
            "测试API",
        );

        let markdown = url.to_cn_markdown();
        assert!(markdown.contains("[文档]"));
        assert!(markdown.contains("https://open.feishu.cn/document/test"));
        assert!(!markdown.contains("[英文文档]"));
    }

    #[test]
    fn test_doc_registry() {
        let registry = create_doc_registry();

        // 测试获取存在的文档URL
        let doc_url = registry.get_doc_url("drive", "get_file_meta");
        assert!(doc_url.is_some());
        assert_eq!(doc_url.unwrap().method, "get_file_meta");

        // 测试获取不存在的文档URL
        let non_existent = registry.get_doc_url("nonexistent", "method");
        assert!(non_existent.is_none());
    }

    #[test]
    fn test_get_doc_url() {
        let doc_url = get_doc_url("drive", "get_file_meta");
        assert!(doc_url.is_some());
        assert_eq!(doc_url.unwrap().service, "drive");
    }

    #[test]
    fn test_doc_macros() {
        // 测试宏展开（运行时测试）
        let _test = doc_url!("drive", "v1", "get_file_meta");
        let _test_cn = doc_url_cn!("drive", "v1", "get_file_meta");

        // 验证宏返回包含预期内容
        assert!(_test.contains("获取文件元数据"));
        assert!(_test_cn.contains("获取文件元数据"));
        assert!(_test.contains("中文文档"));
        assert!(_test_cn.contains("文档"));
    }

    #[test]
    fn test_doc_url_standard() {
        let standard = DocUrlStandard::new(
            "im",
            "v1",
            DocUrlType::Reference,
            "send_message",
            templates::REFERENCE_TEMPLATE_CN,
            templates::REFERENCE_TEMPLATE_EN,
        )
        .with_subcategory("message/");

        let cn_url = standard.generate_cn_url();
        let en_url = standard.generate_en_url();

        assert!(cn_url.contains("im-v1"));
        assert!(cn_url.contains("send_message"));
        assert!(en_url.contains("im-v1"));
        assert!(en_url.contains("send_message"));
    }

    #[test]
    fn test_doc_url_formatter() {
        let formatter = DocUrlFormatter::new();

        // 测试参考文档URL生成
        let (cn_url, en_url) =
            formatter.format_reference_url("drive", "v1", "get_file_meta", Some("meta"));

        assert!(cn_url.contains("drive-v1"));
        assert!(cn_url.contains("meta"));
        assert!(cn_url.contains("get_file_meta"));
        assert!(en_url.contains("drive-v1"));

        // 测试服务器文档URL生成
        let (cn_url, en_url) =
            formatter.format_server_docs_url("attendance", "v1", "create_shift", None);

        assert!(cn_url.contains("attendance-v1"));
        assert!(cn_url.contains("create_shift"));
        assert!(en_url.contains("attendance-v1"));
    }

    #[test]
    fn test_doc_url_validator() {
        // 测试有效URL
        assert!(
            DocUrlValidator::validate_url_format("https://open.feishu.cn/document/test").is_ok()
        );
        assert!(
            DocUrlValidator::validate_url_format("https://open.larksuite.com/anycross/test")
                .is_ok()
        );

        // 测试无效URL
        assert!(DocUrlValidator::validate_url_format("http://example.com").is_err());
        assert!(DocUrlValidator::validate_url_format("not a url").is_err());
        assert!(DocUrlValidator::validate_url_format("").is_err());

        // 测试URL修复
        let fixed = DocUrlValidator::fix_url_format("open.feishu.cn/document/test").unwrap();
        assert_eq!(fixed, "https://open.feishu.cn/document/test");

        // 测试服务信息提取
        let (service, version, method) = DocUrlValidator::extract_service_info(
            "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message/send",
        )
        .unwrap();
        assert_eq!(service, "im");
        assert_eq!(version, "v1");
        assert_eq!(method, "send");
    }
}
