use crate::structs::main_summary::{Attribute, MainSummary, MainSummaryChildren};
use calamine::{DataType, Range};
use chrono::{DateTime, Local};

pub fn main_summary_generate(range: Range<DataType>, last_update: DateTime<Local>) -> MainSummary {

    return MainSummary {
        attr: "検査実施人数".to_string(),
        value: get_sum(&range, (0, 1)).unwrap(),
        children: [
            MainSummaryChildren {
                attr: "陽性患者数".to_string(),
                value: get_sum(&range, (0, 2)).unwrap(),
                children: [
                    Attribute {
                        attr: "入院中・入院調整中".to_string(),
                        value: get_sum(&range, (0, 4)).unwrap()
                    },
                    Attribute {
                        attr: "高度重症病床".to_string(),
                        value: get_sum(&range, (0, 5)).unwrap()
                    },
                    Attribute {
                        attr: "その他".to_string(),
                        value: get_sum(&range, (0, 6)).unwrap()
                    },
                    Attribute {
                        attr: "宿泊施設".to_string(),
                        value: get_sum(&range, (0, 7)).unwrap()
                    },
                    Attribute {
                        attr: "自宅療養".to_string(),
                        value: get_sum(&range, (0, 8)).unwrap()
                    },
                    Attribute {
                        attr: "死亡".to_string(),
                        value: get_sum(&range, (0, 9)).unwrap()
                    },
                    Attribute {
                        attr: "退院".to_string(),
                        value: get_sum(&range, (0, 3)).unwrap()
                    },
                    Attribute {
                        attr: "調整中".to_string(),
                        value: get_sum(&range, (0, 10)).unwrap()
                    },
                ].to_vec()
            }
        ].to_vec(),
        last_update: last_update
    }

}

fn get_sum(range: &Range<DataType>, relative_position: (usize, usize)) -> Option<i64> {

    let cell: Option<&DataType> = range.get(relative_position);

    match cell {
        Some(cell) => {
            if cell.is_float() {
                Some(cell.get_float().unwrap() as i64)
            } else {
                Some(get_sum(range, (relative_position.0 + 1, relative_position.1)).unwrap())
            }
        },
        None => None
    }

}
