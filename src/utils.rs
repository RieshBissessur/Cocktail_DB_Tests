use std::{collections::HashSet, error::Error, fs};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use csv::Reader;

// Public structs
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[allow(non_snake_case)]
pub struct Drink {
    pub idDrink: Option<String>,
    pub strDrink: Option<String>,
    pub strIngredient: Option<String>,
    pub strTags: Option<String>,
    pub strCategory: Option<String>,
    pub strIBA: Option<String>,
    pub strAlcoholic: Option<String>,
    pub strGlass: Option<String>,
    pub strInstructions: Option<String>,
    pub strIngredient1: Option<String>,
    pub strMeasure1: Option<String>,
    pub strCreativeCommonsConfirmed: Option<String>,
    pub dateModified: Option<String>
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[allow(non_snake_case)]
pub struct DrinkData {
    pub strDrink: Option<String>,
    pub strIngredient: Option<String>,
    pub strInstructions: Option<String>,
    pub strIngredient1: Option<String>,
    pub strIngredient2: Option<String>,
    pub strIngredient3: Option<String>,
    pub strIngredient4: Option<String>,
    pub strIngredient5: Option<String>,
    pub strIngredient6: Option<String>,
    pub strIngredient7: Option<String>,
    pub strIngredient8: Option<String>,
    pub strIngredient9: Option<String>,
    pub strIngredient10: Option<String>,
    pub strIngredient11: Option<String>,
    pub strIngredient12: Option<String>,
    pub strIngredient13: Option<String>,
    pub strIngredient14: Option<String>,
    pub strIngredient15: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Drinks {
    pub drinks: Option<Vec<Drink>>
}

#[derive(Debug, Deserialize, Serialize, Clone,PartialEq)]
#[allow(non_snake_case)]
pub struct Ingredient {
    pub idIngredient: String,
    pub strIngredient: String,
    pub strDescription: Option<String>,
    pub strType: Option<String>,
    pub strAlcohol: Option<String>,
    pub strABV: Option<String>
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Ingredients {
    pub ingredients: Option<Vec<Ingredient>>
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
#[allow(non_snake_case)]
pub struct ReportDetails {
    pub has_result: bool,
    pub missing_fields: u8,
}

// Public functions
pub fn handle_cocktail_testing_data() -> Result<(), Box<dyn Error>>{
    let mut reader = Reader::from_path("./data/all_drinks.csv")?;
    let mut drinks = HashSet::new();
    let mut ingredients = HashSet::new();
    for record in reader.deserialize() {
        let drink: DrinkData = record?;
        match drink.strDrink {
            Some(drink_name) => drinks.insert(drink_name),
            None => false,
        };

        match drink.strIngredient {
            Some(ingredient) => ingredients.insert(ingredient),
            None => false,
        };

        match drink.strIngredient1 {
            Some(ingredient) => ingredients.insert(ingredient),
            None => false,
        };

        match drink.strIngredient2 {
            Some(ingredient) => ingredients.insert(ingredient),
            None => false,
        };

        match drink.strIngredient3 {
            Some(ingredient) => ingredients.insert(ingredient),
            None => false,
        };

        match drink.strIngredient4 {
            Some(ingredient) => ingredients.insert(ingredient),
            None => false,
        };

        match drink.strIngredient5 {
            Some(ingredient) => ingredients.insert(ingredient),
            None => false,
        };

        match drink.strIngredient6 {
            Some(ingredient) => ingredients.insert(ingredient),
            None => false,
        };

        match drink.strIngredient7 {
            Some(ingredient) => ingredients.insert(ingredient),
            None => false,
        };

        match drink.strIngredient8 {
            Some(ingredient) => ingredients.insert(ingredient),
            None => false,
        };

        match drink.strIngredient9 {
            Some(ingredient) => ingredients.insert(ingredient),
            None => false,
        };

        match drink.strIngredient10 {
            Some(ingredient) => ingredients.insert(ingredient),
            None => false,
        };

        match drink.strIngredient11 {
            Some(ingredient) => ingredients.insert(ingredient),
            None => false,
        };

        match drink.strIngredient12 {
            Some(ingredient) => ingredients.insert(ingredient),
            None => false,
        };

        match drink.strIngredient13 {
            Some(ingredient) => ingredients.insert(ingredient),
            None => false,
        };

        match drink.strIngredient14 {
            Some(ingredient) => ingredients.insert(ingredient),
            None => false,
        };

        match drink.strIngredient15 {
            Some(ingredient) => ingredients.insert(ingredient),
            None => false,
        };        
    }

    let ingredients_str = match serde_json::to_string(&ingredients) {
        Ok(data) => data,
        Err(_) => String::new()
    };

    let drink_str = match serde_json::to_string(&drinks) {
        Ok(data) => data,
        Err(_) => String::new()
    };

    let _ = fs::write("./data/drinks.txt", drink_str);
    let _ = fs::write("./data/ingredients.txt", ingredients_str);
    return Ok(());
    
}

pub async fn get_ingredients(ingredient: &str) -> Result<Vec<Ingredient>, String>{
    let url = format!("https://www.thecocktaildb.com/api/json/v1/1/search.php?i={}",ingredient);
    let response = match handle_get_request(url).await {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    let response = match serde_json::from_str::<Ingredients>(&response) {
        Ok(ingredients) => ingredients,
        Err(_) => return Err("Can not deserialize response".to_string()),
    };

    match response.ingredients {
        Some(ingredients) => return Ok(ingredients),
        None => return Err("Ingredients is empty".to_string()),
    };
}

pub async fn get_ingredient(ingredient_name: &str) -> Result<Ingredient, String>{
    let ingredients =  match get_ingredients(ingredient_name).await {
        Ok(ingredients) => ingredients,
        Err(_) => Vec::new(),
    };

    for ingredient in ingredients {
        if ingredient_name.to_ascii_lowercase() == ingredient.strIngredient.to_ascii_lowercase(){
            return Ok(ingredient);
        }
    }

    return Err(format!("Unable to find ingredient"));
}

pub async fn get_drinks(recipe: &str) -> Result<Vec<Drink>, String>{
    let url = format!("https://www.thecocktaildb.com/api/json/v1/1/search.php?s={}",recipe);
    let response = match handle_get_request(url).await {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    let response = match serde_json::from_str::<Drinks>(&response) {
        Ok(drinks) => drinks,
        Err(_) => return Err("Can not deserialize response".to_string()),
    };

    match response.drinks {
        Some(drinks) => return Ok(drinks),
        None => return Err("Drinks is empty".to_string()),
    };
}

pub async fn get_drink(drink_name: &str) -> Result<Drink, String>{
    let drinks =  match get_drinks(drink_name).await {
        Ok(drinks) => drinks,
        Err(_) => Vec::new(),
    };

    for drink in drinks {
        let name = match drink.strDrink.clone(){
            Some(name) => name,
            None => return Err(format!("Unable to find ingredient")),
        };
        
        if drink_name.to_ascii_lowercase() == name.to_ascii_lowercase(){
            return Ok(drink);
        }
    }

    return Err(format!("Unable to find ingredient"));
}

pub async fn check_ingredient_is_valid(ingredient_name: &str, alcoholic: bool) -> bool{
    let ingredients =  match get_ingredients(ingredient_name).await {
        Ok(ingredients) => ingredients,
        Err(_) => Vec::new(),
    };

    for ingredient in ingredients {
        if ingredient_name.to_ascii_lowercase() == ingredient.strIngredient.to_ascii_lowercase(){
            if ingredient.strABV.is_some() && alcoholic || ingredient.strABV.is_none() && !alcoholic{
                if ingredient.strDescription.is_none() {
                    return false;
                }
                
                if ingredient.strType.is_none() {
                    return false;
                }

                if ingredient.strAlcohol.is_none() {
                    return false;
                }

                return true;

            }
            
        }
    }

    return false;
}

pub async fn check_drink_is_valid(drink_name: &str) -> bool{
    let drinks =  match get_drinks(drink_name).await {
        Ok(drinks) => drinks,
        Err(_) => Vec::new(),
    };

    for drink in drinks {
        let name = match drink.strDrink.clone(){
            Some(name) => name,
            None => String::new(),
        };

        if name.to_ascii_lowercase() == drink_name.to_ascii_lowercase(){
            return true;
        }
    }

    return false;
}

pub fn read_ingredients_hash_set() -> HashSet<String>{
    let ingredients_path = format!("./data/ingredients.txt");
    match read_hash_set(&ingredients_path) {
        Ok(data) => return data,
        Err(_) => return HashSet::new()
    };
}

pub fn read_drinks_hash_set()-> HashSet<String>{
    let drinks_path = format!("./data/drinks.txt");
    match read_hash_set(&drinks_path) {
        Ok(data) => return data,
        Err(_) =>  return HashSet::new()
    };
}

// Private functions
fn read_hash_set(path: &str) -> Result<HashSet<String>, String>{
    let data = match fs::read_to_string(&path) {
        Ok(data) => data,
        Err(err) =>  return Err(err.to_string())
    };

   match serde_json::from_str(&data) {
        Ok(data) => return Ok(data),
        Err(_) => return Err("Failed to deserialize contract".to_string())
    };
}

async fn handle_get_request(url: String) -> Result<String, String> {
    let client = Client::new();
    let response = client.get(&url).send().await;
    match response {
        Ok(response) => {
            if !response.status().is_success() {
                return Err(String::new());
            }
            let body = response.text().await;
            match body {
                Ok(body_data) => return Ok(body_data),
                Err(_) => return Err(String::new()),
            }
        }
        Err(_) => return Err(String::new()),
    }
}