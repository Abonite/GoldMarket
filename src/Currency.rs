pub struct R {
    value: f64
}

impl R {
    pub fn new(init_value: f64) -> R {
        R {
            value: init_value
        }
    }

    pub fn setCurrentValue(&self, current_value: f64) {
        self.value = current_value;
    }
}