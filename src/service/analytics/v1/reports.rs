#![allow(clippy::derivable_impls)]
#![allow(elided_lifetimes_in_paths)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
//! 报表服务
//!
//! 提供企业级报表生成和管理功能：
//! - 自定义报表创建和管理
//! - 报表模板和样式配置
//! - 报表导出和分发管理
//! - 报表权限和访问控制
//! - 报表数据缓存和性能优化

use crate::core::config::Config;
use crate::core::SDKResult;
use crate::service::analytics::v1::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 报表服务
#[derive(Debug, Clone)]
pub struct ReportService {
    config: Config,
}

impl ReportService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    // ==================== 报表管理 ====================

    /// 创建自定义报表
    pub async fn create_custom_report(
        &self,
        request: &CreateCustomReportRequest,
    ) -> SDKResult<CreateCustomReportResponse> {
        let report_id = format!("report_{}", chrono::Utc::now().timestamp());

        Ok(CreateCustomReportResponse {
            report_id: report_id.clone(),
            report_name: request.report_name.clone(),
            report_description: request.description.clone(),
            report_type: ReportType::Custom,
            template_id: request.template_id.clone(),
            creator_id: request.creator_id.clone(),
            status: ReportStatus::Draft,
            created_at: Some(chrono::Utc::now()),
        })
    }

    /// 获取报表详情
    pub async fn get_report(&self, report_id: &str) -> SDKResult<GetReportResponse> {
        // 模拟实现
        Ok(GetReportResponse {
            report_id: report_id.to_string(),
            report_name: "月度业务分析报表".to_string(),
            report_description: Some("企业月度综合业务数据分析报表".to_string()),
            report_type: ReportType::Custom,
            template_id: Some("template_001".to_string()),
            creator_id: "user_001".to_string(),
            status: ReportStatus::Published,
            created_at: Some(chrono::Utc::now()),
            updated_at: Some(chrono::Utc::now()),
            last_generated_at: Some(chrono::Utc::now() - chrono::Duration::hours(2)),
            next_scheduled_run: Some(chrono::Utc::now() + chrono::Duration::days(30)),
            generated_count: 12,
            view_count: 156,
            export_count: 23,
            schedule_config: Some(ReportScheduleConfig {
                frequency: ScheduleFrequency::Monthly,
                schedule_time: "09:00".to_string(),
                timezone: "Asia/Shanghai".to_string(),
                enabled: true,
            }),
        })
    }

    /// 更新报表
    pub async fn update_report(
        &self,
        report_id: &str,
        request: &UpdateReportRequest,
    ) -> SDKResult<UpdateReportResponse> {
        Ok(UpdateReportResponse {
            report_id: report_id.to_string(),
            report_name: request.report_name.clone(),
            description: request.description.clone(),
            template_id: request.template_id.clone(),
            schedule_config: request.schedule_config.clone(),
            updated_at: Some(chrono::Utc::now()),
        })
    }

    /// 删除报表
    pub async fn delete_report(&self, report_id: &str) -> SDKResult<DeleteReportResponse> {
        Ok(DeleteReportResponse {
            report_id: report_id.to_string(),
            deleted: true,
            deleted_at: Some(chrono::Utc::now()),
        })
    }

    /// 获取报表列表
    pub async fn list_reports(
        &self,
        request: &ListReportsRequest,
    ) -> SDKResult<ListReportsResponse> {
        let reports = vec![
            ReportInfo {
                report_id: "report_001".to_string(),
                report_name: "月度业务分析报表".to_string(),
                report_type: ReportType::Custom,
                creator_id: "admin_001".to_string(),
                status: ReportStatus::Published,
                created_at: Some(chrono::Utc::now()),
                last_generated_at: Some(chrono::Utc::now() - chrono::Duration::hours(2)),
                next_scheduled_run: Some(chrono::Utc::now() + chrono::Duration::days(30)),
                view_count: 156,
            },
            ReportInfo {
                report_id: "report_002".to_string(),
                report_name: "用户活跃度分析报表".to_string(),
                report_type: ReportType::Template,
                creator_id: "admin_001".to_string(),
                status: ReportStatus::Published,
                created_at: Some(chrono::Utc::now() - chrono::Duration::days(7)),
                last_generated_at: Some(chrono::Utc::now() - chrono::Duration::hours(4)),
                next_scheduled_run: Some(chrono::Utc::now() + chrono::Duration::days(7)),
                view_count: 89,
            },
            ReportInfo {
                report_id: "report_003".to_string(),
                report_name: "应用使用统计报表".to_string(),
                report_type: ReportType::System,
                creator_id: "system".to_string(),
                status: ReportStatus::Published,
                created_at: Some(chrono::Utc::now() - chrono::Duration::days(14)),
                last_generated_at: Some(chrono::Utc::now() - chrono::Duration::hours(1)),
                next_scheduled_run: Some(chrono::Utc::now() + chrono::Duration::days(1)),
                view_count: 234,
            },
        ];

        let filtered_reports = if let Some(report_type_filter) = &request.report_type_filter {
            reports
                .iter()
                .filter(|r| &r.report_type == report_type_filter)
                .cloned()
                .collect()
        } else if let Some(status_filter) = &request.status_filter {
            reports
                .iter()
                .filter(|r| &r.status == status_filter)
                .cloned()
                .collect()
        } else {
            reports
        };

        let total_count = filtered_reports.len() as i32;
        Ok(ListReportsResponse {
            reports: filtered_reports,
            total_count,
            has_more: Some(false),
            next_page_token: None,
        })
    }

    // ==================== 报表生成 ====================

    /// 生成报表
    pub async fn generate_report(
        &self,
        request: &GenerateReportRequest,
    ) -> SDKResult<GenerateReportResponse> {
        let generation_id = format!("gen_{}", chrono::Utc::now().timestamp());

        // 模拟报表生成
        let report_data =
            self.generate_report_data(request.report_id.as_str(), &request.parameters);

        Ok(GenerateReportResponse {
            generation_id: generation_id.clone(),
            report_id: request.report_id.clone(),
            generation_status: GenerationStatus::Completed,
            report_data: Some(report_data),
            started_at: Some(chrono::Utc::now()),
            completed_at: Some(chrono::Utc::now()),
            duration_seconds: Some(45),
            file_size_bytes: Some(1024000), // 1MB
            export_formats: vec!["pdf".to_string(), "xlsx".to_string(), "csv".to_string()],
        })
    }

    /// 获取报表数据
    pub async fn get_report_data(
        &self,
        request: &GetReportDataRequest,
    ) -> SDKResult<GetReportDataResponse> {
        Ok(GetReportDataResponse {
            report_id: request.report_id.clone(),
            data_type: request.data_type.clone(),
            report_data: self.generate_report_data(
                request.report_id.as_str(),
                request
                    .parameters
                    .as_ref()
                    .unwrap_or(&std::collections::HashMap::new()),
            ),
            generated_at: Some(chrono::Utc::now()),
            expires_at: Some(chrono::Utc::now() + chrono::Duration::hours(24)),
        })
    }

    /// 预览报表
    pub async fn preview_report(
        &self,
        request: &PreviewReportRequest,
    ) -> SDKResult<PreviewReportResponse> {
        Ok(PreviewReportResponse {
            report_id: request.report_id.clone(),
            preview_data: self.generate_preview_data(request.report_id.as_str()),
            preview_type: request.preview_type.clone(),
            estimated_generation_time: Some(30),
            estimated_file_size: Some(800000), // 800KB
            created_at: Some(chrono::Utc::now()),
        })
    }

    // ==================== 报表导出 ====================

    /// 导出报表
    pub async fn export_report(
        &self,
        request: &ExportReportRequest,
    ) -> SDKResult<ExportReportResponse> {
        let export_id = format!("export_{}", chrono::Utc::now().timestamp());
        let download_url = format!(
            "https://company.com/reports/export/{}.{}",
            export_id, request.format
        );

        Ok(ExportReportResponse {
            export_id: export_id.clone(),
            report_id: request.report_id.clone(),
            format: request.format.clone(),
            download_url,
            file_size: Some(2048576), // 2MB
            expires_at: Some(chrono::Utc::now() + chrono::Duration::hours(24)),
            exported_at: Some(chrono::Utc::now()),
        })
    }

    /// 获取导出历史
    pub async fn get_export_history(
        &self,
        request: &GetExportHistoryRequest,
    ) -> SDKResult<GetExportHistoryResponse> {
        Ok(GetExportHistoryResponse {
            report_id: request.report_id.clone(),
            exports: vec![
                ExportHistory {
                    export_id: "export_001".to_string(),
                    format: "pdf".to_string(),
                    exported_by: "user_001".to_string(),
                    exported_at: chrono::Utc::now() - chrono::Duration::hours(1),
                    file_size: 2048576,
                    download_count: 5,
                    expires_at: chrono::Utc::now() + chrono::Duration::hours(23),
                    status: ExportStatus::Available,
                },
                ExportHistory {
                    export_id: "export_002".to_string(),
                    format: "xlsx".to_string(),
                    exported_by: "user_001".to_string(),
                    exported_at: chrono::Utc::now() - chrono::Duration::days(2),
                    file_size: 1536000,
                    download_count: 12,
                    expires_at: chrono::Utc::now() + chrono::Duration::hours(22),
                    status: ExportStatus::Expired,
                },
            ],
            total_count: 2,
            generated_at: Some(chrono::Utc::now()),
        })
    }

    // ==================== 报表模板管理 ====================

    /// 创建报表模板
    pub async fn create_report_template(
        &self,
        request: &CreateReportTemplateRequest,
    ) -> SDKResult<CreateReportTemplateResponse> {
        let template_id = format!("template_{}", chrono::Utc::now().timestamp());

        Ok(CreateReportTemplateResponse {
            template_id: template_id.clone(),
            template_name: request.template_name.clone(),
            template_description: request.description.clone(),
            template_category: request.category.clone(),
            creator_id: request.creator_id.clone(),
            is_public: request.is_public,
            created_at: Some(chrono::Utc::now()),
        })
    }

    /// 获取报表模板列表
    pub async fn list_report_templates(
        &self,
        _request: &ListReportTemplatesRequest,
    ) -> SDKResult<ListReportTemplatesResponse> {
        Ok(ListReportTemplatesResponse {
            templates: vec![
                ReportTemplateInfo {
                    template_id: "template_001".to_string(),
                    template_name: "月度业务分析模板".to_string(),
                    template_description: Some("标准月度业务数据分析报表模板".to_string()),
                    template_category: "业务分析".to_string(),
                    creator_id: "admin_001".to_string(),
                    is_public: true,
                    usage_count: 45,
                    rating: 4.5,
                    created_at: Some(chrono::Utc::now()),
                },
                ReportTemplateInfo {
                    template_id: "template_002".to_string(),
                    template_name: "用户活跃度分析模板".to_string(),
                    template_description: Some("用户活跃度和行为分析报表模板".to_string()),
                    template_category: "用户分析".to_string(),
                    creator_id: "admin_002".to_string(),
                    is_public: true,
                    usage_count: 32,
                    rating: 4.3,
                    created_at: Some(chrono::Utc::now() - chrono::Duration::days(10)),
                },
                ReportTemplateInfo {
                    template_id: "template_003".to_string(),
                    template_name: "应用使用统计模板".to_string(),
                    template_description: Some("应用使用情况和趋势分析模板".to_string()),
                    template_category: "应用分析".to_string(),
                    creator_id: "admin_001".to_string(),
                    is_public: false,
                    usage_count: 18,
                    rating: 4.1,
                    created_at: Some(chrono::Utc::now() - chrono::Duration::days(5)),
                },
            ],
            total_count: 3,
            generated_at: Some(chrono::Utc::now()),
        })
    }

    // ==================== 报表调度管理 ====================

    /// 设置报表调度
    pub async fn set_report_schedule(
        &self,
        request: &SetReportScheduleRequest,
    ) -> SDKResult<SetReportScheduleResponse> {
        Ok(SetReportScheduleResponse {
            report_id: request.report_id.clone(),
            schedule_config: request.schedule_config.clone(),
            next_run_time: self.calculate_next_run_time(&request.schedule_config),
            updated_at: Some(chrono::Utc::now()),
        })
    }

    /// 获取报表调度信息
    pub async fn get_report_schedule(
        &self,
        report_id: &str,
    ) -> SDKResult<GetReportScheduleResponse> {
        Ok(GetReportScheduleResponse {
            report_id: report_id.to_string(),
            schedule_config: ReportScheduleConfig {
                frequency: ScheduleFrequency::Monthly,
                schedule_time: "09:00".to_string(),
                timezone: "Asia/Shanghai".to_string(),
                enabled: true,
            },
            next_run_time: chrono::Utc::now() + chrono::Duration::days(30),
            last_run_time: Some(chrono::Utc::now() - chrono::Duration::days(30)),
            run_history: vec![
                ScheduleRunHistory {
                    run_id: "run_001".to_string(),
                    scheduled_time: chrono::Utc::now() - chrono::Duration::days(30),
                    actual_time: chrono::Utc::now() - chrono::Duration::days(30)
                        + chrono::Duration::minutes(5),
                    status: RunStatus::Success,
                    duration_seconds: 45,
                },
                ScheduleRunHistory {
                    run_id: "run_002".to_string(),
                    scheduled_time: chrono::Utc::now() - chrono::Duration::days(60),
                    actual_time: chrono::Utc::now() - chrono::Duration::days(60)
                        + chrono::Duration::minutes(12),
                    status: RunStatus::Success,
                    duration_seconds: 52,
                },
            ],
        })
    }

    // ==================== 报表权限管理 ====================

    /// 设置报表权限
    pub async fn set_report_permissions(
        &self,
        request: &SetReportPermissionsRequest,
    ) -> SDKResult<SetReportPermissionsResponse> {
        Ok(SetReportPermissionsResponse {
            report_id: request.report_id.clone(),
            permissions: request.permissions.clone(),
            updated_at: Some(chrono::Utc::now()),
        })
    }

    /// 获取报表权限
    pub async fn get_report_permissions(
        &self,
        report_id: &str,
    ) -> SDKResult<GetReportPermissionsResponse> {
        Ok(GetReportPermissionsResponse {
            report_id: report_id.to_string(),
            permissions: vec![
                ReportPermission {
                    user_id: "user_001".to_string(),
                    user_name: "张三".to_string(),
                    permission_type: PermissionType::View,
                    granted_at: Some(chrono::Utc::now() - chrono::Duration::days(5)),
                    granted_by: "admin_001".to_string(),
                },
                ReportPermission {
                    user_id: "user_002".to_string(),
                    user_name: "李四".to_string(),
                    permission_type: PermissionType::Edit,
                    granted_at: Some(chrono::Utc::now() - chrono::Duration::days(3)),
                    granted_by: "admin_001".to_string(),
                },
                ReportPermission {
                    user_id: "user_003".to_string(),
                    user_name: "王五".to_string(),
                    permission_type: PermissionType::Export,
                    granted_at: Some(chrono::Utc::now() - chrono::Duration::days(1)),
                    granted_by: "admin_001".to_string(),
                },
            ],
        })
    }

    // ==================== 辅助方法 ====================

    fn generate_report_data(
        &self,
        report_id: &str,
        _parameters: &std::collections::HashMap<String, serde_json::Value>,
    ) -> ReportData {
        ReportData {
            report_id: report_id.to_string(),
            title: "月度业务分析报表".to_string(),
            subtitle: Some("2024年10月".to_string()),
            generated_at: chrono::Utc::now(),
            sections: vec![
                ReportSection {
                    section_id: "section_001".to_string(),
                    title: "概览".to_string(),
                    section_type: SectionType::Summary,
                    content: ReportContent::Summary(ReportSummaryData {
                        total_users: 1250,
                        active_users: 1180,
                        total_applications: 15,
                        key_metrics: vec![
                            ("用户活跃度".to_string(), "94.4%".to_string()),
                            ("应用使用率".to_string(), "86.7%".to_string()),
                            ("数据质量评分".to_string(), "92.3".to_string()),
                        ],
                        trend_indicators: vec![
                            TrendIndicator {
                                metric: "用户活跃度".to_string(),
                                current_value: 94.4,
                                previous_value: 91.2,
                                trend: TrendDirection::Up,
                                change_percentage: 3.5,
                            },
                            TrendIndicator {
                                metric: "应用使用率".to_string(),
                                current_value: 86.7,
                                previous_value: 82.3,
                                trend: TrendDirection::Up,
                                change_percentage: 5.3,
                            },
                        ],
                    }),
                },
                ReportSection {
                    section_id: "section_002".to_string(),
                    title: "详细分析".to_string(),
                    section_type: SectionType::Detailed,
                    content: ReportContent::Detailed(ReportDetailedData {
                        charts: vec![
                            ChartData {
                                chart_id: "chart_001".to_string(),
                                chart_type: ChartType::Line,
                                title: "用户活跃度趋势".to_string(),
                                data_source: "user_activity".to_string(),
                                x_axis: ChartAxis {
                                    label: "日期".to_string(),
                                    data_type: "date".to_string(),
                                },
                                y_axis: ChartAxis {
                                    label: "活跃度(%)".to_string(),
                                    data_type: "percentage".to_string(),
                                },
                                data_points: self.generate_chart_data("user_activity", 30),
                            },
                            ChartData {
                                chart_id: "chart_002".to_string(),
                                chart_type: ChartType::Bar,
                                title: "应用使用分布".to_string(),
                                data_source: "app_usage".to_string(),
                                x_axis: ChartAxis {
                                    label: "应用".to_string(),
                                    data_type: "string".to_string(),
                                },
                                y_axis: ChartAxis {
                                    label: "使用率(%)".to_string(),
                                    data_type: "percentage".to_string(),
                                },
                                data_points: self.generate_chart_data("app_usage", 15),
                            },
                        ],
                        tables: vec![TableData {
                            table_id: "table_001".to_string(),
                            title: "部门活跃度统计".to_string(),
                            headers: vec![
                                TableHeader {
                                    name: "部门".to_string(),
                                    data_type: "string".to_string(),
                                    sortable: true,
                                },
                                TableHeader {
                                    name: "用户数".to_string(),
                                    data_type: "number".to_string(),
                                    sortable: true,
                                },
                                TableHeader {
                                    name: "活跃用户数".to_string(),
                                    data_type: "number".to_string(),
                                    sortable: true,
                                },
                                TableHeader {
                                    name: "活跃度".to_string(),
                                    data_type: "percentage".to_string(),
                                    sortable: true,
                                },
                            ],
                            rows: vec![
                                TableRow {
                                    cells: vec![
                                        TableCell {
                                            value: serde_json::Value::String(
                                                "技术研发部".to_string(),
                                            ),
                                        },
                                        TableCell {
                                            value: serde_json::Value::Number(
                                                serde_json::Number::from(450),
                                            ),
                                        },
                                        TableCell {
                                            value: serde_json::Value::Number(
                                                serde_json::Number::from(438),
                                            ),
                                        },
                                        TableCell {
                                            value: "97.3%".into(),
                                        },
                                    ],
                                },
                                TableRow {
                                    cells: vec![
                                        TableCell {
                                            value: serde_json::Value::String(
                                                "市场营销部".to_string(),
                                            ),
                                        },
                                        TableCell {
                                            value: serde_json::Value::Number(
                                                serde_json::Number::from(180),
                                            ),
                                        },
                                        TableCell {
                                            value: serde_json::Value::Number(
                                                serde_json::Number::from(168),
                                            ),
                                        },
                                        TableCell {
                                            value: "93.3%".into(),
                                        },
                                    ],
                                },
                            ],
                        }],
                    }),
                },
            ],
            metadata: ReportMetadata {
                generated_version: "v1.0".to_string(),
                data_sources: vec!["user_activity".to_string(), "app_usage".to_string()],
                generation_duration_seconds: 45,
                data_freshness: "实时".to_string(),
            },
        }
    }

    fn generate_preview_data(&self, report_id: &str) -> ReportPreviewData {
        ReportPreviewData {
            report_id: report_id.to_string(),
            title: "月度业务分析报表".to_string(),
            section_count: 3,
            estimated_pages: 8,
            charts_count: 5,
            tables_count: 3,
            data_points_count: 156,
            preview_sections: vec![
                PreviewSection {
                    section_title: "概览".to_string(),
                    content_preview: "包含关键业务指标概览和趋势分析...".to_string(),
                },
                PreviewSection {
                    section_title: "详细分析".to_string(),
                    content_preview: "包含图表和表格形式的详细数据分析...".to_string(),
                },
            ],
        }
    }

    fn generate_chart_data(&self, data_type: &str, data_points: i32) -> Vec<ChartDataPoint> {
        let mut points = Vec::new();
        let base_date = chrono::Utc::now();

        for i in 0..data_points {
            let date = base_date - chrono::Duration::days((data_points - i - 1) as i64);
            let base_value = match data_type {
                "user_activity" => 90.0 + (i as f64 * 0.1),
                "app_usage" => 80.0 + (i as f64 * 0.2),
                _ => 50.0 + (i as f64 * 0.5),
            };

            let random_factor = (i as f64 * 0.3).sin() * 5.0;
            let value = (base_value + random_factor).min(100.0);

            points.push(ChartDataPoint {
                x: date.format("%Y-%m-%d").to_string(),
                y: value,
                label: Some(format!("{:.1}%", value)),
            });
        }

        points
    }

    fn calculate_next_run_time(&self, schedule_config: &ReportScheduleConfig) -> DateTime<Utc> {
        let now = chrono::Utc::now();

        let days = match schedule_config.frequency {
            ScheduleFrequency::Daily => 1,
            ScheduleFrequency::Weekly => 7,
            ScheduleFrequency::Monthly => 30,
            ScheduleFrequency::Quarterly => 90,
            ScheduleFrequency::Yearly => 365,
        };

        // 简化时间计算，实际应该解析schedule_time
        now + chrono::Duration::days(days)
    }
}

