use std::collections::HashMap;

use crate::GoldSupplyDepartment::RoyalBank;
use crate::GoldProductionDepartment::MiningDepartment;
use crate::Trader::ForeignInstitutionTrader;
use crate::TransactionBehavior::Behavior;

pub struct Market {
    current_gold_price: f64,
    current_R_price: f64,

    royal_bank: RoyalBank,
    mining_department: MiningDepartment,
    fits: Vec<ForeignInstitutionTrader>,

    buy_orders: Vec<(f64, u16)>,
    sell_orders: Vec<(f64, u16)>,

    meanline_5day_gold_price: Vec<f64>,
    meanline_5day_R_price: Vec<f64>,
    meanline_10day_gold_price: Vec<f64>,
    meanline_10day_R_price: Vec<f64>,
    meanline_20day_gold_price: Vec<f64>,
    meanline_20day_R_price: Vec<f64>,
    meanline_30day_gold_price: Vec<f64>,
    meanline_30day_R_price: Vec<f64>,
    meanline_60day_gold_price: Vec<f64>,
    meanline_60day_R_price: Vec<f64>,
    meanline_120day_gold_price: Vec<f64>,
    meanline_120day_R_price: Vec<f64>,
    meanline_250day_gold_price: Vec<f64>,
    meanline_250day_R_price: Vec<f64>,
    meanline_300day_gold_price: Vec<f64>,
    meanline_300day_R_price: Vec<f64>
}

impl Market {
    pub fn new(init_price: HashMap<&str, f64>, init_reserve: HashMap<&str, u64>, rb_ds_key: &str, fit_ds_key: &str, fit_num: u16) -> Market {
        Market {
            current_gold_price: match init_price.get("gold_price") {
                None => {
                    println!("Not initial the gold price! Use defaults value 10.0.");
                    10.0
                },
                Some(v) => *v,
            },
            current_R_price: match init_price.get("R_price") {
                None => {
                    println!("Not initial the R price! Use defaults value 10.0.");
                    10.0
                },
                Some(v) => *v,
            },
            royal_bank: RoyalBank::new(
                match init_reserve.get("gold_reserve") {
                    None => {
                        println!("Not initial the gold reserve! Use defaults value 10.");
                        10
                    },
                    Some(v) => *v,
                },
                match init_reserve.get("currency_R_reserve") {
                    None => {
                        println!("Not initial the R reserve! Use defaults value 10.");
                        10
                    },
                    Some(v) => *v,
                },
                rb_ds_key
            ),
            mining_department: MiningDepartment::new(0.9999),
            fits: vec![ForeignInstitutionTrader::new(10_000_000_000, 10_000_000_000, fit_ds_key); fit_num as usize],

            buy_orders: Vec::new(),
            sell_orders: Vec::new(),
            meanline_5day_gold_price: Vec::new(),
            meanline_5day_R_price: Vec::new(),
            meanline_10day_gold_price: Vec::new(),
            meanline_10day_R_price: Vec::new(),
            meanline_20day_gold_price: Vec::new(),
            meanline_20day_R_price: Vec::new(),
            meanline_30day_gold_price: Vec::new(),
            meanline_30day_R_price: Vec::new(),
            meanline_60day_gold_price: Vec::new(),
            meanline_60day_R_price: Vec::new(),
            meanline_120day_gold_price: Vec::new(),
            meanline_120day_R_price: Vec::new(),
            meanline_250day_gold_price: Vec::new(),
            meanline_250day_R_price: Vec::new(),
            meanline_300day_gold_price: Vec::new(),
            meanline_300day_R_price: Vec::new()
        }
    }

    pub fn daysEndUpdate(&mut self) {

    }

    // per 6 seconds
    pub async fn bid(&mut self) -> Result<(), String> {
        let gold_meanline = self.collectGoldMeanline();
        let R_meanline = self.collectRMeanline();
        let ops = match self.royal_bank.tradingStrategies(self.current_gold_price, self.current_R_price, gold_meanline, R_meanline).await {
            Ok(v) => v,
            Err(e) => return Err(e)
        };

        match ops {
            Behavior::Buy(gm) => {
                // how to buy?
                println!("Royal Bank want to buy {}mg gold!", gm);
            },
            Behavior::Sell(gm) => {
                match self.royal_bank.getGold(gm) {
                    Ok(_) => println!("Royal Bank has sold {}mg gold!", gm),
                    Err(e) => println!("Royal Bank couldn't sold {}mg gold!", gm)
                };
            },
            Behavior::Noop => {
                println!("Royal Bank has no operation!");
            }
        }

        Ok(())
    }

    fn collectGoldMeanline(&self) -> HashMap<u16, Vec<f64>> {
        let mut result: HashMap<u16, Vec<f64>> = HashMap::new();
        result.insert(5, self.meanline_5day_gold_price.clone());
        result.insert(10, self.meanline_10day_gold_price.clone());
        result.insert(20, self.meanline_20day_gold_price.clone());
        result.insert(30, self.meanline_30day_gold_price.clone());
        result.insert(60, self.meanline_60day_gold_price.clone());
        result.insert(120, self.meanline_120day_gold_price.clone());
        result.insert(250, self.meanline_250day_gold_price.clone());
        result.insert(300, self.meanline_300day_gold_price.clone());
        result
    }

    fn collectRMeanline(&self) -> HashMap<u16, Vec<f64>> {
        let mut result: HashMap<u16, Vec<f64>> = HashMap::new();
        result.insert(5, self.meanline_5day_R_price.clone());
        result.insert(10, self.meanline_10day_R_price.clone());
        result.insert(20, self.meanline_20day_R_price.clone());
        result.insert(30, self.meanline_30day_R_price.clone());
        result.insert(60, self.meanline_60day_R_price.clone());
        result.insert(120, self.meanline_120day_R_price.clone());
        result.insert(250, self.meanline_250day_R_price.clone());
        result.insert(300, self.meanline_300day_R_price.clone());
        result
    }
}