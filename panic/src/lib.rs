use std::fs::File;
use std::io::ErrorKind;

pub fn open_file(s: &str) -> File {
    let file_result = File::open(s);
    match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!("File not found"),
            other_error => {
                panic!("another error");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
