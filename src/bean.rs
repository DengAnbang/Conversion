use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct XmlBean {
    #[serde(rename = "name")]
    pub key: String,
    #[serde(rename = "$value")]
    pub value: Option<String>,
}


#[derive(Debug, Deserialize)]
pub struct Resource {
    #[serde(rename = "string", default)]
    pub strings: Vec<XmlBean>,
}