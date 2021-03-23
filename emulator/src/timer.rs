use crate::cpu_recievable::{Recievables, CpuRecievable::*, Interrupt};

pub struct Timer {
    div: u8, // register FF04 (increments at 16384Hz [I.E. CPU Clock / 256])
    tima: u8, // register FF05 (timer register)
    tma: u8, // register FF06 (timer modulo)
    tac: u8, // register FF07 (timer controller)
    recievables: Option<Recievables>,
    ticker: usize,
}

impl Timer {
    pub fn new() -> Timer {
        Self {
            div: 0, tima: 0, tma: 0, tac: 0, recievables: None, ticker: 0
        }
    }

    pub fn set_recievables(&mut self, recievables: Recievables) {
        self.recievables = Some(recievables)
    }

    pub fn read(&self, loc: u16) -> u8 {
        match loc {
            0xFF04 => self.div,
            0xFF05 => self.tima,
            0xFF06 => self.tma,
            0xFF07 => self.tac,
            _ => panic!("Address should not have been routed to timer"),
        }

    }

    pub fn write(&mut self, loc: u16, val: u8) {
        match loc {
            0xFF04 => self.div = 0,
            0xFF05 => self.tima = val,
            0xFF06 => self.tma = val,
            0xFF07 => self.tac = val,
            _ => panic!("Address should not have been routed to timer"),
        }
    }

    pub fn tick(&mut self) {
        self.ticker = (self.ticker + 1) & 0xFFFF;
        let divider_clock = 8; // 2^8 = 256
        if self.ticker & ((1 << divider_clock) - 1) == 0 {
            self.div = self.div.wrapping_add(1);
        }

        if self.is_timer_enabled() {
            let timer_clock = self.timer_clock();
            if self.ticker & ((1 << timer_clock) - 1) == 0 {
                if self.tima == 0xFF {
                    self.tima = self.tma;
                    match &self.recievables {
                        Some(r) => r.send(SendInterrupt(Interrupt::Timer)),
                        None => ()
                    }
                } else {
                    self.tima += 1
                }
            }
        }


    }

    fn is_timer_enabled(&self) -> bool {
        self.tac & 0b100 != 0
    }

    fn timer_clock(&self) -> usize {
        match self.tac & 0b11 {
            0b00 => 10, // 2^10 = 1024
            0b01 => 4, // 2^4 = 16
            0b10 => 6, // 2^6 = 64
            0b11 => 8,// 2^8 = 256
            _ => panic!("This should be impossible")
        }
    }
}
