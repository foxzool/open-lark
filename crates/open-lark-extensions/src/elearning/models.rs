use serde::{Deserialize, Serialize};

/// 分页响应基础结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageResponse<T> {
    /// 数据项列表
    pub items: Vec<T>,
    /// 分页标记，用于获取下一页数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// 课程学习进度记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseRegistration {
    /// 学习进度记录ID
    pub registration_id: String,
    /// 课程ID
    pub course_id: String,
    /// 用户ID
    pub user_id: String,
    /// 注册类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_type: Option<String>,
    /// 学习状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 学习进度（百分比）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<f64>,
    /// 开始学习时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 完成学习时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_time: Option<i64>,
    /// 总学习时长（秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_duration: Option<i64>,
    /// 已学习时长（秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub studied_duration: Option<i64>,
    /// 学习成绩
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    /// 是否通过
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passed: Option<bool>,
    /// 课程信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub course_info: Option<CourseInfo>,
    /// 用户信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_info: Option<UserInfo>,
    /// 学习记录详情
    #[serde(skip_serializing_if = "Option::is_none")]
    pub learning_records: Option<Vec<LearningRecord>>,
    /// 创建时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// 更新时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
}

/// 课程信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseInfo {
    /// 课程ID
    pub course_id: String,
    /// 课程名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub course_name: Option<String>,
    /// 课程描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 课程类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub course_type: Option<String>,
    /// 课程分类
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// 课程标签
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// 课程时长（秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// 课程难度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub difficulty: Option<String>,
    /// 课程封面URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_url: Option<String>,
    /// 课程创建者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    /// 课程状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 用户邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 用户头像URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    /// 用户部门
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
    /// 用户职位
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// 学习记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningRecord {
    /// 记录ID
    pub record_id: String,
    /// 学习章节ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chapter_id: Option<String>,
    /// 章节名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chapter_name: Option<String>,
    /// 学习进度（百分比）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<f64>,
    /// 学习时长（秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// 是否完成
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed: Option<bool>,
    /// 学习开始时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 学习结束时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 学习次数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempt_count: Option<i32>,
}

/// 学习统计数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningStatistics {
    /// 总课程数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_courses: Option<i64>,
    /// 已完成课程数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_courses: Option<i64>,
    /// 进行中课程数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_progress_courses: Option<i64>,
    /// 总学习时长（秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_study_time: Option<i64>,
    /// 平均学习进度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_progress: Option<f64>,
    /// 平均成绩
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_score: Option<f64>,
    /// 通过率
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pass_rate: Option<f64>,
}

