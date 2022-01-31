use crate::utils::date_format::IncorrectFormatError;
use chrono::{DateTime, Duration, Local, Utc};
use crate::structs::{summary::Summary, sumdata::SumData};

pub fn patients_summary_generate(datetimes: Vec<DateTime<Utc>>, min: DateTime<Utc>, max: DateTime<Utc>, last_update: DateTime<Local>) -> Result<Summary, IncorrectFormatError> {

    if !(min > max) {

        let mut min: DateTime<Utc> = min.clone();

        let mut datetimes: Vec<DateTime<Utc>> = datetimes.clone();
        datetimes.sort();

        let mut datetimes_index: usize = 0;
        let mut summary_index: usize = 0;

        let mut summary: Summary = Summary {
            data: Vec::new(),
            last_update: last_update
        };
    
        while max >= min {

            summary.data.push(SumData {
                date: min,
                sum: 0
            });

            while datetimes.len() > datetimes_index {
                if datetimes[datetimes_index] == summary.data[summary_index].date {
                    summary.data[summary_index].sum += 1;
                    datetimes_index += 1;
                } else {
                    break;
                }
            }

            summary_index += 1;
            min = min + Duration::days(1);
        }

        return Ok(summary);

    }

    return Err(IncorrectFormatError {});

}
