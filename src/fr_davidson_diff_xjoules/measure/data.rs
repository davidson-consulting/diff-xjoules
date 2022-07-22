use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub indicator: String,
    pub value: f64,
}
