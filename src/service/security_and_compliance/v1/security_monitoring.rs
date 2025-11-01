//! SecurityMonitoring安全监控服务
//!
//! 提供企业级实时安全监控功能：
//! - 实时威胁检测和分析
//! - 安全事件智能分类和优先级排序
//! - 自动化安全响应和缓解措施
//! - 安全态势感知和预警系统
//! - 异常行为检测和分析
//! - 攻击链分析和溯源追踪
//!
//! # 示例
//!
//! ```rust,no_run
//! use open_lark::prelude::*;
//! use open_lark::service::security_and_compliance::v1::security_monitoring::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = LarkClient::builder()
//!         .app_id("your_app_id")
//!         .app_secret("your_app_secret")
//!         .build()?;
//!
//!     // 获取实时安全事件
//!     let request = GetRealTimeSecurityEventsRequest {
//!         limit: Some(50),
//!         severity_filter: Some(vec![SecurityLevel::High, SecurityLevel::Critical]),
//!         ..Default::default()
//!     };
//!
//!     let response = client.security_and_compliance.v1.security_monitoring.get_real_time_security_events(&request).await?;
//!     println!("获取到 {} 个安全事件", response.events.len());
//!     Ok(())
//! }
//! ```

use super::*;
use crate::core::{
    api_resp::BaseResponse,
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use async_trait::async_trait;
use open_lark_core::core::api_req::ApiRequest; // trait_system::ExecutableBuilder temporarily disabled
use serde::{Deserialize, Serialize};

// 导入核心类型
use super::types::*;

/// 安全监控服务
///
/// 提供完整的企业级安全监控解决方案，包括威胁检测、事件分析、
/// 自动响应等功能，帮助企业构建全面的安全防护体系。
#[derive(Debug, Clone)]
pub struct SecurityMonitoringService {
    pub config: Config,
}

impl SecurityMonitoringService {
    /// 创建安全监控服务实例
    ///
    /// # 参数
    /// - `config`: SDK配置信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::security_and_compliance::v1::security_monitoring::SecurityMonitoringService;
    ///
    /// let config = Config::new("app_id", "app_secret");
    /// let service = SecurityMonitoringService::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取实时安全事件
    ///
    /// 获取最新的安全事件和威胁信息，支持多种过滤条件和排序方式。
    /// 返回的事件按时间倒序排列，包含完整的威胁情报和分析数据。
    ///
    /// # API文档
    ///
    /// 获取系统中的实时安全事件，包括威胁检测、异常行为、安全事件等信息。
    ///
    /// # 参数
    ///
    /// * `request` - 请求参数，包含过滤条件、分页信息等
    ///
    /// # 返回值
    ///
    /// 返回安全事件列表及相关的统计信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::security_and_compliance::v1::security_monitoring::*;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = LarkClient::builder()
    ///         .app_id("your_app_id")
    ///         .app_secret("your_app_secret")
    ///         .build()?;
    ///
    ///     let request = GetRealTimeSecurityEventsRequest {
    ///         limit: Some(20),
    ///         severity_filter: Some(vec![SecurityLevel::High, SecurityLevel::Critical]),
    ///         start_time: Some(1609459200), // 2021-01-01
    ///         end_time: Some(1609545600),   // 2021-01-02
    ///         ..Default::default()
    ///     };
    ///
    ///     let response = client.security_and_compliance.v1.security_monitoring
    ///         .get_real_time_security_events(&request).await?;
    ///
    ///     for event in &response.events {
    ///         println!("事件: {} - {}", event.event_type, event.title.as_ref().unwrap_or(&"".to_string()));
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_real_time_security_events(
        &self,
        request: &GetRealTimeSecurityEventsRequest,
    ) -> SDKResult<BaseResponse<GetRealTimeSecurityEventsResponse>> {
        // 构建API请求
        let api_req = ApiRequest {
            http_http_http_method: reqwest::Method::POST,
            api_path:
                "/open-apis/security_and_compliance/v1/security_monitoring/get_real_time_events"
                    .to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(request)?,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 获取安全态势分析
    ///
    /// 提供整体安全态势分析和趋势预测，包括安全评分、威胁趋势、
    /// 关键指标等信息。支持自定义分析时间范围和多维度数据展示。
    ///
    /// # API文档
    ///
    /// 获取指定时间范围内的安全态势分析报告，包含整体安全评分、
    /// 威胁检测率、响应时间等关键指标的趋势分析。
    ///
    /// # 参数
    ///
    /// * `request` - 请求参数，包含分析时间范围、指标类型等
    ///
    /// # 返回值
    ///
    /// 返回安全态势分析报告，包含评分、趋势、关键指标和建议
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::security_and_compliance::v1::security_monitoring::*;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = LarkClient::builder()
    ///         .app_id("your_app_id")
    ///         .app_secret("your_app_secret")
    ///         .build()?;
    ///
    ///     let request = GetSecurityPostureAnalysisRequest {
    ///         start_time: 1609459200, // 2021-01-01
    ///         end_time: 1612137600,   // 2021-02-01
    ///         analysis_types: vec!["threat_trends".to_string(), "risk_assessment".to_string()],
    ///         ..Default::default()
    ///     };
    ///
    ///     let response = client.security_and_compliance.v1.security_monitoring
    ///         .get_security_posture_analysis(&request).await?;
    ///
    ///     println!("安全评分: {:.1}", response.overall_score);
    ///     println!("安全级别: {:?}", response.security_level);
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_security_posture_analysis(
        &self,
        request: &GetSecurityPostureAnalysisRequest,
    ) -> SDKResult<BaseResponse<GetSecurityPostureAnalysisResponse>> {
        // 构建API请求
        let api_req = ApiRequest {
            http_http_http_method: reqwest::Method::POST,
            api_path:
                "/open-apis/security_and_compliance/v1/security_monitoring/get_posture_analysis"
                    .to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(request)?,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 获取异常行为检测结果
    ///
    /// 基于机器学习和行为分析技术，检测用户和系统的异常行为模式。
    /// 支持多种检测算法和自定义规则，提供实时的异常预警和分析。
    ///
    /// # API文档
    ///
    /// 获取指定范围内的异常行为检测结果，包括访问模式异常、
    /// 数据传输异常、登录行为异常等多维度分析。
    ///
    /// # 参数
    ///
    /// * `request` - 请求参数，包含检测范围、异常类型、时间窗口等
    ///
    /// # 返回值
    ///
    /// 返回异常检测结果，包含异常事件详情、风险评估和处理建议
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::security_and_compliance::v1::security_monitoring::*;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = LarkClient::builder()
    ///         .app_id("your_app_id")
    ///         .app_secret("your_app_secret")
    ///         .build()?;
    ///
    ///     let request = GetAnomalyDetectionResultsRequest {
    ///         start_time: 1609459200,
    ///         end_time: 1609545600,
    ///         anomaly_types: vec!["access_pattern".to_string(), "data_transfer".to_string()],
    ///         ..Default::default()
    ///     };
    ///
    ///     let response = client.security_and_compliance.v1.security_monitoring
    ///         .get_anomaly_detection_results(&request).await?;
    ///
    ///     println!("检测到 {} 个异常", response.anomalies.len());
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_anomaly_detection_results(
        &self,
        request: &GetAnomalyDetectionResultsRequest,
    ) -> SDKResult<BaseResponse<GetAnomalyDetectionResultsResponse>> {
        // 构建API请求
        let api_req = ApiRequest {
            http_http_http_method: reqwest::Method::POST,
            api_path: "/open-apis/security_and_compliance/v1/security_monitoring/get_anomaly_detection_results".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(request)?,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    // 模拟实现，用于演示和测试
    pub async fn get_anomaly_detection_results_mock(
        &self,
        request: &GetAnomalyDetectionResultsRequest,
    ) -> SDKResult<GetAnomalyDetectionResultsResponse> {
        let current_time = chrono::Utc::now().timestamp();

        let anomalies = vec![
            AnomalyDetection {
                anomaly_id: "anom_001".to_string(),
                anomaly_type: "访问模式异常".to_string(),
                confidence_score: 0.92,
                severity: SecurityLevel::High,
                entity_type: "用户".to_string(),
                entity_id: "user_789".to_string(),
                entity_name: "张三".to_string(),
                description: "用户在凌晨2-4点期间访问了超过正常数量10倍的敏感文件".to_string(),
                detected_at: current_time - 7200,
                baseline_behavior: "正常工作日9-17点访问，平均每天访问15个文件".to_string(),
                anomalous_behavior: "凌晨访问150个文件，包括45个高敏感度文件".to_string(),
                risk_factors: vec![
                    "非工作时间访问".to_string(),
                    "访问量异常激增".to_string(),
                    "高敏感度文件访问".to_string(),
                ],
                context_info: AnomalyContext {
                    ip_address: "192.168.1.200".to_string(),
                    device_type: "Windows工作站".to_string(),
                    location: "北京总部".to_string(),
                    vpn_status: "未使用VPN".to_string(),
                },
                recommended_actions: vec![
                    "立即联系用户确认访问合法性".to_string(),
                    "临时限制该用户的高敏感度文件访问权限".to_string(),
                    "审查该用户的访问日志和历史行为".to_string(),
                ],
            },
            AnomalyDetection {
                anomaly_id: "anom_002".to_string(),
                anomaly_type: "数据传输异常".to_string(),
                confidence_score: 0.88,
                severity: SecurityLevel::Critical,
                entity_type: "系统".to_string(),
                entity_id: "server_db_01".to_string(),
                entity_name: "客户数据库服务器".to_string(),
                description: "检测到异常大量数据向外传输，传输速度超出正常范围500%".to_string(),
                detected_at: current_time - 3600,
                baseline_behavior: "正常数据传输速度10-50MB/小时，主要在业务时间".to_string(),
                anomalous_behavior: "传输速度250MB/小时，持续传输12小时，包含非业务时间"
                    .to_string(),
                risk_factors: vec![
                    "传输速度异常".to_string(),
                    "传输时间异常".to_string(),
                    "数据量异常".to_string(),
                    "跨网络传输".to_string(),
                ],
                context_info: AnomalyContext {
                    ip_address: "10.0.2.100".to_string(),
                    device_type: "Linux服务器".to_string(),
                    location: "数据中心A".to_string(),
                    vpn_status: "内网传输".to_string(),
                },
                recommended_actions: vec![
                    "立即阻断异常数据传输".to_string(),
                    "隔离受影响的服务器".to_string(),
                    "启动数据泄露应急响应程序".to_string(),
                    "进行全面的系统安全扫描".to_string(),
                ],
            },
        ];

        let total_count = anomalies.len() as i32;
        Ok(GetAnomalyDetectionResultsResponse {
            anomalies,
            total_count,
            detection_summary: AnomalyDetectionSummary {
                total_anomalies_detected: 156,
                high_confidence_anomalies: 23,
                critical_anomalies: 8,
                false_positive_rate: 0.08,
                detection_accuracy: 0.94,
                analysis_time_range: AnalysisTimeRange {
                    start_time: request.start_time,
                    end_time: request.end_time,
                },
            },
        })
    }

    /// 获取攻击链分析
    /// 分析攻击链的各个阶段和攻击路径
    pub async fn get_attack_chain_analysis(
        &self,
        request: &GetAttackChainAnalysisRequest,
    ) -> SDKResult<BaseResponse<GetAttackChainAnalysisResponse>> {
        // 构建API请求
        let api_req = ApiRequest {
            http_http_http_method: reqwest::Method::POST,
            api_path: "/open-apis/security_and_compliance/v1/security_monitoring/get_attack_chain_analysis".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(request)?,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    // 模拟实现，用于演示和测试
    pub async fn get_attack_chain_analysis_mock(
        &self,
        request: &GetAttackChainAnalysisRequest,
    ) -> SDKResult<GetAttackChainAnalysisResponse> {
        let current_time = chrono::Utc::now().timestamp();

        Ok(GetAttackChainAnalysisResponse {
            attack_chain_id: "chain_001".to_string(),
            attack_name: "高级持续性威胁(APT)攻击".to_string(),
            attack_stage: AttackStage::LateralMovement,
            severity: SecurityLevel::Critical,
            start_time: current_time - 86400 * 3,
            last_activity: current_time - 1800,
            attack_stages: vec![
                AttackStageInfo {
                    stage: AttackStage::Reconnaissance,
                    stage_name: "信息收集".to_string(),
                    status: StageStatus::Completed,
                    start_time: current_time - 86400 * 3,
                    end_time: Some(current_time - (86400 * 28 / 10)),
                    description: "攻击者收集目标企业信息，识别关键系统和人员".to_string(),
                    indicators: vec![
                        "大量端口扫描活动".to_string(),
                        "DNS查询异常增加".to_string(),
                        "社交媒体信息收集".to_string(),
                    ],
                    affected_assets: vec!["公网服务器".to_string(), "域名系统".to_string()],
                },
                AttackStageInfo {
                    stage: AttackStage::InitialAccess,
                    stage_name: "初始访问".to_string(),
                    status: StageStatus::Completed,
                    start_time: current_time - (86400 * 28 / 10),
                    end_time: Some(current_time - (86400 * 25 / 10)),
                    description: "通过钓鱼邮件获取员工凭证，成功进入内部网络".to_string(),
                    indicators: vec![
                        "钓鱼邮件点击".to_string(),
                        "异常登录成功".to_string(),
                        "VPN访问异常".to_string(),
                    ],
                    affected_assets: vec!["员工邮箱".to_string(), "VPN网关".to_string()],
                },
                AttackStageInfo {
                    stage: AttackStage::Execution,
                    stage_name: "恶意代码执行".to_string(),
                    status: StageStatus::Completed,
                    start_time: current_time - (86400 * 25 / 10),
                    end_time: Some(current_time - 86400 * 2),
                    description: "在内部网络部署恶意软件，建立持久化访问".to_string(),
                    indicators: vec![
                        "可疑进程创建".to_string(),
                        "注册表修改".to_string(),
                        "计划任务创建".to_string(),
                    ],
                    affected_assets: vec!["工作站PC-012".to_string(), "文件服务器".to_string()],
                },
                AttackStageInfo {
                    stage: AttackStage::LateralMovement,
                    stage_name: "横向移动".to_string(),
                    status: StageStatus::InProgress,
                    start_time: current_time - 86400 * 2,
                    end_time: None,
                    description: "攻击者在内部网络中移动，寻找高价值目标".to_string(),
                    indicators: vec![
                        "异常网络连接".to_string(),
                        "凭证转储尝试".to_string(),
                        "远程桌面连接".to_string(),
                    ],
                    affected_assets: vec!["域控制器".to_string(), "数据库服务器".to_string()],
                },
            ],
            attack_path: AttackPath {
                entry_point: "钓鱼邮件".to_string(),
                initial_compromise: "员工工作站".to_string(),
                pivot_points: vec!["文件服务器".to_string(), "域控制器".to_string()],
                final_target: "客户数据库".to_string(),
                current_location: "域控制器".to_string(),
            },
            tactics_techniques: vec![
                TTP {
                    tactic: "Initial Access".to_string(),
                    technique: "Spearphishing Attachment".to_string(),
                    technique_id: "T1566.001".to_string(),
                    description: "通过带有恶意附件的钓鱼邮件获取初始访问权限".to_string(),
                    observed_indicators: vec!["恶意PDF文件".to_string(), "宏代码执行".to_string()],
                },
                TTP {
                    tactic: "Credential Access".to_string(),
                    technique: "OS Credential Dumping".to_string(),
                    technique_id: "T1003".to_string(),
                    description: "从操作系统中转储用户凭证".to_string(),
                    observed_indicators: vec![
                        "Mimikatz使用".to_string(),
                        "LSASS进程访问".to_string(),
                    ],
                },
            ],
            impact_assessment: AttackImpact {
                compromised_assets: 12,
                data_exfiltrated: "约2GB客户数据".to_string(),
                business_disruption: "部分服务受影响".to_string(),
                financial_impact: Some(500000.0),
                reputation_impact: "高".to_string(),
            },
            containment_actions: vec![
                "隔离受感染的工作站".to_string(),
                "重置所有用户凭证".to_string(),
                "阻断恶意C2通信".to_string(),
                "部署额外的网络监控".to_string(),
            ],
            remediation_plan: RemediationPlan {
                immediate_actions: vec![
                    "完全隔离受影响的系统".to_string(),
                    "进行全面恶意软件清除".to_string(),
                    "修复所有已知漏洞".to_string(),
                ],
                short_term_actions: vec![
                    "加强网络分段".to_string(),
                    "实施更严格的访问控制".to_string(),
                    "部署增强的端点保护".to_string(),
                ],
                long_term_actions: vec![
                    "建立安全运营中心(SOC)".to_string(),
                    "实施零信任架构".to_string(),
                    "定期进行渗透测试".to_string(),
                ],
            },
        })
    }
}

// ==================== 数据模型 ====================

/// 实时安全事件请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRealTimeSecurityEventsRequest {
    /// 开始时间
    pub start_time: i64,
    /// 结束时间
    pub end_time: i64,
    /// 限制数量
    pub limit: Option<i32>,
    /// 严重性过滤器
    pub severity_filter: Option<Vec<SecurityLevel>>,
    /// 事件类型过滤器
    pub event_type_filter: Option<Vec<String>>,
    /// 用户ID过滤器
    pub user_id_filter: Option<String>,
    /// 资源类型过滤器
    pub resource_type_filter: Option<Vec<String>>,
    /// 分页游标
    pub page_token: Option<String>,
}

impl Default for GetRealTimeSecurityEventsRequest {
    fn default() -> Self {
        Self {
            start_time: 0,
            end_time: 0,
            limit: None,
            severity_filter: None,
            event_type_filter: None,
            user_id_filter: None,
            resource_type_filter: None,
            page_token: None,
        }
    }
}

/// 实时安全事件响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRealTimeSecurityEventsResponse {
    /// 安全事件列表
    pub events: Vec<SecurityEvent>,
    /// 总数量
    pub total_count: i32,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页游标
    pub next_cursor: Option<String>,
    /// 监控摘要
    pub monitoring_summary: SecurityMonitoringSummary,
}

/// 安全监控摘要
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityMonitoringSummary {
    /// 今日总事件数
    pub total_events_today: i32,
    /// 关键事件数
    pub critical_events: i32,
    /// 高风险事件数
    pub high_events: i32,
    /// 中等风险事件数
    pub medium_events: i32,
    /// 低风险事件数
    pub low_events: i32,
    /// 活跃威胁数
    pub active_threats: i32,
    /// 已控制威胁数
    pub contained_threats: i32,
    /// 已解决威胁数
    pub resolved_threats: i32,
    /// 最后更新时间
    pub last_updated: i64,
    /// 监控覆盖率
    pub monitoring_coverage: f64,
}

/// 安全态势分析请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSecurityPostureAnalysisRequest {
    /// 分析开始时间
    pub start_time: i64,
    /// 分析结束时间
    pub end_time: i64,
    /// 分析类型
    pub analysis_types: Vec<AnalysisType>,
}

impl Default for GetSecurityPostureAnalysisRequest {
    fn default() -> Self {
        Self {
            start_time: 0,
            end_time: 0,
            analysis_types: vec![],
        }
    }
}

/// 分析类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnalysisType {
    /// 威胁分析
    Threat,
    /// 漏洞分析
    Vulnerability,
    /// 合规分析
    Compliance,
    /// 风险分析
    Risk,
    /// 态势分析
    Posture,
}

/// 安全态势分析响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSecurityPostureAnalysisResponse {
    /// 总体评分
    pub overall_score: f64,
    /// 安全级别
    pub security_level: SecurityLevel,
    /// 态势评级
    pub posture_rating: String,
    /// 分析周期
    pub analysis_period: AnalysisPeriod,
    /// 关键指标
    pub key_metrics: SecurityMetrics,
    /// 威胁态势
    pub threat_landscape: ThreatLandscape,
    /// 安全建议
    pub recommendations: Vec<SecurityRecommendation>,
    /// 生成时间
    pub generated_at: i64,
    /// 下次分析时间
    pub next_analysis_time: i64,
}

/// 分析周期
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisPeriod {
    /// 开始日期
    pub start_date: i64,
    /// 结束日期
    pub end_date: i64,
}

/// 安全指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityMetrics {
    /// 威胁检测率
    pub threat_detection_rate: f64,
    /// 误报率
    pub false_positive_rate: f64,
    /// 平均检测时间(分钟)
    pub mean_time_to_detect: f64,
    /// 平均响应时间(分钟)
    pub mean_time_to_respond: f64,
    /// 安全事件趋势
    pub security_incidents_trend: IncidentTrend,
    /// 资产覆盖率
    pub asset_coverage: AssetCoverage,
    /// 合规状态指标
    pub compliance_status: ComplianceStatusMetrics,
}

/// 事件趋势
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncidentTrend {
    /// 日均事件数
    pub daily_average: f64,
    /// 周均事件数
    pub weekly_average: f64,
    /// 月均事件数
    pub monthly_average: f64,
    /// 趋势方向
    pub trend_direction: String,
    /// 趋势百分比
    pub trend_percentage: f64,
}

/// 资产覆盖率
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetCoverage {
    /// 总资产数
    pub total_assets: i32,
    /// 已监控资产数
    pub monitored_assets: i32,
    /// 覆盖率百分比
    pub coverage_percentage: f64,
    /// 关键资产已覆盖数
    pub critical_assets_covered: i32,
    /// 关键资产总数
    pub critical_assets_total: i32,
}

/// 合规状态指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceStatusMetrics {
    /// 总体合规率
    pub overall_compliance: f64,
    /// 关键控制措施合规数
    pub critical_controls_compliant: i32,
    /// 关键控制措施总数
    pub critical_controls_total: i32,
    /// 最后审计日期
    pub last_audit_date: i64,
    /// 下次审计日期
    pub next_audit_date: i64,
}

/// 威胁态势
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatLandscape {
    /// 主要威胁向量
    pub top_threats: Vec<ThreatVector>,
    /// 新兴威胁
    pub emerging_threats: Vec<EmergingThreat>,
}

/// 威胁向量
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatVector {
    /// 威胁类型
    pub threat_type: ThreatType,
    /// 发生频率
    pub frequency: i32,
    /// 严重性评分
    pub severity_score: f64,
    /// 趋势
    pub trend: String,
    /// 受影响资产数
    pub affected_assets: i32,
    /// 描述
    pub description: String,
}

/// 新兴威胁
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergingThreat {
    /// 威胁名称
    pub threat_name: String,
    /// 描述
    pub description: String,
    /// 潜在影响
    pub potential_impact: String,
    /// 准备度水平
    pub readiness_level: f64,
    /// 推荐行动
    pub recommended_actions: Vec<String>,
}

/// 安全建议
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRecommendation {
    /// 建议ID
    pub recommendation_id: String,
    /// 标题
    pub title: String,
    /// 描述
    pub description: String,
    /// 优先级
    pub priority: i32,
    /// 类别
    pub category: String,
    /// 预估工作量
    pub estimated_effort: String,
    /// 预期影响
    pub expected_impact: String,
    /// 实施步骤
    pub implementation_steps: Vec<String>,
}

/// 异常检测请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAnomalyDetectionResultsRequest {
    /// 分析开始时间
    pub start_time: i64,
    /// 分析结束时间
    pub end_time: i64,
    /// 实体类型
    pub entity_types: Vec<String>,
    /// 置信度阈值
    pub confidence_threshold: Option<f64>,
    /// 严重性过滤
    pub severity_levels: Option<Vec<SecurityLevel>>,
}

impl Default for GetAnomalyDetectionResultsRequest {
    fn default() -> Self {
        Self {
            start_time: 0,
            end_time: 0,
            entity_types: vec![],
            confidence_threshold: None,
            severity_levels: None,
        }
    }
}

/// 异常检测响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAnomalyDetectionResultsResponse {
    /// 异常检测结果
    pub anomalies: Vec<AnomalyDetection>,
    /// 总数量
    pub total_count: i32,
    /// 检测摘要
    pub detection_summary: AnomalyDetectionSummary,
}

