use crate::Patient;
use crate::utils::date_format::{convert_datetime_to_date_and_time, convert_utc_to_date};
use crate::utils::merge_age_and_gender::merge_age_and_gender;
use chrono::{Local, SecondsFormat};
use crate::structs::json::{jsonize_patient::JsonizePatient, patients::Patients};

pub fn jsonize_patients_generate(patients: Vec<Patient>) -> String {

    // JSONにシリアライズする形態の構造体を生成
    let mut serialize_patients: Patients = Patients {
        data: Vec::new(),
        last_update: convert_datetime_to_date_and_time(Local::now())
    };

    let patients_length: usize = patients.len();

    for i in 0..patients_length {
        serialize_patients.data.push(JsonizePatient {
            number: patients[i].number,
            release_date: if patients[i].release_date.is_some() { Some(patients[i].release_date.unwrap().to_rfc3339_opts(SecondsFormat::Millis, true)) } else { None },
            place: if patients[i].place.is_some() { patients[i].place.clone().unwrap() } else { String::from("") },
            age_and_gender: if patients[i].age.is_none() && patients[i].gender.is_none() {
                    String::from("")
                } else {
                    let mut age: String = String::from("");
                    let mut gender: String = String::from(""); 

                    if patients[i].age.is_some() {
                        age = patients[i].age.clone().unwrap();
                    }

                    if patients[i].gender.is_some() {
                        gender = patients[i].gender.clone().unwrap();
                    }
                    merge_age_and_gender(&age, &gender)
                },
            leave: if patients[i].leave.is_some() {
                    Some(patients[i].leave.clone().unwrap())
                } else {
                    None
                },
            date: convert_utc_to_date(patients[i].release_date.unwrap())
        });
    }

    // 陽性者の属性を可読性の高い形でシリアライズ
    return serde_json::to_string_pretty(&serialize_patients).unwrap();

}
