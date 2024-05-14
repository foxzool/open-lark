use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// 图标库 FIXME 还有很多没写
#[derive(Debug)]
pub enum FeishuCardIcon {
    Custom(String),
    ///  ![add_outlined](https://sf3-cn.feishucdn.com/obj/open-platform-opendoc/2d736a499f96fcab1fa40c0c2f507d15_pZTUyrLkNs.png?height=56&lazyload=true&width=56)
    AddOutlined,
    /// ![add-app_outlined](https://sf3-cn.feishucdn.com/obj/open-platform-opendoc/389d625f583d52732ba2b10ffa978a17_eqvwGBqfWP.png?height=56&lazyload=true&width=56)
    AddAppOutlined,
    /// ![add-bold_outlined](https://sf3-cn.feishucdn.com/obj/open-platform-opendoc/7caac0a8ab496b27f188b1f03f7dbcc6_VLNPXvQee5.png?height=56&lazyload=true&width=56)
    AddBoldOutlined,
    /// ![add-middle_outlined](https://sf3-cn.feishucdn.com/obj/open-platform-opendoc/9e97a069f504f559a2046ea20de0afd3_3UD11LpA5l.png?height=56&lazyload=true&width=56)
    AddMiddleOutlined,
    /// ![admin_outlined](https://sf3-cn.feishucdn.com/obj/open-platform-opendoc/1bae2ac9fa227546bb675a094ce27bf3_BZqKQ64PtK.png?height=56&lazyload=true&width=56)
    AdminOutlined,
    /// ![app-default_filled](https://sf3-cn.feishucdn.com/obj/open-platform-opendoc/0dc57b74878076f081eaff1f5da18ba4_r5uBCGviEs.png?height=56&lazyload=true&width=56)
    AppDefaultFilled,
}

impl Serialize for FeishuCardIcon {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            FeishuCardIcon::Custom(custom) => serializer.serialize_str(custom),
            FeishuCardIcon::AddOutlined => serializer.serialize_str("add_outlined"),
            FeishuCardIcon::AddAppOutlined => serializer.serialize_str("add-app_outlined"),
            FeishuCardIcon::AddBoldOutlined => serializer.serialize_str("add-bold_outlined"),
            FeishuCardIcon::AddMiddleOutlined => serializer.serialize_str("add-middle_outlined"),
            FeishuCardIcon::AdminOutlined => serializer.serialize_str("admin_outlined"),
            FeishuCardIcon::AppDefaultFilled => serializer.serialize_str("app-default_filled"),
        }
    }
}

impl<'de> Deserialize<'de> for FeishuCardIcon {
    fn deserialize<D>(deserializer: D) -> Result<FeishuCardIcon, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "add_outlined" => Ok(FeishuCardIcon::AddOutlined),
            "add-app_outlined" => Ok(FeishuCardIcon::AddAppOutlined),
            "add-bold_outlined" => Ok(FeishuCardIcon::AddBoldOutlined),
            "add-middle_outlined" => Ok(FeishuCardIcon::AddMiddleOutlined),
            "admin_outlined" => Ok(FeishuCardIcon::AdminOutlined),
            _ => Ok(FeishuCardIcon::Custom(s)),
        }
    }
}
