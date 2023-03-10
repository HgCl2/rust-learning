use std::fs::File;
use std::io::ErrorKind;
use std::fs;

pub fn open_or_create(file: &str, content: &str) {
    let opening_result = File::open(file);

    match opening_result {
        Ok(_open_file) => fs::write(file, content).unwrap(),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(file) {
                Ok(_created_file) => fs::write(file, content).unwrap(),
                Err(err) => panic!("{}", err),
            },
            other_error => panic!("{}", other_error)
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let path = "a.txt";
        File::create(path).unwrap();
        open_or_create(path, "my content");

        let mut file = File::open(path).unwrap();

        let mut s: String = fs::read_to_string(path).unwrap();
        assert_eq!(s, "my content".to_string());
    }
}
