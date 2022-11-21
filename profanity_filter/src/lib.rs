pub struct Message {
    content: String,
    user: String,
}

impl Message {
    pub fn new(ms: String, u: String) -> Message {
        Message {
            content: ms,
            user: u,
        }
    }

    pub fn send_ms(&self) -> Option<&str> {
        if self.content.is_empty() || 
            self.content.contains("stupid") {
            return None
        }else {
            Some(&(self.content))
        }
    }
}

pub fn check_ms(ms: &Message) -> (bool, &str) {
    match ms.send_ms() {
        None => (false, "ERROR: illegal"),
        _ => (true, &ms.content),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let m0 = Message::new("hello there".to_string(), "toby".to_string());
        assert_eq!(check_ms(&m0), (true, "hello there"));

        let m1 = Message::new("".to_string(), "toby".to_string());
        assert_eq!(check_ms(&m1), (false, "ERROR: illegal"));
        
        let m2 = Message::new("you are stupid".to_string(), "toby".to_string());
        assert_eq!(check_ms(&m2), (false, "ERROR: illegal"));

    }
}