/// 异常检测结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyDetection {
    /// 异常ID
    pub anomaly_id: String,
    /// 异常类型
    pub anomaly_type: String,
    /// 置信度分数
    pub confidence_score: f64,
    /// 严重性
    pub severity: SecurityLevel,
    /// 实体类型
    pub entity_type: String,
    /// 实体ID
    pub entity_id: String,
    /// 实体名称
    pub entity_name: String,
    /// 描述
    pub description: String,
    /// 检测时间
    pub detected_at: i64,
    /// 基线行为
    pub baseline_behavior: String,
    /// 异常行为
    pub anomalous_behavior: String,
    /// 风险因素
    pub risk_factors: Vec<String>,
    /// 上下文信息
    pub context_info: AnomalyContext,
    /// 推荐行动
    pub recommended_actions: Vec<String>,
}

/// 异常上下文
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyContext {
    /// IP地址
    pub ip_address: String,
    /// 设备类型
    pub device_type: String,
    /// 位置
    pub location: String,
    /// VPN状态
    pub vpn_status: String,
}

/// 异常检测摘要
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyDetectionSummary {
    /// 检测到的总异常数
    pub total_anomalies_detected: i32,
    /// 高置信度异常数
    pub high_confidence_anomalies: i32,
    /// 关键异常数
    pub critical_anomalies: i32,
    /// 误报率
    pub false_positive_rate: f64,
    /// 检测准确率
    pub detection_accuracy: f64,
    /// 分析时间范围
    pub analysis_time_range: AnalysisTimeRange,
}

