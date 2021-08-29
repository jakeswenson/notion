use super::{DateOrDateTime, PropertyValue};
use chrono::NaiveDate;

#[test]
fn verify_date_parsing() {
    let date = NaiveDate::from_ymd(2021, 01, 02);
    let result = serde_json::to_string(&DateOrDateTime::Date(date)).unwrap();
    let parsed: DateOrDateTime = serde_json::from_str(&result).unwrap();
    println!("{:?}", parsed);
}

#[test]
fn parse_date_property() {
    let _property: PropertyValue =
        serde_json::from_str(include_str!("tests/date_property.json")).unwrap();
}

#[test]
fn parse_null_select_property() {
    let _property: PropertyValue =
        serde_json::from_str(include_str!("tests/null_select_property.json")).unwrap();
}

#[test]
fn parse_select_property() {
    let _property: PropertyValue =
        serde_json::from_str(include_str!("tests/select_property.json")).unwrap();
}
