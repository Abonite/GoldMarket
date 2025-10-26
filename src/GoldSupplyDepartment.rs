use crate::TransactionBehavior::Behavior;

use std::ops::Not;
use std::{collections::HashMap, result::Result};
use deepseek_api::{CompletionsRequestBuilder, DeepSeekClient, DeepSeekClientBuilder, RequestBuilder};
use deepseek_api::request::MessageRequest;
use deepseek_api::response::ModelType;

pub struct RoyalBank {
    gold_reserve: u128,
    currency_R_reserves: u128,

    deepseek_client: DeepSeekClient
}

impl RoyalBank {
    /// # RoyalBank
    /// Royal bank means the
    pub fn new(init_gold_reserve: u64, init_currency_R_reserves: u64, ds_key: &str) -> RoyalBank {
        RoyalBank {
            gold_reserve: init_gold_reserve as u128,
            currency_R_reserves: init_currency_R_reserves as u128,

            deepseek_client: match DeepSeekClientBuilder::new(ds_key.to_string()).build() {
                Err(e) => {
                    panic!("RoyalBank deepSeek client build error: {}", e);
                },
                Ok(v) => v,
            }
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

    pub async fn tradingStrategies(
        &mut self,
        current_gold_price: f64,
        current_R_price: f64,
        gold_meanline: HashMap<u16, Vec<f64>>,
        R_meanline: HashMap<u16, Vec<f64>>
    ) -> Result<Behavior, String> {
        const method: &str = r#"You are currently in a simulated trading platform. 
        You are acting as a central banker, responsible for maintaining the relative stability of your currency relative to gold. 
        You will be trading in "R" currency. You can only perform the following operations: 
            1. Buy a specified amount of gold (in milligrams). For example, reply "buy gold 2000000000mg" to buy 2 tons of gold.
            2. Sell a specified amount of gold (in milligrams). For example, reply "sell gold 2000000000mg" to sell 2 tons of gold.
            3. No operation."#;

        let infos = format!("The following data is provided for your reference:You currently have {} mg of gold reserves and {} units of R reserves. \
            The current gold price is {} R per gram, and the current R price is {} grams of gold per unit. \
            The 5-day moving average of the gold price over the past 30 days is: {:?}; \
            the 10-day moving average of the gold price over the past 30 days is: {:?}; \
            the 20-day moving average of the gold price over the past 30 days is: {:?}; \
            the 30-day moving average of the gold price over the past 30 days is: {:?}; \
            the 60-day moving average of the gold price over the past 30 days is: {:?}; \
            The 120-day moving average data for gold prices over the past 30 days is: {:?}; \
            the 250-day moving average data for gold prices over the past 30 days is: {:?}; \
            the 300-day moving average data for gold prices over the past 30 days is: {:?}; \
            the 5-day moving average data for gold prices over the past 30 days is: {:?}; \
            the 10-day moving average data for gold prices over the past 30 days is: {:?}; \
            the 20-day moving average data for gold prices over the past 30 days is: {:?}; \
            the 30-day moving average data for gold prices over the past 30 days is: {:?}; \
            the 60-day moving average data for gold prices over the past 30 days is: {:?}; \
            the 120-day moving average data for gold prices over the past 30 days is: {:?}; \
            the 250-day moving average data for gold prices over the past 30 days is: {:?}; \
            the 300-day moving average data for gold prices over the past 30 days is: {:?}.",
            self.gold_reserve,
            self.currency_R_reserves,
            current_gold_price,
            current_R_price,
            match gold_meanline.get(&5) {
                None => Vec::new(),
                Some(v) => v.clone(),
            },
            match gold_meanline.get(&10) {
                None => Vec::new(),
                Some(v) => v.clone(),
            },
            match gold_meanline.get(&20) {
                None => Vec::new(),
                Some(v) => v.clone(),
            },
            match gold_meanline.get(&30) {
                None => Vec::new(),
                Some(v) => v.clone(),
            },
            match gold_meanline.get(&60) {
                None => Vec::new(),
                Some(v) => v.clone(),
            },
            match gold_meanline.get(&120) {
                None => Vec::new(),
                Some(v) => v.clone(),
            },
            match gold_meanline.get(&250) {
                None => Vec::new(),
                Some(v) => v.clone(),
            },
            match gold_meanline.get(&300) {
                None => Vec::new(),
                Some(v) => v.clone(),
            },
            match R_meanline.get(&5) {
                None => Vec::new(),
                Some(v) => v.clone(),
            },
            match R_meanline.get(&10) {
                None => Vec::new(),
                Some(v) => v.clone(),
            },
            match R_meanline.get(&20) {
                None => Vec::new(),
                Some(v) => v.clone(),
            },
            match R_meanline.get(&30) {
                None => Vec::new(),
                Some(v) => v.clone(),
            },
            match R_meanline.get(&60) {
                None => Vec::new(),
                Some(v) => v.clone(),
            },
            match R_meanline.get(&120) {
                None => Vec::new(),
                Some(v) => v.clone(),
            },
            match R_meanline.get(&250) {
                None => Vec::new(),
                Some(v) => v.clone(),
            },
            match R_meanline.get(&300) {
                None => Vec::new(),
                Some(v) => v.clone(),
            }
        );

        let prompt = format!("{}\n{}\nPlease make your decision based on the above data.", method, infos);
        let resp = match CompletionsRequestBuilder::new(&[MessageRequest::user(&prompt)])
            .use_model(ModelType::DeepSeekReasoner)
            .do_request(&self.deepseek_client)
            .await {
                Err(e) => {
                    return Err(format!("RoyalBank trading strategies completions request error: {}", e));
                },
                Ok(v) => v.must_response(),
            };

        let mut is_no_op = false;
        let mut is_buy = false;
        let mut is_sell = false;
        let mut gold_mess = 0;
        for msg in resp.choices.iter() {
            let temp = msg.message.as_ref().unwrap().content.clone();
            if temp.matches("No operation").collect::<Vec<&str>>().is_empty().not() {
                is_no_op = is_no_op | true;
            } else if temp.matches("buy").collect::<Vec<&str>>().is_empty().not() {
                is_buy = is_buy | true;
                gold_mess = match u64::from_str_radix(
                match temp.split(" ").collect::<Vec<&str>>().get(2) {
                        Some(v) => &v.trim_end_matches("mg"),
                        None => return Err("RoyalBank trading strategies completions request error: The large model return operation does not follow the prompt words.".to_string()),
                    }, 10) {
                            Ok(v) => v,
                            Err(e) => return Err(format!("RoyalBank trading strategies completions request error: {}", e)),
                };
            } else if temp.matches("sell").collect::<Vec<&str>>().is_empty().not() {
                is_sell = is_sell | true;
                gold_mess = match u64::from_str_radix(
                match temp.split(" ").collect::<Vec<&str>>().get(2) {
                        Some(v) => &v.trim_end_matches("mg"),
                        None => return Err("RoyalBank trading strategies completions request error: The large model return operation does not follow the prompt words.".to_string()),
                    }, 10) {
                            Ok(v) => v,
                            Err(e) => return Err(format!("RoyalBank trading strategies completions request error: {}", e)),
                };
            }
        }

        match (is_no_op, is_buy, is_sell) {
            (false, true, false) => Ok(Behavior::Buy(gold_mess)),
            (false, false, true) => Ok(Behavior::Sell(gold_mess)),
            _ => Ok(Behavior::Noop)
        }
    }

    pub async fn productionStrategy(&mut self, current_strategy: String, gold_production: u64, gold_meanline: HashMap<u16, Vec<f64>>, R_meanline: HashMap<u16, Vec<f64>>) {
        const method: &str = r#"You are currently in a simulated trading platform. 
        You are acting as a central banker, responsible for maintaining the relative stability of your currency relative to gold. 
        You will be trading in "R" currency. Now that the market is closed, you need to make a decision about tomorrow's gold production.
        You can only perform the following operations: 
            1. Request a production increase (in milligrams). The mining department will increase gold production over the next one to several days. For example, reply "increase gold 50000000000" to increase daily production to 50 tons.
            2. Request a production reduction (in milligrams). The mining department will immediately reduce gold production. For example, reply "reduction gold 1000000000" to reduce daily production to 1 ton.
            3. Request the production department to resume normal production.
            4. No operation."#;

        let infos = format!("The following data is provided for your reference:You currently have {} mg of gold reserves and {} units of R reserves. \
            The 5-day moving average of the gold price over the past 30 days is: {:?}; \
            the 10-day moving average of the gold price over the past 30 days is: {:?}; \
            the 20-day moving average of the gold price over the past 30 days is: {:?}; \
            the 30-day moving average of the gold price over the past 30 days is: {:?}; \
            the 60-day moving average of the gold price over the past 30 days is: {:?}; \
            The 120-day moving average data for gold prices over the past 30 days is: {:?}; \
            the 250-day moving average data for gold prices over the past 30 days is: {:?}; \
            the 300-day moving average data for gold prices over the past 30 days is: {:?}; \
            the 5-day moving average data for gold prices over the past 30 days is: {:?}; \
            the 10-day moving average data for gold prices over the past 30 days is: {:?}; \
            the 20-day moving average data for gold prices over the past 30 days is: {:?}; \
            the 30-day moving average data for gold prices over the past 30 days is: {:?}; \
            the 60-day moving average data for gold prices over the past 30 days is: {:?}; \
            the 120-day moving average data for gold prices over the past 30 days is: {:?}; \
            the 250-day moving average data for gold prices over the past 30 days is: {:?}; \
            the 300-day moving average data for gold prices over the past 30 days is: {:?}.",
            self.gold_reserve,
            self.currency_R_reserves,
            match gold_meanline.get(&5) {
                None => Vec::new(),
                Some(v) => v.clone(),
            },
            match gold_meanline.get(&10) {
                None => Vec::new(),
                Some(v) => v.clone(),
            },
            match gold_meanline.get(&20) {
                None => Vec::new(),
                Some(v) => v.clone(),
            },
            match gold_meanline.get(&30) {
                None => Vec::new(),
                Some(v) => v.clone(),
            },
            match gold_meanline.get(&60) {
                None => Vec::new(),
                Some(v) => v.clone(),
            },
            match gold_meanline.get(&120) {
                None => Vec::new(),
                Some(v) => v.clone(),
            },
            match gold_meanline.get(&250) {
                None => Vec::new(),
                Some(v) => v.clone(),
            },
            match gold_meanline.get(&300) {
                None => Vec::new(),
                Some(v) => v.clone(),
            },
            match R_meanline.get(&5) {
                None => Vec::new(),
                Some(v) => v.clone(),
            },
            match R_meanline.get(&10) {
                None => Vec::new(),
                Some(v) => v.clone(),
            },
            match R_meanline.get(&20) {
                None => Vec::new(),
                Some(v) => v.clone(),
            },
            match R_meanline.get(&30) {
                None => Vec::new(),
                Some(v) => v.clone(),
            },
            match R_meanline.get(&60) {
                None => Vec::new(),
                Some(v) => v.clone(),
            },
            match R_meanline.get(&120) {
                None => Vec::new(),
                Some(v) => v.clone(),
            },
            match R_meanline.get(&250) {
                None => Vec::new(),
                Some(v) => v.clone(),
            },
            match R_meanline.get(&300) {
                None => Vec::new(),
                Some(v) => v.clone(),
            }
        );

        let prompt = format!("{}\n{}\nPlease make your decision based on the above data.", method, infos);
        let resp = match CompletionsRequestBuilder::new(&[MessageRequest::user(&prompt)])
            .use_model(ModelType::DeepSeekReasoner)
            .do_request(&self.deepseek_client)
            .await {
                Err(e) => {
                    println!("RoyalBank trading strategies completions request error: {}", e);
                    return;
                },
                Ok(v) => v.must_response(),
            };

        let prompt = format!("{}\n{}\nPlease make your decision based on the above data.", method, infos);
        let resp = match CompletionsRequestBuilder::new(&[MessageRequest::user(&prompt)])
            .use_model(ModelType::DeepSeekReasoner)
            .do_request(&self.deepseek_client)
            .await {
                Err(e) => {
                    println!("RoyalBank trading strategies completions request error: {}", e);
                    return;
                },
                Ok(v) => v.must_response(),
            };

        for msg in resp.choices.iter() {
            println!("RoyalBank trading strategies response: {}", msg.message.as_ref().unwrap().content);
        }
    }
}