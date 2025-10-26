mod TransactionBehavior;
use TransactionBehavior::behavior;

pub struct ForeignInstitutionTrader {
    hold_gold_weight: f64,
    the_amount_of_currency_R_held: f64
}

impl ForeignInstitutionTrader {
    pub fn new(init_gold_holdings: f64, init_R_holdings: f64) -> ForeignTrader {
        ForeignTrader {
            hold_gold_weight: init_gold_holdings,
            the_amount_of_currency_R_held: init_R_holdings
        }
    }

    pub fn strategicJudgment(&self) -> behavior {
    }
}