use crate::utils::date_format::convert_datetime_to_date_and_time;
use chrono::{DateTime, Local, SecondsFormat};
use crate::structs::{summary::Summary, json::{jsonize_sumdata::JsonizeSumData, jsonize_summary::JsonizeSummary}};

pub fn jsonize_summary_generate(summary: Summary, last_update: DateTime<Local>) -> String {

    let mut jsonize_sumdatas: Vec<JsonizeSumData> = Vec::new();

    for data in summary.data {
        jsonize_sumdatas.push(JsonizeSumData {
            date: data.date.to_rfc3339_opts(SecondsFormat::Millis, true),
            sum: data.sum
        });
    }

    let jsonize_summary: JsonizeSummary = JsonizeSummary {
        data: jsonize_sumdatas,
        last_update: convert_datetime_to_date_and_time(last_update)
    };

    return serde_json::to_string_pretty(&jsonize_summary).unwrap();

}
