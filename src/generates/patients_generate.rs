use calamine::DataType;
use crate::structs::patient::Patient;
use crate::utils::date_format::{convert_japanese_era_to_utc};

use calamine::{Range};

pub fn patients_generate(range: Range<DataType>) -> Vec<Patient> {

    let mut patients: Vec<Patient> = Vec::new();

    // 陽性者を全て取得
    for row in range.rows().rev() {
        // 通し番号を符号なし32bit整数になるようにパース
        let number: i32 = row[0].to_string().replace("例目", "").parse().unwrap();
        let release_date_str: String = row[1].to_string();
        let age_str: String = row[2].to_string();
        let gender_str: String = row[3].to_string();
        let place_str: String = row[4].to_string();
        let leave_str: String = row[5].to_string();

        /*
            陽性者の属性シートのデータ構造
            1列目: 通し番号
            2列目: 公表日
            3列目: 年齢
            4列目: 性別
            5列目: 居住地
            6列目: 退院
        */

        // 生成した構造体をpatientsに追加する際、空チェックとハイフンチェックを行う
        patients.push(Patient {
            number: number,
            release_date: if release_date_str == "" { None } else { Some(convert_japanese_era_to_utc(&release_date_str).unwrap()) },
            age: if age_str == "" || gender_str == "-" { None } else { Some(age_str) },
            gender: if gender_str == "" || gender_str == "-" { None } else { Some(gender_str) },
            place: if place_str == "" { None } else { Some(place_str) },
            leave: if leave_str == "" { None } else { Some(leave_str) }
        });
    }

    return patients;
}
