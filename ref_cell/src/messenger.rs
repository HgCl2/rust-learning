pub use std::rc::Rc;

pub trait Logger {
    fn warning(&self, msg: &str);
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
}

pub struct Tracker<'a, T: Logger>{
    pub logger: &'a T,
    pub value: Rc<usize>,
    pub max: usize,
}

impl<'a, T> Tracker<'a, T>
where T: Logger {
    pub fn new(logger: &'a T, max: usize) -> Tracker<'a, T>{
        Tracker {
            logger,
            value: Rc::new(0),
            max,
        }
    }

    pub fn set_value(&'a self, value: &'a Rc<usize>) {
        let percentage: f64 = Rc::strong_count(value) as f64 / self.max as f64;

        if percentage >= 1.0{
            self.logger.error("Error: you are over your quota!");
        }else if percentage >= 0.7 && percentage < 1.0{
            let message = format!("Warning: you have used up over {}% of your quota! Proceeds with precaution", (percentage * 100.0) as i64);
            self.logger.warning(&message);
        }
    }

    pub fn peek(&self, track_value: &'a Rc<usize>) {
        let percentage: f64 = Rc::strong_count(track_value) as f64 / self.max as f64;
        let message = format!("Info: you are using up to {}% of your quota", (percentage * 100.0) as i64);
        self.logger.info(&message);
    }
}