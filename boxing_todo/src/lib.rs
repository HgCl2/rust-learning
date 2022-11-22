mod err;
use err::{ ParseErr, ReadErr };

use serde::{Deserialize, Serialize};
//use serde_json::{Result};
use std::fs::File;
use std::io::Read;
pub use std::error::Error;


#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        let file = File::open(path)?;
        let mut data: String = String::new();
        file.read_to_string(&mut data);
        let json_value: TodoList = serde_json::from_str(&data)?;

        if json_value.tasks.is_empty(){
            return Err(Box::new(ParseErr::Empty));
        }else{
            Ok(json_value)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::fs;

    fn new_todo(s: String, v: Vec<Task>) -> TodoList {
        TodoList { title: s, tasks: v }
    }
    fn run(s: &TodoList, f: &str) -> Result<TodoList, Box<dyn Error>> {
        serde_json::to_writer(&File::create(f)?, s)?;
        let result = TodoList::get_todo(f);
		fs::remove_file(f)?;
        return result;
    }

    #[test]
    fn test_good_todo() {
        let file_name = "todo.json";
        let good_struct = new_todo(
            String::from("todo list for something"),
            vec![
                Task { id: 0, description: "do this".to_string(), level: 0 },
                Task { id: 1, description: "do that".to_string(), level: 5 },
            ],
        );
        let result = run(&good_struct, file_name).unwrap();

        assert_eq!(result.title, good_struct.title);
        assert_eq!(&result.tasks, &good_struct.tasks);
    }

    #[test]
    fn test_empty_tasks() {
        let file_name = "empty_tasks.json";
        let result = run(&new_todo(String::from("empty tasks"), vec![]), file_name).unwrap_err();

        assert_eq!(result.to_string(), "Failed to parses todo");
        assert_eq!(result.description(), "Todo List parse failed: ");
        assert!(!result.cause().is_some());
    }
    #[test]
    fn test_read() {
        let result = TodoList::get_todo("no_file.json").unwrap_err();

        assert_eq!(result.to_string(), "Failed to read todo file");
        assert_eq!(result.description(), "Todo List read failed: ");
    }

    #[test]
    #[should_panic(expected = "Malformed(Error(\"missing field `title`\", line: 1, column: 2))")]
    fn test_malformed_json() {
        #[derive(Serialize, Deserialize)]
        struct Mal {};
        let file_name = "malformed.json";
        let malformed: Mal = serde_json::from_str(r#"{}"#).unwrap();
        serde_json::to_writer(&File::create(file_name).unwrap(), &malformed).unwrap();
        let result = TodoList::get_todo(file_name);
        fs::remove_file(file_name).unwrap();

        result.unwrap_or_else(|e| panic!("{:?}", e));
    }

    #[test]
    #[should_panic(expected = "ReadErr { child_err: Os { code: 2, kind: NotFound, message: \"No such file or directory\" } }")]
    fn test_read_error() {
        TodoList::get_todo("no_file.json").unwrap_or_else(|e| panic!("{:?}", e));
    }
}
