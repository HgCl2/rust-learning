mod messenger;
use std::collections::HashMap;
use crate::messenger::Logger;
pub enum MessageType {
    info,
    error,
    warning,
}

pub struct Worker{
    pub track_value: usize,
    pub mapped_message: HashMap<MessageType, String>,
    pub all_messages: Vec<String>,
}

impl Worker {
    fn new(value: usize) -> Worker {
        Worker { 
            track_value: value, 
            mapped_message: HashMap::new(), 
            all_messages: Vec::new(),
        }
    }
}

impl Logger for Worker {
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