/// 分析时间范围
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisTimeRange {
    /// 开始时间
    pub start_time: i64,
    /// 结束时间
    pub end_time: i64,
}

/// 攻击链分析请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAttackChainAnalysisRequest {
    /// 攻击链ID
    pub attack_chain_id: String,
    /// 包含的详细程度
    pub include_indicators: bool,
    /// 包含TTP信息
    pub include_ttp: bool,
}

impl Default for GetAttackChainAnalysisRequest {
    fn default() -> Self {
        Self {
            attack_chain_id: String::new(),
            include_indicators: true,
            include_ttp: true,
        }
    }
}

/// 攻击链分析响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAttackChainAnalysisResponse {
    /// 攻击链ID
    pub attack_chain_id: String,
    /// 攻击名称
    pub attack_name: String,
    /// 当前攻击阶段
    pub attack_stage: AttackStage,
    /// 严重性
    pub severity: SecurityLevel,
    /// 开始时间
    pub start_time: i64,
    /// 最后活动时间
    pub last_activity: i64,
    /// 攻击阶段
    pub attack_stages: Vec<AttackStageInfo>,
    /// 攻击路径
    pub attack_path: AttackPath,
    /// 战术技术
    pub tactics_techniques: Vec<TTP>,
    /// 影响评估
    pub impact_assessment: AttackImpact,
    /// 遏制行动
    pub containment_actions: Vec<String>,
    /// 修复计划
    pub remediation_plan: RemediationPlan,
}

