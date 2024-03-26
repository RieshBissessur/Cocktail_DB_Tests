This is dependant on RUST being installed

Rust can be installed by following these instructions:

	https://www.rust-lang.org/tools/install

Or on Mac OS X using this curl command:

	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh 

<br>

# Tests

- ## Compile and run the tests
  - In the project directory run the following commands to compile and run the rust tests.
 
  - ```  cargo test ```

  <br>

### Functional Ingredients Tests
1. Perform a test with known ingredient names that is alcoholic
    - Should return a 200 status for this API call
    - Should return all ingredient fields with a value for abv
    - eg. Vodka or Brandy
    - Test will fail if any ingredient field is null as types are strings and not nullable

2. Perform a test with known ingredient names that is non-alcoholic
    - Should return a 200 status for this API call
    - Should return all ingredient fields with null abv
    - eg. Lemon or Cream
    - Test will fail if any ingredient field is null as types are strings and not nullable

3. Perform a test with the first few letters of an ingredient
    - Should return a 200 status for this API call
    - Should return the same ingredient fields for the full searched ingredient 
    - eg. Vod or Cr
    - Test will fail if any ingredient field is null as types are strings and not nullable

4. Perform a test for an ingredient that does not exist
    - Should return null
    - eg. rdj
    - Test will fail if any ingredient field is null as types are strings and not nullable

5. Perform a test that ensure the same response for case no sensitivity in ingredients names
    - Should return a 200 status for this API call
    - Should the same response irrespective of the case
    - eg. Vodka and vodka and vOdKa

### Non-Functional Ingredients Tests
6. Test the response time for an ingredient search
    - Should return a 200 status for this API call
    - Should have response time of less than a second

7. Test for fuzzy searching for a an ingredient search
    - Should return a 200 status for this API call
    - Should give the correct response for an ingredient name with a spelling error
    - eg. vdoka

### Functional Drink Tests
8. Perform a test with known drink names
    - Should return a 200 status for this API call
    - Should return a valid drinks schema
    - eg. Negroni or Mojito

9. Perform a test with the first few letters of an cocktail
    - Should return a 200 status for this API call
    - Should return the same drink schema for the full searched ingredient 
    - eg. Ne or Moj

10. Perform a test for a cocktail that does not exist
    - Should return null
    - eg. rdj

11. Perform a test that ensure the same response for case insensitivity of drinks
    - Should return a 200 status for this API call
    - Should the same response irrespective of the case
    - eg. Mojito and mojito and mOjItO

### Non-Functional Drink Tests
12. Test the response time for a search a drink
    - Should return a 200 status for this API call
    - Should have response time of less than a second

13. Test for fuzzy searching for a an drink search
    - Should return a 200 status for this API call
    - Should give the correct response for an drink name with a spelling error
    - eg. magarita

<br>

# Test Reports

  - ## Compile and run the report generation
  - In the project directory run the following commands to compile and run the console application.
 
  - ```  cargo run ingredient_report ```
  - Or
  - ```  cargo run drink_report ```

  <br>

- A CSV file containing historical data from Cocktail DB with 546 drinks was used. A hash map was employed to extract all the unique ingredients from these drinks. These results were written to files as JSON objects in the results folder.

- The files drinks.txt and ingredients.txt are Json serialized Hash Sets and can be modified by adding or removing drink and ingredient files to their respective files.

- These were then utilized to generate reports for searching by ingredient name and by drink name. The resulting files generated in the results folder are hash maps where the key-value pair consists of the ingredient or drink name as the key, and a boolean value indicating whether the API returned the correct data for the searched name and if any fields were missing. For drinks, all missing fields will be set to zero since the provided schema allows null values for all fields.
