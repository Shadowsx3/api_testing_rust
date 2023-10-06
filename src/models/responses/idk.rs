use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub data: Vec<Daum>,
    pub source: Vec<Source>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Daum {
    #[serde(rename = "ID Nation")]
    pub id_nation: String,
    #[serde(rename = "Nation")]
    pub nation: String,
    #[serde(rename = "ID Year")]
    pub id_year: i64,
    #[serde(rename = "Year")]
    pub year: String,
    #[serde(rename = "Population")]
    pub population: i64,
    #[serde(rename = "Slug Nation")]
    pub slug_nation: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    pub measures: Vec<String>,
    pub annotations: Annotations,
    pub name: String,
    pub substitutions: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Annotations {
    #[serde(rename = "source_name")]
    pub source_name: String,
    #[serde(rename = "source_description")]
    pub source_description: String,
    #[serde(rename = "dataset_name")]
    pub dataset_name: String,
    #[serde(rename = "dataset_link")]
    pub dataset_link: String,
    #[serde(rename = "table_id")]
    pub table_id: String,
    pub topic: String,
    pub subtopic: String,
}
