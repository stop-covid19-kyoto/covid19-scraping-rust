use crate::structs::json::jsonize_patient::JsonizePatient;
use serde::ser::{Serialize, Serializer, SerializeStruct};

pub struct Patients {
    pub data: Vec<JsonizePatient>,
    pub last_update: String
}

impl Serialize for Patients {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Patients", 6)?;
        state.serialize_field("data", &self.data)?;
        state.serialize_field("last_update", &self.last_update)?;
        state.end()
    }
}