// ==================== 请求数据模型 ====================

/// 创建自定义报表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomReportRequest {
    /// 报表名称
    pub report_name: String,
    /// 描述
    pub description: Option<String>,
    /// 模板ID
    pub template_id: Option<String>,
    /// 创建者ID
    pub creator_id: String,
    /// 报表配置
    pub report_config: Option<ReportConfig>,
}

/// 更新报表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateReportRequest {
    /// 报表名称
    pub report_name: Option<String>,
    /// 描述
    pub description: Option<String>,
    /// 模板ID
    pub template_id: Option<String>,
    /// 调度配置
    pub schedule_config: Option<ReportScheduleConfig>,
}

/// 获取报表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetReportRequest {
    /// 报表ID
    pub report_id: String,
}

/// 删除报表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteReportRequest {
    /// 报表ID
    pub report_id: String,
}

/// 报表列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListReportsRequest {
    /// 页面大小
    pub page_size: Option<i32>,
    /// 页面标记
    pub page_token: Option<String>,
    /// 报表类型过滤
    pub report_type_filter: Option<ReportType>,
    /// 状态过滤
    pub status_filter: Option<ReportStatus>,
    /// 创建者过滤
    pub creator_filter: Option<String>,
}

/// 生成报表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateReportRequest {
    /// 报表ID
    pub report_id: String,
    /// 生成参数
    pub parameters: std::collections::HashMap<String, serde_json::Value>,
    /// 是否强制重新生成
    pub force_regenerate: Option<bool>,
}

