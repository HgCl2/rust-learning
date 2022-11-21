use std::str::FromStr;
use chrono::{Utc, NaiveDate};

#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (String, String),
    pub date: String,
    pub err: String
}
impl FormError {
    pub fn new(field_name: String, field_value: String, err: String) -> FormError {
        let date = chrono::offset::Local::now();
        let formatted = format!("{}", date.format("%Y-%m-%d %H:%M:%S"));
        FormError { 
            form_values: (field_name, field_value), 
            date: formatted, 
            err: err,
        }
    }
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct Form{
    pub first_name: String,
    pub last_name: String,
    pub birth: NaiveDate,
    pub birth_location: String,
    pub password: String,
}

impl Form{
    pub fn new(f_n: String, l_n: String, birth: NaiveDate, 
        birth_loc: String, passw: String) -> Form{
        Form { 
            first_name: f_n, 
            last_name: l_n, 
            birth: birth, 
            birth_location: birth_loc, 
            password: passw, 
        }
    }

    pub fn validate(&self) -> Result<Vec<&str>, FormError>{
        if self.first_name.is_empty() {
            return Err(FormError::new("first_name".to_string(), 
            self.first_name.clone(), "No user name".to_string()));
        }

        let passw = &(self.password)[..];
        if passw.len() >= 8 && passw.contains(|c: char| c.is_ascii_punctuation()) && 
        passw.contains(|c: char| c.is_alphabetic()) && passw.contains(|c: char| c.is_numeric()){
            return Ok(vec!["Valid first name", "Valid password"]);
        }else if self.password.len() < 8{
            return Err(FormError::new("password".to_string(), self.password.clone(), 
            "At least 8 characters".to_string()));
        }else{
            return Err(FormError::new("password".to_string(), self.password.clone(), 
            "Combination of different ASCII character types (numbers, letters and none alphanumeric characters)".to_string()));
        }
    }
}

pub fn create_date(date: &str) -> NaiveDate {
    match NaiveDate::parse_from_str(&date, "%Y-%m-%d"){
        Ok(d) => d,
        Err(err) => NaiveDate::parse_from_str("2022-11-22", "%Y-%m-%d").unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut form_output = Form::new(
            String::from("Lee"),
            String::from("Silva"),
            create_date("2015-09-05"),
            String::from("Africa"),
            String::from("qwqwsa1dty_"),
        );

        assert_eq!(form_output.validate().unwrap(), ["Valid first name", "Valid password"]);

        //form_output.first_name = String::from("");
        //assert_eq!(form_output.validate().unwrap_err(), FormError { form_values: ("first_name".to_string(), "".to_string()), date: "2022-11-22 01:30:30".to_string(), err: "No user name".to_string() });
        
        form_output.first_name = String::from("as");
        //form_output.password = String::from("dty_1");
        //assert_eq!(form_output.validate().unwrap_err(), FormError { form_values: ("password".to_string(), "dty_1".to_string()), date: "2022-10-17 12:09:25".to_string(), err: "At least 8 characters".to_string() })

        form_output.password = String::from("asdasASd123SA");
        assert_eq!(form_output.validate().unwrap_err(), FormError { form_values: ("password".to_string(), "asdasASd123SA".to_string()), date: "2022-10-17 12:09:25".to_string(), err: "Combination of different ASCII character types (numbers, letters and none alphanumeric characters)".to_string() })
    }
}
