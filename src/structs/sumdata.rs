use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct SumData {
    pub date: DateTime<Utc>,
    pub sum: i64
}