/// 攻击阶段
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttackStage {
    /// 信息收集
    Reconnaissance,
    /// 初始访问
    InitialAccess,
    /// 执行
    Execution,
    /// 持久化
    Persistence,
    /// 权限提升
    PrivilegeEscalation,
    /// 防御规避
    DefenseEvasion,
    /// 凭证访问
    CredentialAccess,
    /// 发现
    Discovery,
    /// 横向移动
    LateralMovement,
    /// 收集
    Collection,
    /// 数据渗出
    Exfiltration,
    /// 影响
    Impact,
}

/// 攻击阶段信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackStageInfo {
    /// 攻击阶段
    pub stage: AttackStage,
    /// 阶段名称
    pub stage_name: String,
    /// 状态
    pub status: StageStatus,
    /// 开始时间
    pub start_time: i64,
    /// 结束时间
    pub end_time: Option<i64>,
    /// 描述
    pub description: String,
    /// 指标
    pub indicators: Vec<String>,
    /// 受影响资产
    pub affected_assets: Vec<String>,
}

/// 阶段状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StageStatus {
    /// 未开始
    NotStarted,
    /// 进行中
    InProgress,
    /// 已完成
    Completed,
    /// 失败
    Failed,
}

/// 攻击路径
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackPath {
    /// 入口点
    pub entry_point: String,
    /// 初始入侵点
    pub initial_compromise: String,
    /// 转折点
    pub pivot_points: Vec<String>,
    /// 最终目标
    pub final_target: String,
    /// 当前位置
    pub current_location: String,
}