/// 获取报表数据请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetReportDataRequest {
    /// 报表ID
    pub report_id: String,
    /// 数据类型
    pub data_type: String,
    /// 参数
    pub parameters: Option<std::collections::HashMap<String, serde_json::Value>>,
}

/// 预览报表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewReportRequest {
    /// 报表ID
    pub report_id: String,
    /// 预览类型
    pub preview_type: PreviewType,
}

/// 导出报表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportReportRequest {
    /// 报表ID
    pub report_id: String,
    /// 导出格式
    pub format: String,
    /// 导出参数
    pub export_options: Option<ExportOptions>,
}

/// 获取导出历史请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetExportHistoryRequest {
    /// 报表ID
    pub report_id: String,
    /// 页面大小
    pub page_size: Option<i32>,
    /// 页面标记
    pub page_token: Option<String>,
}

/// 创建报表模板请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateReportTemplateRequest {
    /// 模板名称
    pub template_name: String,
    /// 描述
    pub description: Option<String>,
    /// 分类
    pub category: String,
    /// 创建者ID
    pub creator_id: String,
    /// 是否公开
    pub is_public: bool,
}

/// 报表模板列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListReportTemplatesRequest {
    /// 分类过滤
    pub category_filter: Option<String>,
    /// 是否只显示公开模板
    pub public_only: Option<bool>,
    /// 使用次数过滤
    pub min_usage_count: Option<i32>,
}

