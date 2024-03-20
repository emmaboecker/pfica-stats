use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseGeneratorRun {
    pub runner_id: String,
    pub submit_type: GeneratorSubmitType,
    pub values: GeneratorValues,
    pub run_at: chrono::DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum GeneratorSubmitType {
    Submit,
    Debounced,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratorValues {
    pub age: i32,
    pub income: i32,
    pub ljsn_member: bool,
    pub employment: GeneratorEmploymentValues,
    pub soli: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum GeneratorEmploymentValues {
    School,
    Student,
    JobSchool,
    Employed,
    Unemployed,
}
