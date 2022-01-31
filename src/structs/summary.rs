use chrono::{DateTime, Local};
use crate::structs::sumdata::SumData;

#[derive(Clone)]
pub struct Summary {
    pub data: Vec<SumData>,
    pub last_update: DateTime<Local>
}
