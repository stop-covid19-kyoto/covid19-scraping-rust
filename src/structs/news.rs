use serde::ser::{Serialize, Serializer, SerializeStruct};

pub struct News {
    pub news_items: Vec<NewsItem>
}

impl Serialize for News {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("News", 6)?;
        state.serialize_field("newsItems", &self.news_items)?;
        state.end()
    }
}

pub struct NewsItem {
    pub date: String,
    pub text: String,
    pub url: String
}

impl Serialize for NewsItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("NewsItem", 6)?;
        state.serialize_field("date", &self.date)?;
        state.serialize_field("text", &self.text)?;
        state.serialize_field("url", &self.url)?;
        state.end()
    }
}
