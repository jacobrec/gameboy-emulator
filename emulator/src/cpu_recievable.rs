use serde::{Serialize, Deserialize};
use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;


pub fn none_recivables() -> Option<Recievables> {
    None
}
pub fn new_recievables() -> Recievables {
    Recievables::new()
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Interrupt {
    VBlank,
    LCDStat,
    Timer,
    Serial,
    Joypad,
}

#[derive(Debug, Clone)]
pub struct Recievables {
    data: Rc<RefCell<VecDeque<CpuRecievable>>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CpuRecievable {
    Delay(usize, Box<CpuRecievable>),
    SendInterrupt(Interrupt),
    EnableInterrupts,
}

impl Recievables {
    pub fn new() -> Self {
        Recievables {
            data: Rc::new(RefCell::new(VecDeque::new()))
        }
    }

    pub fn send(&self, cr: CpuRecievable) {
        self.data.borrow_mut().push_back(cr)
    }

    pub fn recieve(&self) -> Option<CpuRecievable> {
        self.data.borrow_mut().pop_front()
    }

    pub fn serialized_data(&self) -> Vec<CpuRecievable> {
        let mut recievables = Vec::new();
        for x in self.data.borrow().iter() {
            recievables.push(x.clone());
        }
        return recievables

    }
}
