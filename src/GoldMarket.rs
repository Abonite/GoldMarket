use crate::GoldSupplyDepartment::RoyalBank;
use crate::GoldProductionDepartment::MiningDepartment;

use std::collections::HashMap;
use deepseek_api::DeepSeekClient;

pub struct Market {
    current_gold_price: f64,
    current_R_price: f64,
    history_gold_price: Vec<f64>,
    history_R_price: Vec<f64>,

    royal_bank: RoyalBank,
    mining_department: MiningDepartment,

    deepseek_client: DeepSeekClient,

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
    pub fn new(init_price: HashMap<&str, f64>, init_reserve: HashMap<&str, u64>, deepseek_client: DeepSeekClient) -> Market {
        Market {
            history_gold_price: Vec::new(),
            history_R_price: Vec::new(),
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
                match init_reserve.get("currency_R_reserves") {
                    None => {
                        println!("Not initial the R reserve! Use defaults value 10.");
                        10
                    },
                    Some(v) => *v,
                }
            ),
            mining_department: MiningDepartment::new(0.9999),
            deepseek_client: deepseek_client,
            buy_orders: Vec::new(),
            sell_orders: Vec::new()
        }
    }

    pub fn daysEndUpdate(&mut self) {

    }
}