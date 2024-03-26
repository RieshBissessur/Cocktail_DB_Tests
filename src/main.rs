mod utils;
mod test;

use std::{collections::HashMap, env, fs};
use utils::*;

#[tokio::main]
async fn main() {
    if !fs::metadata("./data").is_ok() {
        fs::create_dir("./data").expect("Failed to create data directory");
    }

    if !fs::metadata("./results").is_ok() {
        fs::create_dir("./results").expect("Failed to create results directory");
    }

    let args: Vec<String> = env::args().collect();
     if args.len() > 1 {
        let user_input = &args[1];
        if user_input == "ingredient_report" {
            let _ = generate_ingredients_test_report().await;
        } else if user_input == "drink_report" {
            let _ = generate_drinks_test_report().await;
        }else if user_input == "update_data"{
            let _ = handle_cocktail_testing_data();
        }
    }
}


async fn generate_ingredients_test_report(){
    let ingredient_data = read_ingredients_hash_set();  
    let mut report: HashMap<String, ReportDetails> = HashMap::new();
    let mut count  = 0;
    for ingredient in ingredient_data.clone() {
        count += 1;
        println!("{}/{}", count, ingredient_data.len());
        let result = match get_ingredient(ingredient.as_str()).await{
            Ok(result) => result,
            Err(_) => {
                report.insert(ingredient.to_string(), ReportDetails{ has_result: false, missing_fields: 0});
                continue;
            },
        };

        let mut missing_fields = 0;

        if result.strDescription.is_none() {
            missing_fields += 1;
        }

        if result.strType.is_none() {
            missing_fields += 1;
        }

        if result.strAlcohol.is_none() {
            missing_fields += 1;
        }

        report.insert(ingredient.to_string(), ReportDetails{ has_result: true, missing_fields});
    }

    let ingredient_str = match serde_json::to_string(&report) {
        Ok(data) => data,
        Err(_) => String::new()
    };

    let _ = fs::write("./results/ingredient_report.txt", ingredient_str);
}

async fn generate_drinks_test_report(){
    let drinks_data = read_drinks_hash_set();
    let mut report: HashMap<String, ReportDetails> = HashMap::new();
    let mut count  = 0;
    for drink in drinks_data.clone() {
        count += 1;
        println!("{}/{}", count, drinks_data.len());
        match get_drink(drink.as_str()).await{
            Ok(result) => result,
            Err(_) => {
                report.insert(drink.to_string(), ReportDetails{ has_result: false, missing_fields: 0});
                continue;
            },
        };

        report.insert(drink.to_string(), ReportDetails{ has_result: true, missing_fields:0});
    }

    let ingredient_str = match serde_json::to_string(&report) {
        Ok(data) => data,
        Err(_) => String::new()
    };

    let _ = fs::write("./results/drinks_report.txt", ingredient_str);
}