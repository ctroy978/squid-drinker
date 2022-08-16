use serde::{Deserialize, Serialize};
use crate::schema::{recipes, ingredients, labels, units, qtys};

#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, PartialEq)]
#[table_name = "recipes"]
pub struct Recipe{
    pub id: i32,
    pub title: String,
    pub rank: i32,
    pub directions: String,
}

#[derive(Insertable)]
#[table_name = "recipes"]
pub struct NewRecipe<'a>{
    pub title: &'a str,
    pub rank: &'a i32,
    pub directions: &'a str,
}

#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, Associations, PartialEq)]
#[belongs_to(Recipe)]
#[table_name = "ingredients"]
pub struct Ingredient{
    pub id: i32,
    pub recipe_id: i32,
}

#[derive(Insertable)]
#[table_name = "ingredients"]
pub struct NewIngredient<'a>{
    pub recipe_id: &'a i32,
}



#[derive(Serialize, Deserialize, Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Ingredient)]
#[table_name = "qtys"]
pub struct Qty{
    pub id: i32,
    pub ingredient_id: i32,
    pub quantity: String,
}

#[derive(Insertable)]
#[table_name = "qtys"]
pub struct NewQty<'a>{
    pub ingredient_id: &'a i32,
    pub quantity: &'a str,
}


#[derive(Serialize, Deserialize, Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Ingredient)]
#[table_name = "labels"]
pub struct Label{
    pub id: i32,
    pub ingredient_id: i32,
    pub label: String,
}

#[derive(Insertable)]
#[table_name = "labels"]
pub struct NewLabel<'a>{
    pub ingredient_id: &'a i32,
    pub label: &'a str,
}

#[derive(Serialize, Deserialize, Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Ingredient)]
#[table_name = "units"]
pub struct Unit{
    pub id: i32,
    pub ingredient_id: i32,
    pub unit_description: String,
}

#[derive(Insertable)]
#[table_name = "units"]
pub struct NewUnit<'a>{
    pub ingredient_id: &'a i32,
    pub unit_description: &'a str,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct DrinkPayload{
    pub new_recipe: String,
}