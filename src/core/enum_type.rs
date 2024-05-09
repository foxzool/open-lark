#[derive(Default, Hash, Eq, PartialEq, Debug, Clone)]
pub enum AppType {
    #[default]
    SELF,
    ISV,
}

#[derive(Default, Hash, Eq, PartialEq, Debug, Clone)]
pub enum AccessTokenType {
    #[default]
    TENANT,
    APP,
    USER,
}
