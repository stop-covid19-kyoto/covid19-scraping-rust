use serde::ser::{Serialize, Serializer, SerializeStruct};

pub struct LastUpdate {
    pub last_update: String
}

impl Serialize for LastUpdate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("LastUpdate", 6)?;
        state.serialize_field("last_update", &self.last_update)?;
        state.end()
    }
}
