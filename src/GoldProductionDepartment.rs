use rand_distr::{Poisson, Distribution};
use rand::Rng;
pub struct MiningDepartment {
    // tons
    ideal_yield: f64
}

impl MiningDepartment {
    pub fn new(ideal_yield: f64) -> MiningDepartment {
        MiningDepartment {
            ideal_yield: ideal_yield
        }
    }

    pub fn goldOutput(&self) -> f64 {
        let poi = Poisson::new(100.0).unwrap();
        let yield_coefficient = poi.sample(&mut rand::thread_rng());
        let yield_coefficient = yield_coefficient / 100.0;
        self.ideal_yield * yield_coefficient
    }
}