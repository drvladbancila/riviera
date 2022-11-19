use crate::cli::CLI;

#[allow(dead_code)]
pub struct UART {
    rhr: u8,
    thr: u8,
    ier: u8,
    fcr: u8,
    isr: u8,
    lcr: u8,
    mcr: u8,
    lsr: u8,
    msr: u8,
    spr: u8,
    terminal: CLI
}

#[allow(dead_code)]
impl UART {
    const RHR_THR_ADDR: u8 = 0x0;
    const IER_ADDR:     u8 = 0x1;
    const FCR_ISR_ADDR: u8 = 0x2;
    const LCR_ADDR:     u8 = 0x3;
    const MCR_ADDR:     u8 = 0x4;
    const LSR_ADDR:     u8 = 0x5;
    const MSR_ADDR:     u8 = 0x6;
    const SPR_ADDR:     u8 = 0x7;

    pub fn new() -> UART {
        UART {
            rhr: 0, thr: 0, ier: 0,
            fcr: 0, isr: 0, lcr: 0,
            mcr: 0, lsr: 0x1, msr: 0,
            spr: 0, terminal: CLI::new()
        }
    }

    fn thr_full(&self) -> bool {
        (self.lsr >> 6) & 0x1 == 0x0
    }

    fn set_thr_full(&mut self) {
        self.lsr = self.lsr & 0b10111111;
    }

    fn set_thr_empty(&mut self) {
        self.lsr = self.lsr | 0b01000000;
    }

    fn rhr_ready(&self) -> bool {
        self.lsr & 0x1 == 0x1
    }

    fn rhr_set_not_ready(&mut self) {
        self.lsr = self.lsr & 0b11111110;
    }

    fn rhr_set_ready(&mut self) {
        self.lsr = self.lsr | 0b00000001;
    }

    pub fn cycle(&mut self) {
        if self.thr_full() && self.thr != 0 {
            self.terminal.write_byte(self.thr);
            self.set_thr_empty()
        }

        if self.rhr_ready() {
            self.rhr = self.terminal.read_byte();
            self.rhr_set_not_ready()
        }
    }

    pub fn write(&mut self, addr: u8, data: u8) {
        match addr {
            UART::RHR_THR_ADDR => { self.thr = data;  self.set_thr_full()}
            UART::IER_ADDR     => self.ier = data,
            UART::FCR_ISR_ADDR => self.fcr = data,
            UART::LCR_ADDR     => self.lcr = data,
            UART::MCR_ADDR     => self.mcr = data,
            UART::SPR_ADDR     => self.mcr = data,
            _ => (),
        }
    }

    pub fn read(&mut self, addr: u8) -> u8 {
        match addr {
            UART::RHR_THR_ADDR => {
                let rhr: u8 = self.rhr;
                self.rhr_set_ready();
                self.rhr = 0;
                rhr
            },
            _ => 0x0
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::uart::UART;

    #[test]
    fn write_test() {
        let mut uart = UART::new();
        uart.write(0x0, 0x45);
        uart.cycle();
        uart.write(0x0, 0x46);
        uart.cycle();
        uart.terminal.show_output()
    }

    #[test]
    fn read_test() {
        let mut uart = UART::new();
        uart.terminal.get_input();
        loop {
            uart.cycle();
            let a = uart.read(0);
            if a == 0 {
                break;
            }
            println!("{}", a as char);
        }
    }
}