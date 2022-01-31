use serde::ser::{Serialize, Serializer, SerializeStruct};

// JSONに出力することを目的とした構造体なので、命名規則の警告を出力させないほうが適切
#[allow(non_snake_case)]
pub struct JsonizeSumData {
    pub date: String,
    pub sum: i64
}

impl Serialize for JsonizeSumData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("JsonizeSumData", 6)?;
        state.serialize_field("日付", &self.date)?;
        state.serialize_field("小計", &self.sum)?;
        state.end()
    }
}
