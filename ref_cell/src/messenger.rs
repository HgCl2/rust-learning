

pub trait Logger {
    fn warning(&self, msg: &str);
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
}

pub struct Tracker{
    pub logger: dyn Logger,
    pub value: usize,
    pub max: usize,
}

impl Tracker{
    fn new(logger: dyn Logger, value: usize, max: usize) -> Tracker{
        Tracker {
            logger,
            value,
            max,
        }
    }

    fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage: f64 = value as f64 / self.max as f64;

        if percentage >= 1.0{
            self.error("Error: you are over your quota!");
        }else if percentage >= 0.7 && percentage < 1.0{
            let message = format!("Warning: you have used up over {}% of your quota! Proceeds with precaution", percentage * 100);
            self.warning(&message);
        }
    }

    fn peek(&self) {
        let percentage: f64 = self.value as f64 / self.max as f64;
        let message = format!("Info: you are using up to {}% of your quota", percentage * 100);
        self.info(&message);
    }
}