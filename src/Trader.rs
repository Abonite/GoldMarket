use crate::TransactionBehavior::Behavior;

use deepseek_api::{DeepSeekClient, DeepSeekClientBuilder};

#[derive(Clone)]
pub struct ForeignInstitutionTrader {
    hold_gold_weight: u64,
    the_amount_of_currency_R_held: u64,

    deepseek_client: DeepSeekClient
}

impl ForeignInstitutionTrader {
    pub fn new(init_gold_holdings: u64, init_R_holdings: u64, fit_ds_key: &str) -> ForeignInstitutionTrader {
        ForeignInstitutionTrader {
            hold_gold_weight: init_gold_holdings,
            the_amount_of_currency_R_held: init_R_holdings,

            deepseek_client: match DeepSeekClientBuilder::new(fit_ds_key.to_string()).build() {
                Err(e) => {
                    panic!("ForeignInstitutionTrader deepSeek client build error: {}", e);
                },
                Ok(v) => v,
            }
        }
    }
}