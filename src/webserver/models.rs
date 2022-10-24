use ::chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[readonly::make]
pub struct Alert {
    pub ticker: String,
    pub signal: String,
    pub timestamp: DateTime<Utc>,
    pub category: String,
    pub interval: String,
}