/// 设置报表调度请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetReportScheduleRequest {
    /// 报表ID
    pub report_id: String,
    /// 调度配置
    pub schedule_config: ReportScheduleConfig,
}

/// 获取报表调度信息请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetReportScheduleRequest {
    /// 报表ID
    pub report_id: String,
}

/// 设置报表权限请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetReportPermissionsRequest {
    /// 报表ID
    pub report_id: String,
    /// 权限列表
    pub permissions: Vec<ReportPermission>,
}

/// 获取报表权限请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetReportPermissionsRequest {
    /// 报表ID
    pub report_id: String,
}

// ==================== 响应数据模型 ====================

/// 创建自定义报表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomReportResponse {
    /// 报表ID
    pub report_id: String,
    /// 报表名称
    pub report_name: String,
    /// 报表描述
    pub report_description: Option<String>,
    /// 报表类型
    pub report_type: ReportType,
    /// 模板ID
    pub template_id: Option<String>,
    /// 创建者ID
    pub creator_id: String,
    /// 状态
    pub status: ReportStatus,
    /// 创建时间
    pub created_at: Option<DateTime<Utc>>,
}

/// 获取报表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetReportResponse {
    /// 报表ID
    pub report_id: String,
    /// 报表名称
    pub report_name: String,
    /// 报表描述
    pub report_description: Option<String>,
    /// 报表类型
    pub report_type: ReportType,
    /// 模板ID
    pub template_id: Option<String>,
    /// 创建者ID
    pub creator_id: String,
    /// 状态
    pub status: ReportStatus,
    /// 创建时间
    pub created_at: Option<DateTime<Utc>>,
    /// 更新时间
    pub updated_at: Option<DateTime<Utc>>,
    /// 最后生成时间
    pub last_generated_at: Option<DateTime<Utc>>,
    /// 下次计划运行时间
    pub next_scheduled_run: Option<DateTime<Utc>>,
    /// 生成次数
    pub generated_count: i32,
    /// 查看次数
    pub view_count: i32,
    /// 导出次数
    pub export_count: i32,
    /// 调度配置
    pub schedule_config: Option<ReportScheduleConfig>,
}

