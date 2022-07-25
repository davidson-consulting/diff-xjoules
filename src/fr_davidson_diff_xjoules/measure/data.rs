use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub indicator: String,
    pub value: f64,
}

impl Data {
    pub fn new(indicator: &str, value: f64) -> Data {
        return Data {
            indicator: String::from(indicator),
            value,
        };
    }
}
