use serde::ser::{Serialize, Serializer, SerializeStruct};
use crate::structs::main_summary::MainSummaryChildren;

pub struct JsonizeMainSummary {
    pub attr: String,
    pub value: i64,
    pub children: Vec<MainSummaryChildren>,
    pub last_update: String
}

impl Serialize for JsonizeMainSummary {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("JsonizeMainSummary", 6)?;
        state.serialize_field("attr", &self.attr)?;
        state.serialize_field("value", &self.value)?;
        state.serialize_field("children", &self.children)?;
        state.serialize_field("last_update", &self.last_update)?;
        state.end()
    }
}
