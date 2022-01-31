use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct Patient {
    pub number: i32,
    pub release_date: Option<DateTime<Utc>>,
    pub place: Option<String>,
    pub age: Option<String>,
    pub gender: Option<String>,
    pub leave: Option<String>
}
