mod GoldProductionDepartment;
mod GoldSupplyDepartment;
use GoldProductionDepartment::MiningDepartment;
use GoldSupplyDepartment::RoyalBank;

fn main() {
    let mut RB = RoyalBank::new(1.0, 2.0);
    for i in 0..10 {
        let MD = MiningDepartment::new(1.0);
        RB.getGoldOutput(MD.goldOutput());
        println!("current royal bank gold: {}", RB.checkGold());
    }
}
