use serde::ser::{Serialize, Serializer, SerializeStruct};

// JSONに出力することを目的とした構造体なので、命名規則の警告を出力させないほうが適切
#[allow(non_snake_case)]
pub struct JsonizePatient {
    pub number: i32,
    pub release_date: Option<String>,
    pub place: String,
    pub age_and_gender: String,
    pub leave: Option<String>,
    pub date: String,
}

impl Serialize for JsonizePatient {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("JsonizePatient", 6)?;
        state.serialize_field("No", &self.number)?;
        state.serialize_field("リリース日", &self.release_date)?;
        state.serialize_field("居住地", &self.place)?;
        state.serialize_field("年代と性別", &self.age_and_gender)?;
        state.serialize_field("退院", &self.leave)?;
        state.serialize_field("date", &self.date)?;
        state.end()
    }
}
