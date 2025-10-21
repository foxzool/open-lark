use serde::{Deserialize, Serialize};

use crate::card::icon::FeishuCardTextIcon;

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
/// - 人员的 Open ID：标识一个用户在某个应用中的身份。同一个用户在不同应用中的 Open ID
///   不同。详情参考如何获取 Open ID
/// - 人员的 Union ID：标识一个用户在某个应用开发商下的身份。同一用户在同一开发商下的应用中的 Union
///   ID 是相同的，在不同开发商下的应用中的 Union ID 是不同的。通过 Union
///   ID，应用开发商可以把同个用户在多个应用中的身份关联起来。详情参考如何获取 Union ID
/// - 人员的 User ID ：标识一个用户在某个租户内的身份。同一个用户在租户 A 和租户 B 内的 User ID
///   是不同的。在同一个租户内，一个用户的 User ID 在所有应用（包括商店应用）中都保持一致。User ID
///   主要用于在不同的应用间打通用户数据。详情参考如何获取User ID
#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuCardUserId {
    id: String,
}

impl FeishuCardUserList {
    pub fn new() -> Self {
        FeishuCardUserList::default()
    }

    pub fn lines(mut self, lines: i32) -> Self {
        self.lines = Some(lines);
        self
    }

    pub fn show_name(mut self, show_name: bool) -> Self {
        self.show_name = Some(show_name);
        self
    }

    pub fn show_avatar(mut self, show_avatar: bool) -> Self {
        self.show_avatar = Some(show_avatar);
        self
    }

    pub fn size(mut self, size: &str) -> Self {
        self.size = Some(size.to_string());
        self
    }

    pub fn persons(mut self, persons: Vec<&str>) -> Self {
        self.persons = persons
            .iter()
            .map(|id| FeishuCardUserId { id: id.to_string() })
            .collect();
        self
    }

    pub fn icon(mut self, icon: FeishuCardTextIcon) -> Self {
        self.icon = Some(icon);
        self
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::card::{
        components::content_components::user_list::FeishuCardUserList, icon::FeishuCardTextIcon,
    };

    #[test]
    fn test_user_list() {
        let user_list = FeishuCardUserList::new()
            .lines(1)
            .show_name(true)
            .show_avatar(true)
            .size("small")
            .persons(vec!["user_id"])
            .icon(
                FeishuCardTextIcon::new()
                    .tag("standard_icon")
                    .token("token")
                    .color("red"),
            );
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

        let user_list = FeishuCardUserList::new()
            .lines(1)
            .show_name(true)
            .show_avatar(true)
            .size("large")
            .persons(vec![
                "ou_0fdb0e7663af7128e7d9f8adeb2b689e",
                "ou_47a09ae5a1353f3276924161dc63a2be",
            ])
            .icon(
                FeishuCardTextIcon::new()
                    .token("chat-forbidden_outlined")
                    .color("orange")
                    .img_key("img_v2_38811724"),
            );

        let json = json!({
          "tag": "person_list",
          "lines": 1, // 最大显示行数，默认关闭不限制最大显示行数。
          "show_name": true, // 是否展示人员对应的用户名。
          "show_avatar": true, // 是否展示人员对应的头像。
          "size": "large", // 人员头像的尺寸。
          "persons": [
            // 人员列表。人员的 ID 支持 open_id , user_id, union_id
            { "id": "ou_0fdb0e7663af7128e7d9f8adeb2b689e" },
            { "id": "ou_47a09ae5a1353f3276924161dc63a2be" }
          ],
          "icon": {
            // 前缀图标。
            "tag": "standard_icon", // 图标类型。
            "token": "chat-forbidden_outlined", // 图标的 token。仅在 tag 为 standard_icon 时生效。
            "color": "orange", // 图标颜色。仅在 tag 为 standard_icon 时生效。
            "img_key": "img_v2_38811724" // 图片的 key。仅在 tag 为 custom_icon 时生效。
          }
        });

        assert_eq!(serde_json::to_value(&user_list).unwrap(), json)
    }
}