/// 更新报表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateReportResponse {
    /// 报表ID
    pub report_id: String,
    /// 报表名称
    pub report_name: Option<String>,
    /// 描述
    pub description: Option<String>,
    /// 模板ID
    pub template_id: Option<String>,
    /// 调度配置
    pub schedule_config: Option<ReportScheduleConfig>,
    /// 更新时间
    pub updated_at: Option<DateTime<Utc>>,
}

/// 删除报表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteReportResponse {
    /// 报表ID
    pub report_id: String,
    /// 是否删除成功
    pub deleted: bool,
    /// 删除时间
    pub deleted_at: Option<DateTime<Utc>>,
}

/// 报表列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListReportsResponse {
    /// 报表列表
    pub reports: Vec<ReportInfo>,
    /// 总数量
    pub total_count: i32,
    /// 是否有更多
    pub has_more: Option<bool>,
    /// 下一页标记
    pub next_page_token: Option<String>,
}

/// 生成报表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateReportResponse {
    /// 生成ID
    pub generation_id: String,
    /// 报表ID
    pub report_id: String,
    /// 生成状态
    pub generation_status: GenerationStatus,
    /// 报表数据
    pub report_data: Option<ReportData>,
    /// 开始时间
    pub started_at: Option<DateTime<Utc>>,
    /// 完成时间
    pub completed_at: Option<DateTime<Utc>>,
    /// 持续时间(秒)
    pub duration_seconds: Option<i64>,
    /// 文件大小(字节)
    pub file_size_bytes: Option<i64>,
    /// 支持的导出格式
    pub export_formats: Vec<String>,
}

/// 获取报表数据响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetReportDataResponse {
    /// 报表ID
    pub report_id: String,
    /// 数据类型
    pub data_type: String,
    /// 报表数据
    pub report_data: ReportData,
    /// 生成时间
    pub generated_at: Option<DateTime<Utc>>,
    /// 过期时间
    pub expires_at: Option<DateTime<Utc>>,
}

/// 预览报表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewReportResponse {
    /// 报表ID
    pub report_id: String,
    /// 预览数据
    pub preview_data: ReportPreviewData,
    /// 预览类型
    pub preview_type: PreviewType,
    /// 预估生成时间(秒)
    pub estimated_generation_time: Option<i32>,
    /// 预估文件大小(字节)
    pub estimated_file_size: Option<i64>,
    /// 创建时间
    pub created_at: Option<DateTime<Utc>>,
}

/// 导出报表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportReportResponse {
    /// 导出ID
    pub export_id: String,
    /// 报表ID
    pub report_id: String,
    /// 导出格式
    pub format: String,
    /// 下载链接
    pub download_url: String,
    /// 文件大小
    pub file_size: Option<i64>,
    /// 过期时间
    pub expires_at: Option<DateTime<Utc>>,
    /// 导出时间
    pub exported_at: Option<DateTime<Utc>>,
}

/// 获取导出历史响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetExportHistoryResponse {
    /// 报表ID
    pub report_id: String,
    /// 导出记录
    pub exports: Vec<ExportHistory>,
    /// 总数量
    pub total_count: i32,
    /// 生成时间
    pub generated_at: Option<DateTime<Utc>>,
}

/// 创建报表模板响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateReportTemplateResponse {
    /// 模板ID
    pub template_id: String,
    /// 模板名称
    pub template_name: String,
    /// 模板描述
    pub template_description: Option<String>,
    /// 模板分类
    pub template_category: String,
    /// 创建者ID
    pub creator_id: String,
    /// 是否公开
    pub is_public: bool,
    /// 创建时间
    pub created_at: Option<DateTime<Utc>>,
}

/// 报表模板列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListReportTemplatesResponse {
    /// 模板列表
    pub templates: Vec<ReportTemplateInfo>,
    /// 总数量
    pub total_count: i32,
    /// 生成时间
    pub generated_at: Option<DateTime<Utc>>,
}

/// 设置报表调度响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetReportScheduleResponse {
    /// 报表ID
    pub report_id: String,
    /// 调度配置
    pub schedule_config: ReportScheduleConfig,
    /// 下次运行时间
    pub next_run_time: DateTime<Utc>,
    /// 更新时间
    pub updated_at: Option<DateTime<Utc>>,
}

