use std::cmp;

pub struct Bar {
    note: u8,
    value: i16,
    active: bool,
}

impl Bar {
    pub fn new(note: u8) -> Bar {
        Bar {
            note,
            value: 0,
            active: false,
        }
    }

    pub fn get_value(&self) -> i16 {
        self.value
    }

    pub fn set_value(&mut self, value: i16) {
        self.value = value;
    }

    pub fn adjust(&mut self, delta: i16) {
        if self.active && self.value < 1000 {
            self.value += delta;
        } else if !self.active && self.value > 0 {
            self.value -= delta;
        }
        self.value = cmp::max(0, cmp::min(1000, self.value));
    }

    pub fn is_active(&self) -> bool {
        self.value > 0
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }
}
