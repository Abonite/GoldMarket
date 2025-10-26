use std::result::Result;

// sk-b9c7388a03c34f7c9c90fb84dfb1bc5b
pub struct RoyalBank {
    gold_reserve: u128,
    currency_R_reserves: u128
}

impl RoyalBank {
    /// # RoyalBank
    /// Royal bank means the
    pub fn new(init_gold_reserve: u64, init_currency_R_reserves: u64) -> RoyalBank {
        RoyalBank {
            gold_reserve: init_gold_reserve as u128,
            currency_R_reserves: init_currency_R_reserves as u128
        }
    }

    pub fn storageGold(&mut self, gold_mess: u64) {
        self.gold_reserve += gold_mess as u128;
    }

    pub fn getGold(&mut self, gold_mess: u64) -> Result<(), &'static str> {
        if gold_mess as u128 > self.gold_reserve {
            Err("Royal bank gold reserve is not enough!")
        } else {
            self.gold_reserve -= gold_mess as u128;
            Ok(())
        }
    }

    pub fn storageR(&mut self, R_value: u64) {
        self.currency_R_reserves += R_value as u128;
    }

    pub fn getR(&mut self, R_value: u64) -> Result<(), &'static str> {
        if R_value as u128 > self.currency_R_reserves {
            Err("Royal bank currency R reserves is not enough!")
        } else {
            self.currency_R_reserves -= R_value as u128;
            Ok(())
        }
    }
}