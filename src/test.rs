use std::time::Instant;
use crate::{check_drink_is_valid, check_ingredient_is_valid, get_drinks, get_ingredients};

// Functional Ingredients Tests
#[tokio::test]
async fn test_1_alcoholic_ingredient( ){
    let ingredient_name = "vodka";
    let result = check_ingredient_is_valid(ingredient_name, true).await;

    assert_eq!(result, true);
}

#[tokio::test]
async fn test_2_non_alcoholic_ingredient(){
    let ingredient_name = "lemon";
    let result = check_ingredient_is_valid(ingredient_name, false).await;

    assert_eq!(result, true);
}

#[tokio::test]
async fn test_3_partial_ingredient(){
    let partial_list =  match get_ingredients("lem").await {
        Ok(ingredients) => ingredients,
        Err(_) => Vec::new(),
    };

    let full_list =  match get_ingredients("lemon").await {
        Ok(ingredients) => ingredients,
        Err(_) => Vec::new(),
    };

    let mut result = false;
    for item in partial_list {
        if  item.idIngredient == full_list[0].idIngredient  {
            result = item == full_list[0];
        }
    }

    assert_eq!(result, true);
}


#[tokio::test]
async fn test_4_non_existent_ingredient(){
    let res =  match get_ingredients("rdj").await {
        Ok(_) => String::new(),
        Err(err) => err ,
    };

    let mut result = false;
    if  res == "Ingredients is empty" {
        result= true;
    }

    assert_eq!(result, true);
}

#[tokio::test]
async fn test_5_ingredient_case_sensitivity(){
    let ingredients_name_1 = "Vodka";
    let ingredients_name_2 = "vodka";
    let ingredients_name_3 = "vOdKa";

    let ingredients_1 =  match get_ingredients(ingredients_name_1).await {
        Ok(ingredients) => ingredients,
        Err(_) => Vec::new(),
    };

    let ingredients_2 =  match get_ingredients(ingredients_name_2).await {
        Ok(ingredients) => ingredients,
        Err(_) => Vec::new(),
    };

    let ingredients_3 =  match get_ingredients(ingredients_name_3).await {
        Ok(ingredients) => ingredients,
        Err(_) => Vec::new(),
    };

    let mut result = ingredients_1 == ingredients_2;
    if result {
        result = ingredients_3 == ingredients_2;
    }

    assert_eq!(result, true, "Results for {}, {} and {} were not identical", ingredients_name_1, ingredients_name_2, ingredients_name_3);
}

// Non-Functional Ingredients Tests
#[tokio::test]
async fn test_6_ingredient_response_time(){
    let start_time = Instant::now();
    let ingredients =  match get_ingredients("vodka").await {
        Ok(ingredients) => ingredients,
        Err(_) => Vec::new(),
    };

    if ingredients.len() < 1 {
        return
    }

    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;

    assert_eq!(elapsed_time.as_millis() < 1000, true, "Response took: {}", elapsed_time.as_millis());
}

#[tokio::test]
async fn test_7_ingredient_fuzzy_search(){
    let ingredient_name = "vdoka";
    let ingredients =  match get_ingredients(ingredient_name).await {
        Ok(ingredients) => ingredients,
        Err(_) => Vec::new(),
    };

    let mut result = false;
    for ingredient in ingredients {
        if ingredient.strIngredient.to_ascii_lowercase() == ingredient_name.to_ascii_lowercase(){
            result = true;
            break;
        }
    }

    assert_eq!(result, true, "No results found for {}", ingredient_name);
}
// Functional Drinks Tests
#[tokio::test]
async fn test_8_drink(){
    let drink_name = "margarita";
    let result = check_drink_is_valid(drink_name).await;

    assert_eq!(result, true, "Serching a specific drink: {drink_name}");
}

#[tokio::test]
async fn test_9_partial_drink(){
    let full_drinks =  match get_drinks("moj").await {
        Ok(drinks) => drinks,
        Err(_) => Vec::new(),
    };

    let partial_drinks =  match get_drinks("mojito").await {
        Ok(drinks) => drinks,
        Err(_) => Vec::new(),
    };

    let mut result = false;

    for item in partial_drinks {
        if  item.idDrink == full_drinks[0].idDrink  {
            result = item == full_drinks[0];
        }
    }

    assert_eq!(result, true);
}

#[tokio::test]
async fn test_10_non_existent_drink(){
    let res =  match get_drinks("rdj").await {
        Ok(_) => String::new(),
        Err(err) => err ,
    };

    let mut result = false;
    if  res == "Drinks is empty" {
        result= true;
    }

    assert_eq!(result, true);
}

#[tokio::test]
async fn test_11_drinks_case_sensitivity(){
    let drink_name_1 = "Mojito";
    let drink_name_2 = "mojito";
    let drink_name_3 = "mOjIto";

    let drinks_1 =  match get_drinks(drink_name_1).await {
        Ok(drinks) => drinks,
        Err(_) => Vec::new(),
    };

    let drinks_2 =  match get_drinks(drink_name_2).await {
        Ok(drinks) => drinks,
        Err(_) => Vec::new(),
    };

    let drinks_3 =  match get_drinks(drink_name_3).await {
        Ok(drinks) => drinks,
        Err(_) => Vec::new(),
    };

    let mut result = drinks_1 == drinks_2;
    if result {
        result = drinks_3 == drinks_2;
    }

    assert_eq!(result, true, "Results for {}, {} and {} were not identical", drink_name_1, drink_name_2, drink_name_3);
}

// Non-Functional Drinks Tests
#[tokio::test]
async fn test_12_drinks_response_time(){
    let start_time = Instant::now();
    let drinks =  match get_drinks("Mojito").await {
        Ok(drinks) => drinks,
        Err(_) => Vec::new(),
    };

    if drinks.len() < 1 {
        return
    }

    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;

    assert_eq!(elapsed_time.as_millis() < 1000, true, "Response took: {}", elapsed_time.as_millis());
}

#[tokio::test]
async fn test_13_drink_fuzzy_search(){
    let drink_name = "magarita";
    let drinks =  match get_drinks(drink_name).await {
        Ok(drinks) => drinks,
        Err(_) => Vec::new(),
    };

    let mut result = false;
    for drink in drinks {
        let name = match drink.strDrink.clone(){
            Some(name) => name,
            None => String::new(),
        };

        if name.to_ascii_lowercase() == drink_name.to_ascii_lowercase(){
            result = true;
            break;
        }
    }

    assert_eq!(result, true, "No results found for {}", drink_name);
}