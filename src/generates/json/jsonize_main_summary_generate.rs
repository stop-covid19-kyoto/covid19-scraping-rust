use crate::{MainSummary, structs::json::jsonize_main_summary::JsonizeMainSummary};
use crate::utils::date_format::convert_datetime_to_date_and_time;
use chrono::{DateTime, Local};

pub fn jsonize_main_summary_generate(main_summary: MainSummary, last_update: DateTime<Local>) -> String {

    return serde_json::to_string_pretty(&JsonizeMainSummary {
        attr: main_summary.attr,
        value: main_summary.value,
        children: main_summary.children,
        last_update: convert_datetime_to_date_and_time(last_update)
    }).unwrap();

}
