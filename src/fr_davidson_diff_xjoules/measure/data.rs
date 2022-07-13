use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub indicator: String,
    pub value: f64,
}