/// 获取报表调度信息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetReportScheduleResponse {
    /// 报表ID
    pub report_id: String,
    /// 调度配置
    pub schedule_config: ReportScheduleConfig,
    /// 下次运行时间
    pub next_run_time: DateTime<Utc>,
    /// 上次运行时间
    pub last_run_time: Option<DateTime<Utc>>,
    /// 运行历史
    pub run_history: Vec<ScheduleRunHistory>,
}

/// 设置报表权限响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetReportPermissionsResponse {
    /// 报表ID
    pub report_id: String,
    /// 权限列表
    pub permissions: Vec<ReportPermission>,
    /// 更新时间
    pub updated_at: Option<DateTime<Utc>>,
}

/// 获取报表权限响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetReportPermissionsResponse {
    /// 报表ID
    pub report_id: String,
    /// 权限列表
    pub permissions: Vec<ReportPermission>,
}

// ==================== 数据模型 ====================

/// 报表信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportInfo {
    /// 报表ID
    pub report_id: String,
    /// 报表名称
    pub report_name: String,
    /// 报表类型
    pub report_type: ReportType,
    /// 创建者ID
    pub creator_id: String,
    /// 状态
    pub status: ReportStatus,
    /// 创建时间
    pub created_at: Option<DateTime<Utc>>,
    /// 最后生成时间
    pub last_generated_at: Option<DateTime<Utc>>,
    /// 下次计划运行时间
    pub next_scheduled_run: Option<DateTime<Utc>>,
    /// 查看次数
    pub view_count: i32,
}

/// 报表类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ReportType {
    /// 系统报表
    System,
    /// 模板报表
    Template,
    /// 自定义报表
    Custom,
}

/// 报表状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ReportStatus {
    /// 草稿
    Draft,
    /// 已发布
    Published,
    /// 生成中
    Generating,
    /// 已暂停
    Paused,
    /// 已归档
    Archived,
    /// 错误
    Error,
}

/// 生成状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GenerationStatus {
    /// 等待中
    Pending,
    /// 生成中
    InProgress,
    /// 已完成
    Completed,
    /// 失败
    Failed,
    /// 已取消
    Cancelled,
}

/// 预览类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewType {
    /// 摘要预览
    Summary,
    /// 详细预览
    Detailed,
    /// 数据预览
    DataOnly,
}

/// 导出状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportStatus {
    /// 可用
    Available,
    /// 已过期
    Expired,
    /// 下载中
    Downloading,
    /// 失败
    Failed,
}

/// 调度频率
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScheduleFrequency {
    /// 每日
    Daily,
    /// 每周
    Weekly,
    /// 每月
    Monthly,
    /// 每季度
    Quarterly,
    /// 每年
    Yearly,
}

/// 调度配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportScheduleConfig {
    /// 调度频率
    pub frequency: ScheduleFrequency,
    /// 调度时间
    pub schedule_time: String,
    /// 时区
    pub timezone: String,
    /// 是否启用
    pub enabled: bool,
}

/// 调度运行历史
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleRunHistory {
    /// 运行ID
    pub run_id: String,
    /// 计划时间
    pub scheduled_time: DateTime<Utc>,
    /// 实际时间
    pub actual_time: DateTime<Utc>,
    /// 运行状态
    pub status: RunStatus,
    /// 持续时间(秒)
    pub duration_seconds: i64,
}

/// 运行状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunStatus {
    /// 成功
    Success,
    /// 失败
    Failed,
    /// 跳过
    Skipped,
    /// 部分成功
    PartialSuccess,
}

/// 报表权限
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportPermission {
    /// 用户ID
    pub user_id: String,
    /// 用户姓名
    pub user_name: String,
    /// 权限类型
    pub permission_type: PermissionType,
    /// 授权时间
    pub granted_at: Option<DateTime<Utc>>,
    /// 授权人
    pub granted_by: String,
}

/// 权限类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PermissionType {
    /// 查看
    View,
    /// 编辑
    Edit,
    /// 导出
    Export,
    /// 删除
    Delete,
    /// 管理
    Admin,
}

/// 报表模板信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportTemplateInfo {
    /// 模板ID
    pub template_id: String,
    /// 模板名称
    pub template_name: String,
    /// 模板描述
    pub template_description: Option<String>,
    /// 模板分类
    pub template_category: String,
    /// 创建者ID
    pub creator_id: String,
    /// 是否公开
    pub is_public: bool,
    /// 使用次数
    pub usage_count: i32,
    /// 评分
    pub rating: f64,
    /// 创建时间
    pub created_at: Option<DateTime<Utc>>,
}

/// 导出历史
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportHistory {
    /// 导出ID
    pub export_id: String,
    /// 导出格式
    pub format: String,
    /// 导出者
    pub exported_by: String,
    /// 导出时间
    pub exported_at: DateTime<Utc>,
    /// 文件大小
    pub file_size: i64,
    /// 下载次数
    pub download_count: i32,
    /// 过期时间
    pub expires_at: DateTime<Utc>,
    /// 状态
    pub status: ExportStatus,
}

/// 报表数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportData {
    /// 报表ID
    pub report_id: String,
    /// 标题
    pub title: String,
    /// 副标题
    pub subtitle: Option<String>,
    /// 生成时间
    pub generated_at: DateTime<Utc>,
    /// 报表章节
    pub sections: Vec<ReportSection>,
    /// 元数据
    pub metadata: ReportMetadata,
}

/// 报表章节
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportSection {
    /// 章节ID
    pub section_id: String,
    /// 章节标题
    pub title: String,
    /// 章节类型
    pub section_type: SectionType,
    /// 章节内容
    pub content: ReportContent,
}

/// 报表内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReportContent {
    /// 汇总数据
    Summary(ReportSummaryData),
    /// 详细数据
    Detailed(ReportDetailedData),
    /// 图表数据
    Charts(Vec<ChartData>),
    /// 表格数据
    Tables(Vec<TableData>),
}

/// 报表汇总数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportSummaryData {
    /// 总用户数
    pub total_users: i32,
    /// 活跃用户数
    pub active_users: i32,
    /// 总应用数
    pub total_applications: i32,
    /// 关键指标
    pub key_metrics: Vec<(String, String)>,
    /// 趋势指标
    pub trend_indicators: Vec<TrendIndicator>,
}

