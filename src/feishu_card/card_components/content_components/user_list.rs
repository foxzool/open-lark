use serde::{Deserialize, Serialize};

use crate::feishu_card::icon::FeishuCardTextIcon;

/// 人员列表
#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuCardUserList {
    /// 组件的标签，人员列表组件的取值为 person_list。
    tag: String,
    /// 最大显示行数，默认不限制最大显示行数
    #[serde(skip_serializing_if = "Option::is_none")]
    lines: Option<i32>,
    /// 是否展示人员的用户名。
    #[serde(skip_serializing_if = "Option::is_none")]
    show_name: Option<bool>,
    /// 是否展示人员的头像。
    #[serde(skip_serializing_if = "Option::is_none")]
    show_avatar: Option<bool>,
    /// 人员的头像尺寸。可取值：
    ///
    /// - extra_small：超小尺寸
    /// - small：小尺寸
    /// - medium：中尺寸
    /// - large：大尺寸
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<String>,
    /// 人员列表。
    persons: Vec<FeishuCardUserId>,
    /// 添加图标作为文本前缀图标。支持自定义或使用图标库中的图标。
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<FeishuCardTextIcon>,
}

impl Default for FeishuCardUserList {
    fn default() -> Self {
        FeishuCardUserList {
            tag: "person_list".to_string(),
            lines: None,
            show_name: None,
            show_avatar: None,
            size: None,
            persons: vec![],
            icon: None,
        }
    }
}

/// 人员的 ID。可选值有：
/// - 人员的 Open ID：标识一个用户在某个应用中的身份。同一个用户在不同应用中的 Open ID 不同。详情参考如何获取 Open ID
/// - 人员的 Union ID：标识一个用户在某个应用开发商下的身份。同一用户在同一开发商下的应用中的 Union ID 是相同的，在不同开发商下的应用中的 Union ID 是不同的。通过 Union ID，应用开发商可以把同个用户在多个应用中的身份关联起来。详情参考如何获取 Union ID
/// - 人员的 User ID ：标识一个用户在某个租户内的身份。同一个用户在租户 A 和租户 B 内的 User ID 是不同的。在同一个租户内，一个用户的 User ID 在所有应用（包括商店应用）中都保持一致。User ID 主要用于在不同的应用间打通用户数据。详情参考如何获取User ID
#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuCardUserId {
    id: String,
}

/// 人员列表构建器
pub struct FeishuCardUserListBuilder {
    user_list: FeishuCardUserList,
}

impl FeishuCardUserListBuilder {
    pub fn new() -> Self {
        Self {
            user_list: FeishuCardUserList::default(),
        }
    }

    pub fn lines(mut self, lines: i32) -> Self {
        self.user_list.lines = Some(lines);
        self
    }

    pub fn show_name(mut self, show_name: bool) -> Self {
        self.user_list.show_name = Some(show_name);
        self
    }

    pub fn show_avatar(mut self, show_avatar: bool) -> Self {
        self.user_list.show_avatar = Some(show_avatar);
        self
    }

    pub fn size(mut self, size: &str) -> Self {
        self.user_list.size = Some(size.to_string());
        self
    }

    pub fn persons(mut self, persons: Vec<&str>) -> Self {
        self.user_list.persons = persons
            .iter()
            .map(|id| FeishuCardUserId { id: id.to_string() })
            .collect();
        self
    }

    pub fn icon(mut self, icon: FeishuCardTextIcon) -> Self {
        self.user_list.icon = Some(icon);
        self
    }

    pub fn build(self) -> FeishuCardUserList {
        self.user_list
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::feishu_card::card_components::content_components::user_list::FeishuCardUserListBuilder;
    use crate::feishu_card::icon::FeishuCardTextIconBuilder;

    #[test]
    fn test_user_list() {
        let user_list = FeishuCardUserListBuilder::new()
            .lines(1)
            .show_name(true)
            .show_avatar(true)
            .size("small")
            .persons(vec!["user_id"])
            .icon(
                FeishuCardTextIconBuilder::new()
                    .tag("standard_icon")
                    .token("token")
                    .color("red")
                    .build(),
            )
            .build();
        let json = json!({
            "tag": "person_list",
            "lines": 1,
            "show_name": true,
            "show_avatar": true,
            "size": "small",
            "persons": [{ "id": "user_id" }],
            "icon": {
                "tag": "standard_icon",
                "token": "token",
                "color": "red"
            }
        });
        assert_eq!(serde_json::to_value(&user_list).unwrap(), json);

        let user_list = FeishuCardUserListBuilder::new()
            .size("medium")
            .show_avatar(true)
            .show_name(true)
            .persons(vec![
                "ou_449b53ad6aee526f7ed311b216aabcef",
                "ou_449b53ad6aee526f7ed311b216aabcef",
            ])
            .build();
    }
}
