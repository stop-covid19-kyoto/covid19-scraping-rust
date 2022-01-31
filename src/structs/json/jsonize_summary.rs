use crate::structs::json::jsonize_sumdata::JsonizeSumData;
use serde::ser::{Serialize, Serializer, SerializeStruct};

// JSONに出力することを目的とした構造体なので、命名規則の警告を出力させないほうが適切
#[allow(non_snake_case)]
pub struct JsonizeSummary {
    pub data: Vec<JsonizeSumData>,
    pub last_update: String
}

impl Serialize for JsonizeSummary {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("JsonizeSummary", 6)?;
        state.serialize_field("data", &self.data)?;
        state.serialize_field("last_update", &self.last_update)?;
        state.end()
    }
}
