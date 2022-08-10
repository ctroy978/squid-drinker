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

#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, Associations, PartialEq)]
#[belongs_to(Recipe)]
#[table_name(ingredients)]
pub struct Ingredient{
    pub id: i32,
    pub recipe_id: i32,
}

#[derive(Serialize, Deserialize, Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Ingredient)]
#[table_name = "qtys"]
pub struct Qty{
    pub id: i32,
    pub ingredient_id: i32,
    pub quantity: String,
}

#[derive(Serialize, Deserialize, Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Ingredient)]
#[table_name = "labels"]
pub struct Label{
    pub id: i32,
    pub ingredient_id: i32,
    pub label: String,
}

#[derive(Serialize, Deserialize, Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Ingredient)]
#[table_name = "units"]
pub struct Unit{
    pub id: i32,
    pub ingredient_id: i32,
    pub description: String,
}