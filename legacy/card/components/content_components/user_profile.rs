use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuCardUserProfile {
    /// 组件的标签，人员组件的取值为 person。
    tag: String,
    /// 人员的头像尺寸。可取值：
    ///
    /// - extra_small：超小尺寸
    /// - small：小尺寸
    /// - medium：中尺寸
    /// - large：大尺寸
    size: Option<String>,
    /// 人员的 ID。可选值有：
    ///
    /// 人员的 Open ID：标识一个用户在某个应用中的身份。同一个用户在不同应用中的 Open ID
    /// 不同。详情参考如何获取 Open ID 人员的 Union ID：标识一个用户在某个应用开发商下的身份。
    /// 同一用户在同一开发商下的应用中的 Union ID 是相同的，在不同开发商下的应用中的 Union ID
    /// 是不同的。通过 Union
    /// ID，应用开发商可以把同个用户在多个应用中的身份关联起来。详情参考如何获取 Union ID
    /// 人员的 User ID ：标识一个用户在某个租户内的身份。同一个用户在租户 A 和租户 B 内的 User ID
    /// 是不同的。在同一个租户内，一个用户的 User ID 在所有应用（包括商店应用）中都保持一致。User
    /// ID 主要用于在不同的应用间打通用户数据。详情参考如何获取User ID
    user_id: String,
}

impl Default for FeishuCardUserProfile {
    fn default() -> Self {
        Self {
            tag: "person".to_string(),
            size: None,
            user_id: "".to_string(),
        }
    }
}

impl FeishuCardUserProfile {
    pub fn new() -> Self {
        FeishuCardUserProfile::default()
    }

    pub fn size(mut self, size: &str) -> Self {
        self.size = Some(size.to_string());
        self
    }

    pub fn user_id(mut self, user_id: &str) -> Self {
        self.user_id = user_id.to_string();
        self
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::card::components::content_components::user_profile::*;

    #[test]
    fn test() {
        let user_profile = FeishuCardUserProfile::new()
            .user_id("ou_449b53ad6aee526f7ed311b216aabcef")
            .size("medium");
        let json = json!( {
            "tag": "person",
            "size": "medium",
            "user_id": "ou_449b53ad6aee526f7ed311b216aabcef"
        });

        assert_eq!(serde_json::to_value(&user_profile).unwrap(), json);
    }
}
