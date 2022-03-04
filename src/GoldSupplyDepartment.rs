pub struct RoyalBank {
    gold_reserve: f64,
    currency_R_reserves: f64
}

impl RoyalBank {
    /// # RoyalBank
    /// Royal bank means the
    pub fn new(init_gold_reserve: f64, init_currency_R_reserves: f64) -> RoyalBank {
        RoyalBank {
            gold_reserve: init_gold_reserve,
            currency_R_reserves: init_currency_R_reserves
        }
    }

    pub fn getGoldOutput(&mut self, gold_output: f64) {
        self.gold_reserve += gold_output;
    }

    pub fn returnGold(&mut self, gold_mess: f64) {
        self.gold_reserve += gold_mess;
    }

    pub fn returnR(&mut self, R_reserves: f64) {
        self.currency_R_reserves += R_reserves;
    }

    pub fn putGold(&mut self, gold_mess: f64) {
        self.gold_reserve -= gold_mess;
    }

    pub fn putR(&mut self, R_reserves: f64) {
        self.currency_R_reserves -= R_reserves;
    }

    pub fn checkGold(&self) -> f64 {
        return self.gold_reserve;
    }

    // pub fn strategicJudgment(&self) -> behavior {
    //     behavior::buy
    // }
}