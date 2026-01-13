//! OpenLark CardKit 服务端点定义

/// CardKit 卡片实体 v1
pub const CARDKIT_V1_CARDS: &str = "/open-apis/cardkit/v1/cards";
pub const CARDKIT_V1_CARD_ID_CONVERT: &str = "/open-apis/cardkit/v1/cards/id_convert";

/// CardKit 卡片组件 v1
pub fn cardkit_v1_card(card_id: &str) -> String {
    format!("{CARDKIT_V1_CARDS}/{card_id}")
}

pub fn cardkit_v1_card_settings(card_id: &str) -> String {
    format!("{CARDKIT_V1_CARDS}/{card_id}/settings")
}

pub fn cardkit_v1_card_batch_update(card_id: &str) -> String {
    format!("{CARDKIT_V1_CARDS}/{card_id}/batch_update")
}

pub fn cardkit_v1_card_elements(card_id: &str) -> String {
    format!("{CARDKIT_V1_CARDS}/{card_id}/elements")
}

pub fn cardkit_v1_card_element(card_id: &str, element_id: &str) -> String {
    format!("{CARDKIT_V1_CARDS}/{card_id}/elements/{element_id}")
}

pub fn cardkit_v1_card_element_content(card_id: &str, element_id: &str) -> String {
    format!("{CARDKIT_V1_CARDS}/{card_id}/elements/{element_id}/content")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_endpoints_non_empty() {
        assert!(!CARDKIT_V1_CARDS.is_empty());
        assert!(!CARDKIT_V1_CARD_ID_CONVERT.is_empty());
        assert!(!cardkit_v1_card("c").is_empty());
        assert!(!cardkit_v1_card_settings("c").is_empty());
        assert!(!cardkit_v1_card_batch_update("c").is_empty());
        assert!(!cardkit_v1_card_elements("c").is_empty());
        assert!(!cardkit_v1_card_element("c", "e").is_empty());
        assert!(!cardkit_v1_card_element_content("c", "e").is_empty());
    }
}
