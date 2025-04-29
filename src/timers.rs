#[derive(Debug)]
pub struct Timers {
    pub dt_register: u8,
    pub st_register: u8
}

impl Timers {
    pub fn default() -> Self {
        let timer = Timers {
            dt_register: 0,
            st_register: 0,
        };

        timer
    }

    pub fn decrement_timers(&mut self) {
        if self.dt_register > 0 {
            self.dt_register -= 1;
        }
        if self.st_register > 0 {
            self.st_register -= 1;
        }

        //need thread to generate beep when st_register == 0, avoid holding up code
    }
}