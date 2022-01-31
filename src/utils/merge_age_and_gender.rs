pub fn merge_age_and_gender(age: &str, gender: &str) -> String {

    let mut result: String = String::from("");

    // 空になっていないか確認し、またハイフン（情報なし）になっていないか確認
    if age != "-" && !(age.is_empty()) {
        // 年齢の末尾が以上で終わっていた場合（90歳や100歳以上など）
        if age.ends_with("以上") {
            result.push_str(&age.replace("以上", "歳以上"));
        } else if age.ends_with("未満") {
            result.push_str(&age.replace("未満", "歳未満"));
        } else {
            result.push_str(&age);
            result.push_str("代");
        }
    }
    
    // 空になっていないか確認し、またハイフン（情報なし）になっていないか確認
    if gender != "-" && !(gender.is_empty()) {
        result.push_str(&gender);
    }

    return result;

}
