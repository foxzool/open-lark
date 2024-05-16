use serde::{Deserialize, Serialize};

use crate::feishu_card::icon::FeishuCardTextIcon;

#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuCardUserList {
    tag: String,
    lines: Option<i32>,
    show_name: Option<bool>,
    show_avatar: Option<bool>,
    size: Option<String>,
    persons: Vec<FeishuCardUserId>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuCardUserId {
    id: String,
}

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

    use crate::feishu_card::card_components::content_components::user_list::{
        FeishuCardUserId, FeishuCardUserListBuilder,
    };
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
