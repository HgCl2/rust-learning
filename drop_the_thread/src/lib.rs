use std::cell::{RefCell, Cell};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Workers {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>
}

impl Workers {
    pub fn new() -> Workers {
        Workers { 
            drops: Cell::new(0), 
            states: RefCell::new(Vec::new()) 
        }
    }

    pub fn new_worker(&self, c: String) -> (usize, Thread) {
        self.states.borrow_mut().push(false);
        let pid = self.track_worker() - 1;
        return (pid, Thread::new_thread(pid, c, self));
    }

    pub fn track_worker(&self) -> usize {
        return self.states.borrow().len();
    }

    pub fn is_dropped(&self, id: usize) -> bool {
        return self.states.borrow()[id];
    }

    pub fn add_drop(&self, id: usize) {
        if self.states.borrow_mut()[id]{
            panic!("{} is already dropped", id)
        }else {
            self.states.borrow_mut()[id] = true;
            self.drops.set(self.drops.get() + 1)
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Thread<'a> {
    pub pid: usize,
    pub cmd: String,
    pub parent: &'a Workers
}

impl<'a> Thread<'a> {
    pub fn new_thread(p: usize, c: String, t: &'a Workers) -> Thread {
        Thread { 
            pid: p, 
            cmd: c, 
            parent: t 
        }
    }
    pub fn skill(self) {
        self.parent.add_drop(self.pid);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
