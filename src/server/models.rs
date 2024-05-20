use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetJsonParam {
    pub simple: Option<bool>,
    pub force_parse: Option<bool>,
    pub sort: Option<bool>,
    pub cache: Option<u32>,
}
