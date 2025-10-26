use std::result::Result;
use rand_distr::{Poisson, Distribution};
use rand::Rng;

pub struct MiningDepartment {
    // percent
    ideal_yield: f64,
    // mili grams
    pub daily_gold_production: u64,

    lambda: f64,
    difference: f64,
    change: f64,
    production_increase_slope: f64,
    max_daily_gold_production: u64,
    min_daily_gold_production: u64,

    increasing: bool,
    reducing: bool
}

impl MiningDepartment {
    pub fn new(ideal_yield: f64) -> MiningDepartment {
        MiningDepartment {
            ideal_yield: ideal_yield,
            daily_gold_production: 0,

            // 40 tons per day
            lambda: 40_000_000_000.0,
            difference: 0.0,
            change: 0.0,
            production_increase_slope: 40_000_000_000.0 * 0.15,
            // 75 tons per day
            max_daily_gold_production: 75_000_000_000,
            min_daily_gold_production: 0,

            increasing: false,
            reducing: false
        }
    }

    pub fn goldProduce(&mut self) {
        let poi: Poisson<f64>;
        if self.increasing {
            if self.lambda + self.change > self.max_daily_gold_production as f64 {
                poi = Poisson::new(self.max_daily_gold_production as f64).unwrap();
            } else {
                if self.change + self.production_increase_slope >= self.difference {
                    poi = Poisson::new(self.lambda + self.difference).unwrap();
                } else {
                    self.change += self.production_increase_slope;
                    poi = Poisson::new(self.lambda + self.change).unwrap();
                }
            }
        } else if self.reducing {
            if self.lambda - self.change < self.min_daily_gold_production as f64 {
                poi = Poisson::new(self.min_daily_gold_production as f64).unwrap();
            } else {
                poi = Poisson::new(self.lambda - self.difference).unwrap();
            }
        } else {
            poi = Poisson::new(self.lambda).unwrap();
        }

        let yield_coefficient = poi.sample(&mut rand::thread_rng());
        self.daily_gold_production = f64::floor(self.ideal_yield * yield_coefficient) as u64;
    }

    pub fn getDailyProduction(&self) -> u64 {
        self.daily_gold_production
    }

    pub fn increaseProduction(&mut self, target: u64) -> Result<(), &'static str> {
        if target > self.max_daily_gold_production {
            Err("Exceed maximum daily production limit!")
        } else {
            self.difference = target as f64 - self.lambda;
            self.increasing = true;
            Ok(())
        }
    }
    
    pub fn reduceProduction(&mut self, target: u64) -> Result<(), &'static str> {
        if target < self.min_daily_gold_production {
            Err("Below minimum daily production limit!")
        } else {
            self.difference = self.lambda - target as f64;
            self.reducing = true;
            Ok(())
        }
    }

    pub fn cancelProductionChange(&mut self) {
        self.increasing = false;
        self.reducing = false;
    }

    pub fn setIdealYield(&mut self, ideal_yield: f64) {
        self.ideal_yield = ideal_yield;
    }
}