/// 趋势指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendIndicator {
    /// 指标名称
    pub metric: String,
    /// 当前值
    pub current_value: f64,
    /// 之前值
    pub previous_value: f64,
    /// 趋势方向
    pub trend: TrendDirection,
    /// 变化百分比
    pub change_percentage: f64,
}

/// 报表详细数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportDetailedData {
    /// 图表列表
    pub charts: Vec<ChartData>,
    /// 表格列表
    pub tables: Vec<TableData>,
}

/// 图表数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartData {
    /// 图表ID
    pub chart_id: String,
    /// 图表类型
    pub chart_type: ChartType,
    /// 图表标题
    pub title: String,
    /// 数据源
    pub data_source: String,
    /// X轴配置
    pub x_axis: ChartAxis,
    /// Y轴配置
    pub y_axis: ChartAxis,
    /// 数据点
    pub data_points: Vec<ChartDataPoint>,
}

/// 图表类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChartType {
    /// 线图
    Line,
    /// 柱状图
    Bar,
    /// 饼图
    Pie,
    /// 散点图
    Scatter,
    /// 热力图
    Heatmap,
    /// 面积图
    Area,
    /// 雷达图
    Radar,
}

/// 图表轴配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartAxis {
    /// 标签
    pub label: String,
    /// 数据类型
    pub data_type: String,
}

/// 图表数据点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartDataPoint {
    /// X值
    pub x: String,
    /// Y值
    pub y: f64,
    /// 标签
    pub label: Option<String>,
}

/// 表格数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableData {
    /// 表格ID
    pub table_id: String,
    /// 表格标题
    pub title: String,
    /// 表头
    pub headers: Vec<TableHeader>,
    /// 表格行
    pub rows: Vec<TableRow>,
}

/// 表头
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableHeader {
    /// 名称
    pub name: String,
    /// 数据类型
    pub data_type: String,
    /// 是否可排序
    pub sortable: bool,
}

/// 表格行
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableRow {
    /// 单元格
    pub cells: Vec<TableCell>,
}

/// 表格单元格
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableCell {
    /// 值
    pub value: serde_json::Value,
}

/// 报表元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportMetadata {
    /// 生成版本
    pub generated_version: String,
    /// 数据源
    pub data_sources: Vec<String>,
    /// 生成耗时(秒)
    pub generation_duration_seconds: i64,
    /// 数据新鲜度
    pub data_freshness: String,
}

/// 预览数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportPreviewData {
    /// 报表ID
    pub report_id: String,
    /// 报表标题
    pub title: String,
    /// 章节数量
    pub section_count: i32,
    /// 预估页数
    pub estimated_pages: i32,
    /// 图表数量
    pub charts_count: i32,
    /// 表格数量
    pub tables_count: i32,
    /// 数据点数量
    pub data_points_count: i32,
    /// 预览章节
    pub preview_sections: Vec<PreviewSection>,
}

/// 预览章节
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewSection {
    /// 章节标题
    pub section_title: String,
    /// 内容预览
    pub content_preview: String,
}

/// 报表配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportConfig {
    /// 页面设置
    pub page_settings: PageSettings,
    /// 样式设置
    style_settings: StyleSettings,
    /// 数据过滤设置
    pub data_filters: Vec<DataFilter>,
    /// 排序设置
    sort_settings: Vec<SortSetting>,
}

/// 页面设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageSettings {
    /// 页面大小
    pub page_size: i32,
    /// 页面方向
    page_orientation: PageOrientation,
    /// 页边距
    pub margins: PageMargins,
}

/// 页面方向
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PageOrientation {
    /// 纵向
    Portrait,
    /// 横向
    Landscape,
}

/// 页边距
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageMargins {
    /// 上边距
    pub top: f64,
    /// 右边距
    pub right: f64,
    /// 下边距
    pub bottom: f64,
    /// 左边距
    pub left: f64,
}

/// 样式设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleSettings {
    /// 主题
    pub theme: ReportTheme,
    /// 字体设置
    pub font_settings: FontSettings,
    /// 颜色设置
    pub color_settings: ColorSettings,
    /// 布局设置
    pub layout_settings: LayoutSettings,
}

/// 报表主题
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportTheme {
    /// 默认主题
    Default,
    /// 商业主题
    Business,
    /// 简约主题
    Minimal,
    /// 深色主题
    Dark,
    /// 自定义主题
    Custom,
}

/// 字体设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontSettings {
    /// 标题字体
    pub title_font: String,
    /// 正文字体
    pub body_font: String,
    /// 字号
    pub font_size: f64,
    /// 行高
    pub line_height: f64,
}

/// 颜色设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorSettings {
    /// 主色调
    pub primary_color: String,
    /// 辅助色
    pub secondary_color: String,
    /// 强调色
    pub accent_color: String,
    /// 背景色
    pub background_color: String,
    /// 文字颜色
    pub text_color: String,
}

/// 布局设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutSettings {
    /// 页眉设置
    pub header_settings: HeaderSettings,
    /// 页脚设置
    pub footer_settings: FooterSettings,
    /// 图表布局
    pub chart_layout: ChartLayout,
}

/// 页眉设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeaderSettings {
    /// 是否显示页眉
    pub show_header: bool,
    /// 标题对齐
    pub title_alignment: Alignment,
    /// 页眉高度
    pub height: f64,
}

/// 页脚设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FooterSettings {
    /// 是否显示页脚
    pub show_footer: bool,
    /// 文本对齐
    pub text_alignment: Alignment,
    /// 页脚高度
    pub height: f64,
}

/// 图表布局
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartLayout {
    /// 图表排列方式
    pub arrangement: ChartArrangement,
    /// 图表间距
    pub spacing: f64,
}

/// 图表排列方式
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChartArrangement {
    /// 氃整
    Auto,
    /// 氃整
    Manual,
    /// 网格
    Grid,
    /// 垂直排列
    Vertical,
    /// 水平排列
    Horizontal,
}

/// 数据过滤器
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataFilter {
    /// 字段名
    pub field: String,
    /// 操作符
    pub operator: FilterOperator,
    /// 值
    pub value: serde_json::Value,
    /// 逻辑连接
    pub logic_operator: Option<LogicOperator>,
}

