mod GoldMarket;
mod GoldProductionDepartment;
mod GoldSupplyDepartment;

use anyhow::Result;
use deepseek_api::response::FinishReason;
use schemars::schema::SchemaObject;
use std::{collections::HashMap, str::FromStr};
use deepseek_api::request::{Function, ToolMessageRequest, ToolObject, ToolType, MessageRequest};
use deepseek_api::{DeepSeekClientBuilder, CompletionsRequestBuilder, RequestBuilder};

use GoldMarket::Market;
use GoldProductionDepartment::MiningDepartment;
use GoldSupplyDepartment::RoyalBank;

const INIT_GOLD_PRICE: f64 = 10.0;
const INIT_R_PRICE: f64 = 10.0;
const INIT_GOLD_REVERSE: u64 = 10;
const INIT_R_REVERSE: u64 = 10;

fn main() {
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
    init_reverse.insert("gold_reverse", INIT_GOLD_REVERSE);
    init_reverse.insert("currency_R_reserves", INIT_R_REVERSE);

    let market = Market::new(init_price, init_reverse, client);
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
//             "location": {
//                 "type": "string",
//                 "description": "The location to get the weather for"
//             },
//             "unit": {
//                 "type": "string",
//                 "enum": ["celsius", "fahrenheit"],
//                 "description": "The unit of temperature"
//             }
//         },
//         "required": ["location"]
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
//             description: "Get weather of an location, the user shoud supply a location first".to_string(),
//             name: "get_waether".to_string(),
//             parameters
//         }
//     };

//     let tool_objects = vec![tool_object];
//     let mut messages = vec![MessageRequest::user("How's the weather in Hangzhou?")];
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

//     messages.push(MessageRequest::Tool(ToolMessageRequest::new("24°C", &id)));
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
