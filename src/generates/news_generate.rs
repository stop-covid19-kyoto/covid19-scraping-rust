use calamine::{Range, DataType};
use crate::structs::news::{News, NewsItem};

pub fn news_generate(range: &Range<DataType>) -> News {

    let mut news_items: Vec<NewsItem> = Vec::new();

    for row in range.rows() {
        news_items.push(NewsItem {
            date: row[0].as_datetime().unwrap().format("%Y/%m/%d").to_string(),
            text: row[1].get_string().unwrap().to_string(),
            url: row[2].get_string().unwrap().to_string()
        });
    }

    return News {
        news_items: news_items
    };
}
