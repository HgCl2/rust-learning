#[derive(Debug, PartialEq, Eq)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Worker>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Worker {
    pub role: String,
    pub name: String,
    pub next: Box<Link>,
}

impl WorkEnvironment {
    pub fn new() -> WorkEnvironment {
        WorkEnvironment { 
            grade: None, 
        }
    }

    pub fn add_worker(&mut self, role: String, name: String) {
        if self.grade == None{
            self.grade = Some(Worker{
                role,
                name,
                next: Box::new(None),
            });

            return;
        }

        let p_new_worker: Box<Option<Worker>> = Box::new(Some(Worker{
            role,
            name,
            next: Box::new(self.grade.clone()),
        }));

        self.grade = *p_new_worker;
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        if self.grade == None{
            return None;
        }

        let name = self.grade.clone().unwrap().name;

        if *self.grade.clone().unwrap().next != None{
            self.grade = *self.grade.clone().unwrap().next;
        }else{
            self.grade = None;
        }

        return Some(name);
    }

    pub fn last_worker(&self) -> Option<(String, String)> {
        if self.grade == None{
            return None;
        }

        return Some((self.grade.clone().unwrap().name, self.grade.clone().unwrap().role));
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut list = WorkEnvironment::new();
        list.add_worker(String::from("CEO"), String::from("Marie"));
        assert_eq!(list.grade.as_ref().unwrap().role, "CEO");
    }
}
