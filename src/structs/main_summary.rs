use chrono::{DateTime, Local};
use serde::ser::{Serialize, Serializer, SerializeStruct};

#[derive(Clone)]
pub struct Attribute {
    pub attr: String,
    pub value: i64
}

#[derive(Clone)]
pub struct MainSummaryChildren {
    pub attr: String,
    pub value: i64,
    pub children: Vec<Attribute>
}

#[derive(Clone)]
pub struct MainSummary {
    pub attr: String,
    pub value: i64,
    pub children: Vec<MainSummaryChildren>,
    pub last_update: DateTime<Local>
}

impl Serialize for Attribute {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("JsonizeSummary", 6)?;
        state.serialize_field("attr", &self.attr)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

impl Serialize for MainSummaryChildren {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("MainSummaryChildren", 6)?;
        state.serialize_field("attr", &self.attr)?;
        state.serialize_field("value", &self.value)?;
        state.serialize_field("children", &self.children)?;
        state.end()
    }
}