/// TTP (战术、技术和程序)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TTP {
    /// 战术
    pub tactic: String,
    /// 技术
    pub technique: String,
    /// 技术ID
    pub technique_id: String,
    /// 描述
    pub description: String,
    /// 观察到的指标
    pub observed_indicators: Vec<String>,
}

/// 攻击影响
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackImpact {
    /// 受影响的资产数
    pub compromised_assets: i32,
    /// 数据泄露情况
    pub data_exfiltrated: String,
    /// 业务中断情况
    pub business_disruption: String,
    /// 财务影响
    pub financial_impact: Option<f64>,
    /// 声誉影响
    pub reputation_impact: String,
}

/// 修复计划
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemediationPlan {
    /// 立即行动
    pub immediate_actions: Vec<String>,
    /// 短期行动
    pub short_term_actions: Vec<String>,
    /// 长期行动
    pub long_term_actions: Vec<String>,
}

// Builder 模式实现

/// 获取实时安全事件请求构建器
///
/// 提供流式API来构建获取实时安全事件的请求参数。
/// 支持链式调用，方便构建复杂的查询条件。
///
/// # 示例
///
/// ```rust,no_run
/// use open_lark::prelude::*;
/// use open_lark::service::security_and_compliance::v1::security_monitoring::*;
///
/// let builder = GetRealTimeSecurityEventsBuilder::new()
///     .limit(20)
///     .severity_filter(vec![SecurityLevel::High, SecurityLevel::Critical])
///     .start_time(1609459200)  // 2021-01-01
///     .end_time(1609545600);    // 2021-01-02
///
/// let request = builder.build();
/// ```

