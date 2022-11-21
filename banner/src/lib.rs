use std::collections::HashMap;
use std::num::ParseFloatError;
use std::str::FromStr;

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl Flag {
    pub fn opt_flag(l_h: &str, d: &str) -> Flag {
        let first_letter = l_h.chars().nth(0).unwrap();
        
        let mut s_h = "-".to_string();
        s_h.push(first_letter);

        let mut l_h = String::from(l_h);
        l_h = "--".to_string()+ &l_h;

        Flag {
            short_hand: s_h,
            long_hand: l_h,
            desc: d.to_string(),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;


#[derive(Clone)]
pub struct FlagsHandler {
    pub flags: HashMap<(String, String), Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: (String, String), func: Callback) {
        self.flags.insert(flag, func);
    }

    pub fn exec_func(&mut self, flag: (String, String), argv: &[&str]) -> String {
        match self.flags[&flag](argv[0], argv[1]) {
            Ok(res) => res,
            Err(_) => "invalid float literal".to_string()
        }
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let first_num = match f32::from_str(a){
        Ok(val) => val,
        Err(e) => return Err(e),
    };

    let second_num = match f32::from_str(b){
        Ok(val) => val,
        Err(e) => return Err(e),
    };

    let result = first_num / second_num;

    return Ok(result.to_string());
}
pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let first_num = match f32::from_str(a){
        Ok(val) => val,
        Err(e) => return Err(e),
    };

    let second_num = match f32::from_str(b){
        Ok(val) => val,
        Err(e) => return Err(e),
    };

    let res = first_num % second_num;

    return Ok(res.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut handler = FlagsHandler { flags: HashMap::new() };

    let d = Flag::opt_flag("division", "divides the values, formula (a / b)");
    let r = Flag::opt_flag(
        "remainder",
        "remainder of the division between two values, formula (a % b)",
    );

    handler.add_flag((d.short_hand, d.long_hand), div);
    handler.add_flag((r.short_hand, r.long_hand), rem);

    assert_eq!(handler.exec_func(("-d".to_string(), "--division".to_string()), &["1.0", "2.0"]), "0.5".to_string());
    assert_eq!(handler.exec_func(("-r".to_string(), "--remainder".to_string()), &["2.0", "2.0"]), "0".to_string());
    assert_eq!(handler.exec_func(("-d".to_string(), "--division".to_string()), &["a", "2.0"]), "invalid float literal".to_string());
    assert_eq!(handler.exec_func(("-r".to_string(), "--remainder".to_string()), &["2.0", "fd"]), "invalid float literal".to_string());
    }
}
