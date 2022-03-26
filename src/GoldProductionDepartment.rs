use rand_distr::{Poisson, Distribution};
use rand::Rng;
pub struct MiningDepartment {
    // tons
    ideal_yield: f64,
    pub daily_gold_production: f64,
}

impl MiningDepartment {
    pub fn new(ideal_yield: f64) -> MiningDepartment {
        MiningDepartment {
            ideal_yield: ideal_yield,
            daily_gold_production: 0.0,
        }
    }

    pub fn gold_output(&mut self) -> f64 {
        let poi = Poisson::new(100_000_000.0).unwrap();
        let yield_coefficient = poi.sample(&mut rand::thread_rng());
        let yield_coefficient = yield_coefficient / 100_000_000.0;
        self.daily_gold_production = self.ideal_yield * yield_coefficient;

        self.daily_gold_production
    }
}