#[derive(Debug, PartialEq, Eq)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Box<Option<Worker>>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Worker {
    pub role: String,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> WorkEnvironment {
        WorkEnvironment { 
            grade: Box::new(None), 
        }
    }

    pub fn add_worker(&mut self, role: String, name: String) {
        if *self.grade == None{
            *self.grade = Some(Worker{
                role,
                name,
                next: Box::new(None),
            });

            return;
        }

        let p_new_worker: Box<Option<Worker>> = Box::new(Some(Worker{
            role,
            name,
            next: self.grade.clone(),
        }));

        self.grade = p_new_worker;
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        if *self.grade == None{
            return None;
        }

        let name = self.grade.clone().unwrap().name;

        if *self.grade.clone().unwrap().next != None{
            self.grade = self.grade.clone().unwrap().next;
        }else{
            self.grade = Box::new(None);
        }

        return Some(name);
    }

    pub fn last_worker(&self) -> Option<(String, String)> {
        if *self.grade == None{
            return None;
        }

        return Some((self.grade.clone().unwrap().name, self.grade.clone().unwrap().name));
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
    }
}
