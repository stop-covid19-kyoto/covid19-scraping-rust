use calamine::DataType;
use chrono::{DateTime, Local, Utc};
use crate::{SumData, Summary};

use calamine::{Range};

pub fn inspections_summary_generate(range: Range<DataType>, last_update: DateTime<Local>) -> Summary {

    let mut inspections_summary: Summary = Summary {
        data: Vec::new(),
        last_update: last_update
    };

    let mut last_sum: i64 = 0;

    for row in range.rows().rev() {
        let sum: i64 = row[1].get_float().unwrap() as i64;

        inspections_summary.data.push(SumData {
            date: DateTime::from_utc(row[0].as_datetime().unwrap(), Utc) as DateTime<Utc>,
            sum: sum - last_sum
        });

        last_sum = sum;
    }

    return inspections_summary;

}
