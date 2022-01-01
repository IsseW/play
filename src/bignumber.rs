pub struct Number {
    value: Vec<u8>,
    negative: bool,
}

impl Number {
    pub fn zero() -> Self {
        Self {
            value: Vec::new(),
            negative: false,
        }
    }

    fn shave(&mut self) {
        while self.value.len() > 1 && self.value[self.value.len() - 1] == 0 {
            self.value.pop();
        }
    }

    fn shift_left(&mut self, n: usize) {
        let mut carry = 0;
        for i in (0..self.value.len()).rev() {}
    }
}
