use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;



#[derive(Debug)]
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

#[derive(Debug)]
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
}