/// 学习进度事件数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseRegistrationEvent {
    /// 事件类型
    pub event_type: String,
    /// 学习进度记录
    pub registration: CourseRegistration,
    /// 变更前数据（更新事件时提供）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_registration: Option<CourseRegistration>,
    /// 事件时间戳
    pub timestamp: i64,
    /// 事件来源
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_page_response_serialization() {
        let page_response: PageResponse<CourseInfo> = PageResponse {
            items: vec![CourseInfo {
                course_id: "course_123".to_string(),
                course_name: Some("Rust编程基础".to_string()),
                description: Some("学习Rust编程语言基础知识".to_string()),
                course_type: Some("programming".to_string()),
                category: Some("技术".to_string()),
                tags: Some(vec!["rust".to_string(), "programming".to_string()]),
                duration: Some(7200),
                difficulty: Some("beginner".to_string()),
                cover_url: Some("https://example.com/course.jpg".to_string()),
                creator: Some("teacher_001".to_string()),
                status: Some("active".to_string()),
            }],
            page_token: Some("next_page_token".to_string()),
            has_more: Some(true),
        };

        let serialized = serde_json::to_string(&page_response).unwrap();
        let deserialized: PageResponse<CourseInfo> = serde_json::from_str(&serialized).unwrap();

        assert_eq!(page_response.items.len(), deserialized.items.len());
        assert_eq!(page_response.page_token, deserialized.page_token);
        assert_eq!(page_response.has_more, deserialized.has_more);
        assert_eq!(
            page_response.items[0].course_id,
            deserialized.items[0].course_id
        );
        assert_eq!(
            page_response.items[0].course_name,
            deserialized.items[0].course_name
        );
    }

    #[test]
    fn test_page_response_with_none_values() {
        let page_response: PageResponse<String> = PageResponse {
            items: vec!["item1".to_string(), "item2".to_string()],
            page_token: None,
            has_more: None,
        };

        let serialized = serde_json::to_string(&page_response).unwrap();
        assert!(!serialized.contains("page_token"));
        assert!(!serialized.contains("has_more"));

        let deserialized: PageResponse<String> = serde_json::from_str(&serialized).unwrap();
        assert_eq!(page_response.items, deserialized.items);
        assert_eq!(page_response.page_token, deserialized.page_token);
        assert_eq!(page_response.has_more, deserialized.has_more);
    }

    #[test]
    fn test_course_registration_serialization() {
        let registration = CourseRegistration {
            registration_id: "reg_123".to_string(),
            course_id: "course_456".to_string(),
            user_id: "user_789".to_string(),
            registration_type: Some("required".to_string()),
            status: Some("in_progress".to_string()),
            progress: Some(65.5),
            start_time: Some(1640995200),
            completion_time: None,
            total_duration: Some(7200),
            studied_duration: Some(4716),
            score: Some(85.0),
            passed: Some(false),
            course_info: Some(CourseInfo {
                course_id: "course_456".to_string(),
                course_name: Some("JavaScript高级教程".to_string()),
                description: None,
                course_type: Some("advanced".to_string()),
                category: Some("前端开发".to_string()),
                tags: Some(vec!["javascript".to_string(), "advanced".to_string()]),
                duration: Some(7200),
                difficulty: Some("advanced".to_string()),
                cover_url: None,
                creator: Some("teacher_002".to_string()),
                status: Some("active".to_string()),
            }),
            user_info: Some(UserInfo {
                user_id: "user_789".to_string(),
                name: Some("张三".to_string()),
                email: Some("zhangsan@example.com".to_string()),
                avatar_url: Some("https://example.com/avatar.jpg".to_string()),
                department: Some("技术部".to_string()),
                position: Some("软件工程师".to_string()),
            }),
            learning_records: Some(vec![LearningRecord {
                record_id: "record_001".to_string(),
                chapter_id: Some("chapter_1".to_string()),
                chapter_name: Some("变量与数据类型".to_string()),
                progress: Some(100.0),
                duration: Some(1800),
                completed: Some(true),
                start_time: Some(1640995200),
                end_time: Some(1640997000),
                attempt_count: Some(1),
            }]),
            created_at: Some(1640995200),
            updated_at: Some(1640999000),
        };

        let serialized = serde_json::to_string(&registration).unwrap();
        let deserialized: CourseRegistration = serde_json::from_str(&serialized).unwrap();

        assert_eq!(registration.registration_id, deserialized.registration_id);
        assert_eq!(registration.course_id, deserialized.course_id);
        assert_eq!(registration.user_id, deserialized.user_id);
        assert_eq!(registration.progress, deserialized.progress);
        assert_eq!(registration.score, deserialized.score);
        assert_eq!(registration.passed, deserialized.passed);

        // Check nested structures
        if let (Some(orig_course), Some(deser_course)) =
            (&registration.course_info, &deserialized.course_info)
        {
            assert_eq!(orig_course.course_id, deser_course.course_id);
            assert_eq!(orig_course.course_name, deser_course.course_name);
        }

        if let (Some(orig_user), Some(deser_user)) =
            (&registration.user_info, &deserialized.user_info)
        {
            assert_eq!(orig_user.user_id, deser_user.user_id);
            assert_eq!(orig_user.name, deser_user.name);
            assert_eq!(orig_user.email, deser_user.email);
        }
    }

    #[test]
    fn test_course_info_serialization() {
        let course = CourseInfo {
            course_id: "course_advanced_react".to_string(),
            course_name: Some("React高级开发".to_string()),
            description: Some("深入学习React框架的高级特性和最佳实践".to_string()),
            course_type: Some("advanced".to_string()),
            category: Some("前端开发".to_string()),
            tags: Some(vec![
                "react".to_string(),
                "frontend".to_string(),
                "advanced".to_string(),
                "hooks".to_string(),
            ]),
            duration: Some(14400),
            difficulty: Some("advanced".to_string()),
            cover_url: Some("https://example.com/react-course.jpg".to_string()),
            creator: Some("expert_teacher_001".to_string()),
            status: Some("published".to_string()),
        };

        let serialized = serde_json::to_string(&course).unwrap();
        let deserialized: CourseInfo = serde_json::from_str(&serialized).unwrap();

        assert_eq!(course.course_id, deserialized.course_id);
        assert_eq!(course.course_name, deserialized.course_name);
        assert_eq!(course.description, deserialized.description);
        assert_eq!(course.course_type, deserialized.course_type);
        assert_eq!(course.category, deserialized.category);
        assert_eq!(course.tags, deserialized.tags);
        assert_eq!(course.duration, deserialized.duration);
        assert_eq!(course.difficulty, deserialized.difficulty);
        assert_eq!(course.cover_url, deserialized.cover_url);
        assert_eq!(course.creator, deserialized.creator);
        assert_eq!(course.status, deserialized.status);
    }

    #[test]
    fn test_course_info_minimal_data() {
        let course = CourseInfo {
            course_id: "minimal_course".to_string(),
            course_name: None,
            description: None,
            course_type: None,
            category: None,
            tags: None,
            duration: None,
            difficulty: None,
            cover_url: None,
            creator: None,
            status: None,
        };

        let serialized = serde_json::to_string(&course).unwrap();
        assert_eq!(serialized, "{\"course_id\":\"minimal_course\"}");

        let deserialized: CourseInfo = serde_json::from_str(&serialized).unwrap();
        assert_eq!(course.course_id, deserialized.course_id);
        assert!(deserialized.course_name.is_none());
        assert!(deserialized.description.is_none());
        assert!(deserialized.tags.is_none());
    }

    #[test]
    fn test_user_info_serialization() {
        let user = UserInfo {
            user_id: "employee_12345".to_string(),
            name: Some("李四".to_string()),
            email: Some("lisi@company.com".to_string()),
            avatar_url: Some("https://avatar.company.com/lisi.jpg".to_string()),
            department: Some("产品部".to_string()),
            position: Some("产品经理".to_string()),
        };

        let serialized = serde_json::to_string(&user).unwrap();
        let deserialized: UserInfo = serde_json::from_str(&serialized).unwrap();

        assert_eq!(user.user_id, deserialized.user_id);
        assert_eq!(user.name, deserialized.name);
        assert_eq!(user.email, deserialized.email);
        assert_eq!(user.avatar_url, deserialized.avatar_url);
        assert_eq!(user.department, deserialized.department);
        assert_eq!(user.position, deserialized.position);
    }

    #[test]
    fn test_learning_record_serialization() {
        let record = LearningRecord {
            record_id: "lr_98765".to_string(),
            chapter_id: Some("chapter_advanced_hooks".to_string()),
            chapter_name: Some("React Hooks高级用法".to_string()),
            progress: Some(87.5),
            duration: Some(3600),
            completed: Some(false),
            start_time: Some(1640995200),
            end_time: Some(1640998800),
            attempt_count: Some(3),
        };

        let serialized = serde_json::to_string(&record).unwrap();
        let deserialized: LearningRecord = serde_json::from_str(&serialized).unwrap();

        assert_eq!(record.record_id, deserialized.record_id);
        assert_eq!(record.chapter_id, deserialized.chapter_id);
        assert_eq!(record.chapter_name, deserialized.chapter_name);
        assert_eq!(record.progress, deserialized.progress);
        assert_eq!(record.duration, deserialized.duration);
        assert_eq!(record.completed, deserialized.completed);
        assert_eq!(record.start_time, deserialized.start_time);
        assert_eq!(record.end_time, deserialized.end_time);
        assert_eq!(record.attempt_count, deserialized.attempt_count);
    }

    #[test]
    fn test_learning_statistics_serialization() {
        let stats = LearningStatistics {
            total_courses: Some(50),
            completed_courses: Some(32),
            in_progress_courses: Some(8),
            total_study_time: Some(144000),
            average_progress: Some(78.5),
            average_score: Some(86.2),
            pass_rate: Some(0.94),
        };

        let serialized = serde_json::to_string(&stats).unwrap();
        let deserialized: LearningStatistics = serde_json::from_str(&serialized).unwrap();

        assert_eq!(stats.total_courses, deserialized.total_courses);
        assert_eq!(stats.completed_courses, deserialized.completed_courses);
        assert_eq!(stats.in_progress_courses, deserialized.in_progress_courses);
        assert_eq!(stats.total_study_time, deserialized.total_study_time);
        assert_eq!(stats.average_progress, deserialized.average_progress);
        assert_eq!(stats.average_score, deserialized.average_score);
        assert_eq!(stats.pass_rate, deserialized.pass_rate);
    }

    #[test]
    fn test_learning_statistics_with_none_values() {
        let stats = LearningStatistics {
            total_courses: Some(10),
            completed_courses: None,
            in_progress_courses: None,
            total_study_time: None,
            average_progress: None,
            average_score: None,
            pass_rate: None,
        };

        let serialized = serde_json::to_string(&stats).unwrap();
        assert!(!serialized.contains("completed_courses"));
        assert!(!serialized.contains("average_progress"));
        assert!(!serialized.contains("pass_rate"));

        let deserialized: LearningStatistics = serde_json::from_str(&serialized).unwrap();
        assert_eq!(stats.total_courses, deserialized.total_courses);
        assert!(deserialized.completed_courses.is_none());
        assert!(deserialized.average_progress.is_none());
    }

    #[test]
    fn test_course_registration_event_serialization() {
        let event = CourseRegistrationEvent {
            event_type: "registration_updated".to_string(),
            registration: CourseRegistration {
                registration_id: "reg_event_test".to_string(),
                course_id: "course_event_test".to_string(),
                user_id: "user_event_test".to_string(),
                registration_type: Some("optional".to_string()),
                status: Some("completed".to_string()),
                progress: Some(100.0),
                start_time: Some(1640995200),
                completion_time: Some(1641081600),
                total_duration: Some(7200),
                studied_duration: Some(7200),
                score: Some(92.5),
                passed: Some(true),
                course_info: None,
                user_info: None,
                learning_records: None,
                created_at: Some(1640995200),
                updated_at: Some(1641081600),
            },
            old_registration: Some(CourseRegistration {
                registration_id: "reg_event_test".to_string(),
                course_id: "course_event_test".to_string(),
                user_id: "user_event_test".to_string(),
                registration_type: Some("optional".to_string()),
                status: Some("in_progress".to_string()),
                progress: Some(85.0),
                start_time: Some(1640995200),
                completion_time: None,
                total_duration: Some(7200),
                studied_duration: Some(6120),
                score: Some(88.0),
                passed: Some(false),
                course_info: None,
                user_info: None,
                learning_records: None,
                created_at: Some(1640995200),
                updated_at: Some(1641000000),
            }),
            timestamp: 1641081600,
            source: Some("learning_platform".to_string()),
        };

        let serialized = serde_json::to_string(&event).unwrap();
        let deserialized: CourseRegistrationEvent = serde_json::from_str(&serialized).unwrap();

        assert_eq!(event.event_type, deserialized.event_type);
        assert_eq!(
            event.registration.registration_id,
            deserialized.registration.registration_id
        );
        assert_eq!(event.registration.status, deserialized.registration.status);
        assert_eq!(
            event.registration.progress,
            deserialized.registration.progress
        );
        assert_eq!(event.timestamp, deserialized.timestamp);
        assert_eq!(event.source, deserialized.source);

        // Check old_registration
        if let (Some(orig_old), Some(deser_old)) =
            (&event.old_registration, &deserialized.old_registration)
        {
            assert_eq!(orig_old.status, deser_old.status);
            assert_eq!(orig_old.progress, deser_old.progress);
            assert_eq!(orig_old.passed, deser_old.passed);
        }
    }

    #[test]
    fn test_course_registration_event_without_old_registration() {
        let event = CourseRegistrationEvent {
            event_type: "registration_created".to_string(),
            registration: CourseRegistration {
                registration_id: "reg_new".to_string(),
                course_id: "course_new".to_string(),
                user_id: "user_new".to_string(),
                registration_type: Some("required".to_string()),
                status: Some("not_started".to_string()),
                progress: Some(0.0),
                start_time: None,
                completion_time: None,
                total_duration: Some(3600),
                studied_duration: Some(0),
                score: None,
                passed: None,
                course_info: None,
                user_info: None,
                learning_records: None,
                created_at: Some(1641168000),
                updated_at: Some(1641168000),
            },
            old_registration: None,
            timestamp: 1641168000,
            source: None,
        };

        let serialized = serde_json::to_string(&event).unwrap();
        assert!(!serialized.contains("old_registration"));
        assert!(!serialized.contains("source"));

        let deserialized: CourseRegistrationEvent = serde_json::from_str(&serialized).unwrap();
        assert_eq!(event.event_type, deserialized.event_type);
        assert_eq!(
            event.registration.registration_id,
            deserialized.registration.registration_id
        );
        assert_eq!(event.timestamp, deserialized.timestamp);
        assert!(deserialized.old_registration.is_none());
        assert!(deserialized.source.is_none());
    }

    #[test]
    fn test_complex_course_registration_with_multiple_records() {
        let complex_registration = CourseRegistration {
            registration_id: "complex_reg_001".to_string(),
            course_id: "full_stack_bootcamp".to_string(),
            user_id: "student_advanced".to_string(),
            registration_type: Some("premium".to_string()),
            status: Some("in_progress".to_string()),
            progress: Some(42.5),
            start_time: Some(1640995200),
            completion_time: None,
            total_duration: Some(180000),
            studied_duration: Some(76500),
            score: Some(89.2),
            passed: Some(false),
            course_info: Some(CourseInfo {
                course_id: "full_stack_bootcamp".to_string(),
                course_name: Some("全栈开发训练营".to_string()),
                description: Some("全面学习前端、后端、数据库等全栈开发技能".to_string()),
                course_type: Some("bootcamp".to_string()),
                category: Some("综合开发".to_string()),
                tags: Some(vec![
                    "fullstack".to_string(),
                    "javascript".to_string(),
                    "python".to_string(),
                    "database".to_string(),
                    "devops".to_string(),
                ]),
                duration: Some(180000),
                difficulty: Some("intermediate".to_string()),
                cover_url: Some("https://bootcamp.example.com/cover.jpg".to_string()),
                creator: Some("bootcamp_instructor".to_string()),
                status: Some("active".to_string()),
            }),
            user_info: Some(UserInfo {
                user_id: "student_advanced".to_string(),
                name: Some("王五".to_string()),
                email: Some("wangwu@student.edu".to_string()),
                avatar_url: Some("https://student.edu/avatars/wangwu.jpg".to_string()),
                department: Some("计算机学院".to_string()),
                position: Some("研究生".to_string()),
            }),
            learning_records: Some(vec![
                LearningRecord {
                    record_id: "lr_frontend_001".to_string(),
                    chapter_id: Some("ch_html_css".to_string()),
                    chapter_name: Some("HTML与CSS基础".to_string()),
                    progress: Some(100.0),
                    duration: Some(7200),
                    completed: Some(true),
                    start_time: Some(1640995200),
                    end_time: Some(1641002400),
                    attempt_count: Some(1),
                },
                LearningRecord {
                    record_id: "lr_frontend_002".to_string(),
                    chapter_id: Some("ch_javascript".to_string()),
                    chapter_name: Some("JavaScript进阶".to_string()),
                    progress: Some(75.0),
                    duration: Some(14400),
                    completed: Some(false),
                    start_time: Some(1641088800),
                    end_time: None,
                    attempt_count: Some(2),
                },
                LearningRecord {
                    record_id: "lr_backend_001".to_string(),
                    chapter_id: Some("ch_python".to_string()),
                    chapter_name: Some("Python Web开发".to_string()),
                    progress: Some(0.0),
                    duration: Some(0),
                    completed: Some(false),
                    start_time: None,
                    end_time: None,
                    attempt_count: Some(0),
                },
            ]),
            created_at: Some(1640995200),
            updated_at: Some(1641254400),
        };

        let serialized = serde_json::to_string(&complex_registration).unwrap();
        let deserialized: CourseRegistration = serde_json::from_str(&serialized).unwrap();

        assert_eq!(
            complex_registration.registration_id,
            deserialized.registration_id
        );
        assert_eq!(complex_registration.progress, deserialized.progress);
        assert_eq!(complex_registration.score, deserialized.score);

        // Test learning records array
        if let (Some(orig_records), Some(deser_records)) = (
            &complex_registration.learning_records,
            &deserialized.learning_records,
        ) {
            assert_eq!(orig_records.len(), deser_records.len());
            assert_eq!(orig_records.len(), 3);

            // Test first record (completed)
            assert_eq!(orig_records[0].chapter_name, deser_records[0].chapter_name);
            assert_eq!(orig_records[0].progress, deser_records[0].progress);
            assert_eq!(orig_records[0].completed, deser_records[0].completed);

            // Test second record (in progress)
            assert_eq!(orig_records[1].chapter_name, deser_records[1].chapter_name);
            assert_eq!(orig_records[1].progress, deser_records[1].progress);
            assert_eq!(orig_records[1].completed, deser_records[1].completed);

            // Test third record (not started)
            assert_eq!(orig_records[2].chapter_name, deser_records[2].chapter_name);
            assert_eq!(orig_records[2].progress, deser_records[2].progress);
            assert_eq!(orig_records[2].start_time, deser_records[2].start_time);
        }

        // Test course info
        if let (Some(orig_course), Some(deser_course)) =
            (&complex_registration.course_info, &deserialized.course_info)
        {
            assert_eq!(orig_course.tags, deser_course.tags);
            if let (Some(orig_tags), Some(deser_tags)) = (&orig_course.tags, &deser_course.tags) {
                assert_eq!(orig_tags.len(), 5);
                assert_eq!(orig_tags[0], "fullstack");
                assert_eq!(orig_tags, deser_tags);
            }
        }
    }

    #[test]
    fn test_debug_trait_for_models() {
        let course = CourseInfo {
            course_id: "debug_test".to_string(),
            course_name: Some("Debug Test Course".to_string()),
            description: None,
            course_type: None,
            category: None,
            tags: None,
            duration: None,
            difficulty: None,
            cover_url: None,
            creator: None,
            status: None,
        };

        let debug_string = format!("{:?}", course);
        assert!(debug_string.contains("CourseInfo"));
        assert!(debug_string.contains("debug_test"));
        assert!(debug_string.contains("Debug Test Course"));
    }
}
