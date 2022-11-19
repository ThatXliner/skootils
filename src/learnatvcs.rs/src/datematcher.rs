use lazy_static::lazy_static;
use regex::Regex;
lazy_static! {
    static ref CLASS_DAY_RE: Regex = Regex::new(
        r"(?P<amonth>[a-zA-Z]+) (?P<aday>\d+)(?:/(?P<bmonth>[a-zA-Z]+)?[ ]?(?P<bday>\d+))?"
    )
    .unwrap();
}
pub struct Date {
    month: String,
    day: ui8,
}
impl ToString for Date {
    fn to_string(&self) -> String {
        return format!("{} {}", &self.month, &self.day);
    }
}
