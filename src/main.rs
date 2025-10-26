mod GoldMarket;
mod GoldProductionDepartment;
mod GoldSupplyDepartment;
mod Trader;
mod TransactionBehavior;

use anyhow::Result;
use deepseek_api::response::FinishReason;
use schemars::schema::SchemaObject;
use std::{collections::HashMap, str::FromStr};
use deepseek_api::request::{Function, ToolMessageRequest, ToolObject, ToolType, MessageRequest};
use deepseek_api::{DeepSeekClientBuilder, CompletionsRequestBuilder, RequestBuilder};

use GoldMarket::Market;
use GoldProductionDepartment::MiningDepartment;
use GoldSupplyDepartment::RoyalBank;
use Trader::ForeignInstitutionTrader;

const INIT_GOLD_PRICE: f64 = 10.0;
const INIT_R_PRICE: f64 = 10.0;
const INIT_GOLD_REVERSE: u64 = 10;
const INIT_R_REVERSE: u64 = 10;

const GM_RB_DS_API_KEY: &str = "sk-b9c7388a03c34f7c9c90fb84dfb1bc5b";
const GM_FIT_DS_API_KEY: &str = "sk-677aa1f7a1794058aabd85f2749cd53c";

#[tokio::main]
async fn main() {
    let api_key = "sk-b9c7388a03c34f7c9c90fb84dfb1bc5b".to_string();
    let client = match DeepSeekClientBuilder::new(api_key).build() {
        Err(e) => {
            println!("Client build error: {}", e);
            return;
        },
        Ok(v) => v,
    };

    let mut init_price = HashMap::new();
    let mut init_reverse = HashMap::new();
    init_price.insert("gold_price", INIT_GOLD_PRICE);
    init_price.insert("R_price", INIT_R_PRICE);
    init_reverse.insert("gold_reserve", INIT_GOLD_REVERSE);
    init_reverse.insert("currency_R_reserve", INIT_R_REVERSE);

    let mut market = Market::new(init_price, init_reverse, GM_RB_DS_API_KEY, GM_FIT_DS_API_KEY, 5);
    match market.bid().await {
        Ok(_) => (),
        Err(e) => println!("{}", e)
    };
    market.daysEndUpdate();
}

#[test]
fn test_mining_department() {
    let mut mining_dept = MiningDepartment::new(0.999);
    mining_dept.goldProduce();
    println!("Daily gold production: {} mili grams", mining_dept.getDailyProduction());
}

#[test]
fn test_mining_department_increase() {
    let mut mining_dept = MiningDepartment::new(0.999);
    mining_dept.goldProduce();
    let daily_gold_production = mining_dept.getDailyProduction();
    println!("Daily gold production: {} mili grams", daily_gold_production);

    match mining_dept.increaseProduction(f64::ceil(daily_gold_production as f64 * 1.1) as u64) {
        Ok(_) => println!("Increase production request accepted."),
        Err(e) => println!("Increase production request denied: {}", e),
    };
    for _ in 0..10 {
        mining_dept.goldProduce();
        let daily_gold_production = mining_dept.getDailyProduction();
        println!("Daily gold production: {} mili grams", daily_gold_production);
    }
}

#[test]
fn test_mining_department_reduce() {
    let mut mining_dept = MiningDepartment::new(0.999);
    mining_dept.goldProduce();
    let daily_gold_production = mining_dept.getDailyProduction();
    println!("Daily gold production: {} mili grams", daily_gold_production);

    match mining_dept.reduceProduction(f64::ceil(daily_gold_production as f64 * 0.5) as u64) {
        Ok(_) => println!("Reduce production request accepted."),
        Err(e) => println!("Reduce production request denied: {}", e),
    };
    for _ in 0..10 {
        mining_dept.goldProduce();
        let daily_gold_production = mining_dept.getDailyProduction();
        println!("Daily gold production: {} mili grams", daily_gold_production);
    }
}

// #[tokio::main]
// async fn main() {
//     let api_key = match String::from_str("sk-b9c7388a03c34f7c9c90fb84dfb1bc5b") {
//         Err(e) => {
//             println!("API Key error: {}", e);
//             return;
//         },
//         Ok(v) => v,
//     };
//     let client = match DeepSeekClientBuilder::new(api_key).build() {
//         Err(e) => {
//             println!("Client build error: {}", e);
//             return;
//         },
//         Ok(v) => v,
//     };
    
//     let parameters: SchemaObject = match serde_json::from_str(
//         r#"{
//         "type": "object",
//         "properties": {
//             "price": {
//                 "type": "float",
//                 "description": "The current gold price"
//             },
//             "meanline_5day_gold_price": {
//                 "type": "float",
//                 "description": "The 5 day meanline gold price"
//             }
//         },
//         "required": ["price"]
//         }"#
//     ) {
//         Err(e) => {
//             println!("Parameters schema error: {}", e);
//             return;
//         },
//         Ok(v) => v,
//     };
//     let tool_object = ToolObject {
//         tool_type: ToolType::Function,
//         function: Function {
//             description: "Provide gold investment advice.".to_string(),
//             name: "gold_investment_advice".to_string(),
//             parameters
//         }
//     };

//     let tool_objects = vec![tool_object];
//     let mut messages = vec![MessageRequest::user("Now gold price is 1800.5 USD per ounce. Provide me some investment advice about gold.")];
//     let resp = match CompletionsRequestBuilder::new(&messages)
//         .tools(&tool_objects)
//         .do_request(&client)
//         .await {
//             Err(e) => {
//                 println!("Completions request error: {}", e);
//                 return;
//             },
//             Ok(v) => v.must_response(),
//         };

//     let mut id = String::new();
//     if resp.choices[0].finish_reason == FinishReason::ToolCalls {
//         if let Some(msg) = &resp.choices[0].message {
//             if let Some(tool) = &msg.tool_calls {
//                 id = tool[0].id.clone();
//                 println!("Function id: {}", id);
//                 println!("Function name: {}", tool[0].function.name);
//                 println!("Function parameters: {:?}", tool[0].function.arguments);
//             }
//             messages.push(MessageRequest::Assistant(msg.clone()));
//         }
//     }

//     messages.push(MessageRequest::Tool(ToolMessageRequest::new("1795.3", &id)));
//     let resp = match CompletionsRequestBuilder::new(&messages)
//         .tools(&tool_objects)
//         .do_request(&client)
//         .await {
//             Err(e) => {
//                 println!("Completions request error: {}", e);
//                 return;
//             },
//             Ok(v) => v.must_response()
//         };

//     println!("Final response: {}", resp.choices[0].message.as_ref().unwrap().content);
// }