/// 过滤操作符
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FilterOperator {
    /// 等于
    Equals,
    /// 不等于
    NotEquals,
    /// 大于
    GreaterThan,
    /// 大于等于
    GreaterThanOrEqual,
    /// 小于
    LessThan,
    /// 小于等于
    LessThanOrEqual,
    /// 包含
    Contains,
    /// 不包含
    NotContains,
    /// 开始于
    StartsWith,
    /// 结束于
    EndsWith,
    /// 为空
    IsEmpty,
    /// 不为空
    IsNotEmpty,
}

/// 逻辑连接符
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LogicOperator {
    /// 与
    And,
    /// 或
    Or,
}

/// 排序设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortSetting {
    /// 字段名
    pub field: String,
    /// 排序方向
    pub direction: SortDirection,
    /// 优先级
    pub priority: i32,
}

/// 排序方向
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortDirection {
    /// 升序
    Ascending,
    /// 降序
    Descending,
}

/// 导出选项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportOptions {
    /// 包含原始数据
    pub include_raw_data: Option<bool>,
    /// 数据格式
    pub data_format: String,
    /// 压缩选项
    pub compression: Option<String>,
    /// 氃整选项
    pub formatting: Option<FormattingOptions>,
}

/// 格式化选项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormattingOptions {
    /// 数字格式
    pub number_format: Option<String>,
    /// 日期格式
    pub date_format: Option<String>,
    /// 货币格式
    pub currency_format: Option<String>,
}

// 实现Default trait
impl Default for CreateCustomReportRequest {
    fn default() -> Self {
        Self {
            report_name: String::new(),
            description: None,
            template_id: None,
            creator_id: String::new(),
            report_config: None,
        }
    }
}

impl Default for UpdateReportRequest {
    fn default() -> Self {
        Self {
            report_name: None,
            description: None,
            template_id: None,
            schedule_config: None,
        }
    }
}

impl Default for ListReportsRequest {
    fn default() -> Self {
        Self {
            page_size: Some(20),
            page_token: None,
            report_type_filter: None,
            status_filter: None,
            creator_filter: None,
        }
    }
}

impl Default for GenerateReportRequest {
    fn default() -> Self {
        Self {
            report_id: String::new(),
            parameters: std::collections::HashMap::new(),
            force_regenerate: Some(false),
        }
    }
}

impl Default for GetReportDataRequest {
    fn default() -> Self {
        Self {
            report_id: String::new(),
            data_type: "full".to_string(),
            parameters: None,
        }
    }
}

impl Default for PreviewReportRequest {
    fn default() -> Self {
        Self {
            report_id: String::new(),
            preview_type: PreviewType::Summary,
        }
    }
}

impl Default for ExportReportRequest {
    fn default() -> Self {
        Self {
            report_id: String::new(),
            format: "pdf".to_string(),
            export_options: None,
        }
    }
}

impl Default for GetExportHistoryRequest {
    fn default() -> Self {
        Self {
            report_id: String::new(),
            page_size: Some(20),
            page_token: None,
        }
    }
}

impl Default for CreateReportTemplateRequest {
    fn default() -> Self {
        Self {
            template_name: String::new(),
            description: None,
            category: String::new(),
            creator_id: String::new(),
            is_public: false,
        }
    }
}

impl Default for ListReportTemplatesRequest {
    fn default() -> Self {
        Self {
            category_filter: None,
            public_only: Some(true),
            min_usage_count: Some(5),
        }
    }
}

impl Default for SetReportScheduleRequest {
    fn default() -> Self {
        Self {
            report_id: String::new(),
            schedule_config: ReportScheduleConfig {
                frequency: ScheduleFrequency::Monthly,
                schedule_time: "09:00".to_string(),
                timezone: "Asia/Shanghai".to_string(),
                enabled: true,
            },
        }
    }
}

impl Default for GetReportScheduleRequest {
    fn default() -> Self {
        Self {
            report_id: String::new(),
        }
    }
}

impl Default for SetReportPermissionsRequest {
    fn default() -> Self {
        Self {
            report_id: String::new(),
            permissions: vec![],
        }
    }
}

impl Default for GetReportPermissionsRequest {
    fn default() -> Self {
        Self {
            report_id: String::new(),
        }
    }
}

impl Default for ReportScheduleConfig {
    fn default() -> Self {
        Self {
            frequency: ScheduleFrequency::Monthly,
            schedule_time: "09:00".to_string(),
            timezone: "Asia/Shanghai".to_string(),
            enabled: true,
        }
    }
}

impl Default for ReportMetadata {
    fn default() -> Self {
        Self {
            generated_version: "v1.0".to_string(),
            data_sources: vec![],
            generation_duration_seconds: 0,
            data_freshness: "实时".to_string(),
        }
    }
}

impl Default for ReportPreviewData {
    fn default() -> Self {
        Self {
            report_id: String::new(),
            title: String::new(),
            section_count: 0,
            estimated_pages: 0,
            charts_count: 0,
            tables_count: 0,
            data_points_count: 0,
            preview_sections: vec![],
        }
    }
}

impl Default for ReportConfig {
    fn default() -> Self {
        Self {
            page_settings: PageSettings {
                page_size: 20,
                page_orientation: PageOrientation::Portrait,
                margins: PageMargins {
                    top: 20.0,
                    right: 20.0,
                    bottom: 20.0,
                    left: 20.0,
                },
            },
            style_settings: StyleSettings {
                theme: ReportTheme::Business,
                font_settings: FontSettings {
                    title_font: "Arial".to_string(),
                    body_font: "Arial".to_string(),
                    font_size: 12.0,
                    line_height: 1.5,
                },
                color_settings: ColorSettings {
                    primary_color: "#1890ff".to_string(),
                    secondary_color: "#6c757d".to_string(),
                    accent_color: "#28a745".to_string(),
                    background_color: "#ffffff".to_string(),
                    text_color: "#333333".to_string(),
                },
                layout_settings: LayoutSettings {
                    header_settings: HeaderSettings {
                        show_header: true,
                        title_alignment: Alignment::Center,
                        height: 60.0,
                    },
                    footer_settings: FooterSettings {
                        show_footer: true,
                        text_alignment: Alignment::Center,
                        height: 30.0,
                    },
                    chart_layout: ChartLayout {
                        arrangement: ChartArrangement::Auto,
                        spacing: 20.0,
                    },
                },
            },
            data_filters: vec![],
            sort_settings: vec![],
        }
    }
}