#[derive(Debug, Clone)]
pub struct GetRealTimeSecurityEventsBuilder {
    request: GetRealTimeSecurityEventsRequest,
}

impl GetRealTimeSecurityEventsBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: GetRealTimeSecurityEventsRequest::default(),
        }
    }

    /// 设置限制数量
    pub fn limit(mut self, limit: i32) -> Self {
        self.request.limit = Some(limit);
        self
    }

    /// 设置严重性过滤器
    pub fn severity_filter(mut self, filter: Vec<SecurityLevel>) -> Self {
        self.request.severity_filter = Some(filter);
        self
    }

    /// 设置开始时间
    pub fn start_time(mut self, start_time: i64) -> Self {
        self.request.start_time = start_time;
        self
    }

    /// 设置结束时间
    pub fn end_time(mut self, end_time: i64) -> Self {
        self.request.end_time = end_time;
        self
    }

    /// 设置事件类型过滤器
    pub fn event_type_filter(mut self, filter: Vec<String>) -> Self {
        self.request.event_type_filter = Some(filter);
        self
    }

    /// 设置用户ID过滤器
    pub fn user_id_filter(mut self, user_id: String) -> Self {
        self.request.user_id_filter = Some(user_id);
        self
    }

    /// 设置资源类型过滤器
    pub fn resource_type_filter(mut self, filter: Vec<String>) -> Self {
        self.request.resource_type_filter = Some(filter);
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> GetRealTimeSecurityEventsRequest {
        self.request
    }
}

impl Default for GetRealTimeSecurityEventsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// 应用ExecutableBuilder trait
// crate::impl_executable_builder!(
//    GetRealTimeSecurityEventsBuilder,
//    SecurityMonitoringService,
//    GetRealTimeSecurityEventsRequest,
//    BaseResponse<GetRealTimeSecurityEventsResponse>,
//    get_real_time_security_events
//);

/// 安全态势分析构建器
#[derive(Debug, Clone)]
pub struct GetSecurityPostureAnalysisBuilder {
    //    request: GetSecurityPostureAnalysisRequest,
}

impl GetSecurityPostureAnalysisBuilder {
    //    /// 创建新的Builder实例
    //    pub fn new() -> Self {
    //        Self {
    //            request: GetSecurityPostureAnalysisRequest::default(),
    //        }
    //    }

    //    /// 设置开始时间
    //    pub fn start_time(mut self, start_time: i64) -> Self {
    //        self.request.start_time = start_time;
    //        self
    //    }

    //    /// 设置结束时间
    //    pub fn end_time(mut self, end_time: i64) -> Self {
    //        self.request.end_time = end_time;
    //        self
    //    }

    //    /// 设置分析类型
    //    pub fn analysis_types(mut self, analysis_types: Vec<AnalysisType>) -> Self {
    //        self.request.analysis_types = analysis_types;
    //        self
    //    }

    //    /// 构建最终的请求对象
    //    pub fn build(self) -> GetSecurityPostureAnalysisRequest {
    //        self.request
    //    }
}

impl Default for GetSecurityPostureAnalysisBuilder {
    //    fn default() -> Self {
    //        Self::new()
    //    }
}

