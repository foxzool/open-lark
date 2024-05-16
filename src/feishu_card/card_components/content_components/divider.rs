use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FeishuCardDivider {
    tag: String
}

impl Default for FeishuCardDivider {
    fn default() -> Self {
        FeishuCardDivider {
            tag: "hr".to_string()
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        use serde_json::json;
        use crate::feishu_card::card_components::content_components::divider::*;

        let divider = FeishuCardDivider::default();
        let json = json!({
            "tag": "hr"
        });

        assert_eq!(serde_json::to_string(&divider).unwrap(), json.to_string());
    }
}