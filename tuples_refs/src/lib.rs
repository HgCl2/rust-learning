#[derive(Debug)]
#[derive(Eq)]
#[derive(PartialEq)]
pub struct Student(pub u32, pub String, pub String);

pub fn id(student: &Student) -> u32 {
    return student.0;
}

pub fn first_name(student: &Student) -> String {
    return student.1.clone();
}

pub fn last_name(student: &Student) -> String {
    return student.2.clone();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let student = Student(20, "Pedro".to_string(), "Domingos".to_string());
        let result = (id(&student), first_name(&student), last_name(&student));
        let right = (student.0, student.1, student.2);
        assert_eq!(right, result);
    }
}