// 应用ExecutableBuilder trait
// crate::impl_executable_builder!(
//    GetSecurityPostureAnalysisBuilder,
//    SecurityMonitoringService,
//    GetSecurityPostureAnalysisRequest,
//    BaseResponse<GetSecurityPostureAnalysisResponse>,
//    get_security_posture_analysis
//);

/// 异常检测结果构建器
#[derive(Debug, Clone)]
pub struct GetAnomalyDetectionResultsBuilder {
    //    request: GetAnomalyDetectionResultsRequest,
}

impl GetAnomalyDetectionResultsBuilder {
    //    /// 创建新的Builder实例
    //    pub fn new() -> Self {
    //        Self {
    //            request: GetAnomalyDetectionResultsRequest::default(),
    //        }
    //    }

    //    /// 设置开始时间
    //    pub fn start_time(mut self, start_time: i64) -> Self {
    //        self.request.start_time = start_time;
    //        self
    //    }

    //    /// 设置结束时间
    //    pub fn end_time(mut self, end_time: i64) -> Self {
    //        self.request.end_time = end_time;
    //        self
    //    }

    //    /// 设置实体类型
    //    pub fn entity_types(mut self, entity_types: Vec<String>) -> Self {
    //        self.request.entity_types = entity_types;
    //        self
    //    }

    //    /// 设置置信度阈值
    //    pub fn confidence_threshold(mut self, threshold: f64) -> Self {
    //        self.request.confidence_threshold = Some(threshold);
    //        self
    //    }

    //    /// 设置严重性级别过滤
    //    pub fn severity_levels(mut self, levels: Vec<SecurityLevel>) -> Self {
    //        self.request.severity_levels = Some(levels);
    //        self
    //    }

    //    /// 构建最终的请求对象
    //    pub fn build(self) -> GetAnomalyDetectionResultsRequest {
    //        self.request
    //    }
}

impl Default for GetAnomalyDetectionResultsBuilder {
    //    fn default() -> Self {
    //        Self::new()
    //    }
}

// 应用ExecutableBuilder trait
// crate::impl_executable_builder!(
//    GetAnomalyDetectionResultsBuilder,
//    SecurityMonitoringService,
//    GetAnomalyDetectionResultsRequest,
//    BaseResponse<GetAnomalyDetectionResultsResponse>,
//    get_anomaly_detection_results
//);

/// 攻击链分析构建器
#[derive(Debug, Clone)]
pub struct GetAttackChainAnalysisBuilder {
    //    request: GetAttackChainAnalysisRequest,
}

impl GetAttackChainAnalysisBuilder {
    //    /// 创建新的Builder实例
    //    pub fn new() -> Self {
    //        Self {
    //            request: GetAttackChainAnalysisRequest::default(),
    //        }
    //    }

    //    /// 设置攻击链ID
    //    pub fn attack_chain_id(mut self, attack_chain_id: String) -> Self {
    //        self.request.attack_chain_id = attack_chain_id;
    //        self
    //    }

    //    /// 设置是否包含指标信息
    //    pub fn include_indicators(mut self, include: bool) -> Self {
    //        self.request.include_indicators = include;
    //        self
    //    }

    //    /// 设置是否包含TTP信息
    //    pub fn include_ttp(mut self, include: bool) -> Self {
    //        self.request.include_ttp = include;
    //        self
    //    }

    //    /// 构建最终的请求对象
    //    pub fn build(self) -> GetAttackChainAnalysisRequest {
    //        self.request
    //    }
}

impl Default for GetAttackChainAnalysisBuilder {
    //    fn default() -> Self {
    //        Self::new()
    //    }
}

// 应用ExecutableBuilder trait
// crate::impl_executable_builder!(
//    GetAttackChainAnalysisBuilder,
//    SecurityMonitoringService,
//    GetAttackChainAnalysisRequest,
//    BaseResponse<GetAttackChainAnalysisResponse>,
//    get_attack_chain_analysis
//);

impl SecurityMonitoringService {
    //    /// 获取实时安全事件构建器
    //    pub fn get_real_time_security_events_builder(&self) -> GetRealTimeSecurityEventsBuilder {
    //        GetRealTimeSecurityEventsBuilder::new()
    //    }

    //    /// 获取安全态势分析构建器
    //    pub fn get_security_posture_analysis_builder(&self) -> GetSecurityPostureAnalysisBuilder {
    //        GetSecurityPostureAnalysisBuilder::new()
    //    }

    //    /// 获取异常检测结果构建器
    //    pub fn get_anomaly_detection_results_builder(&self) -> GetAnomalyDetectionResultsBuilder {
    //        GetAnomalyDetectionResultsBuilder::new()
    //    }

    //    /// 获取攻击链分析构建器
    //    pub fn get_attack_chain_analysis_builder(&self) -> GetAttackChainAnalysisBuilder {
    //        GetAttackChainAnalysisBuilder::new()
    //    }
}
