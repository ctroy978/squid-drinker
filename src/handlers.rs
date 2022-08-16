use super::DbPool;


use serde::{Deserialize, Serialize};
use actix_web::{get, post, web, HttpResponse, Result, Error, Responder};
use diesel::prelude::*;

use crate::models::{Recipe, NewRecipe, Ingredient, NewIngredient, Unit, NewUnit, Label, NewLabel, Qty, NewQty};


const MAX_SIZE: usize = 262_144; // max payload size is 256k


#[derive(Debug, Serialize, Deserialize)]
pub struct FullRecipe{
    title: String,
    rank: i32,
    directions: String,
    ingredients: Vec<String>,
}


type DbError = Box<dyn std::error::Error + Send + Sync>;

#[get("/drink/{search_for}")]
async fn show(pool: web::Data<DbPool>, search_for: web::Path<String>) -> Result<HttpResponse, Error>{

    let the_recipe = web::block(move ||{
        let conn = pool.get()?;
        find_recipe(&conn, &search_for)
    }).await?
    .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(the_recipe))
}

#[derive(Deserialize)]
struct Ing{
    label: String,
    unit: String,
    qty: String,
}

#[derive(Deserialize)]
struct Info{
    title: String,
    rank: String,
    directions: String,
    add_ingredient: Vec<Ing>,

}

#[post("/build")]
pub async fn build(pool: web::Data<DbPool>,info: web::Json<Info>)-> Result<HttpResponse, Error>{

    
    let the_recipe = web::block(move ||{
        let conn = pool.get()?;
        post_recipe(&conn, info)
    }).await?
    .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(the_recipe))
}

fn find_recipe(conn: &PgConnection, search_for: &str) -> Result<FullRecipe, DbError>{
    use crate::schema::recipes::dsl::*;
   

    let the_recipe = recipes.filter(title.eq(search_for))
        .first::<Recipe>(conn)?;
 
    let the_ingredients = Ingredient::belonging_to(&the_recipe)
        .limit(10)
        .load::<Ingredient>(conn)?;

    let mut list_ingredients = Vec::new();

    for ingredient in the_ingredients{
        let the_label = Label::belonging_to(&ingredient)
            .first::<Label>(conn)?;
        let the_unit = Unit::belonging_to(&ingredient)
            .first::<Unit>(conn)?;
        let the_qty = Qty::belonging_to(&ingredient)
            .first::<Qty>(conn)?;
        
        let this_ingredient = format!("{} {} {}", 
        the_qty.quantity,
        the_unit.unit_description,
        the_label.label,
        );
        list_ingredients.push(this_ingredient);
        };

        let full_recipe = FullRecipe{
            title: the_recipe.title.to_owned(),
            rank: the_recipe.rank,
            directions: the_recipe.directions.to_owned(),
            ingredients: list_ingredients,
        };
    Ok(full_recipe)

}
    

fn post_recipe(conn: &PgConnection, info: web::Json<Info>)->Result<Recipe, DbError>{
    use crate::schema::recipes::dsl::*;
    use crate::schema::ingredients::dsl::*;
    use crate::schema::labels::dsl::*;
    use crate::schema::qtys::dsl::*;
    use crate::schema::units::dsl::*;

    //recipe
    let new_recipe = NewRecipe {
        title: &info.title.as_str(),
        rank: &info.rank.parse::<i32>().unwrap(),
        directions: &info.directions.as_str(),
    };

    let this_recipe: Recipe = diesel::insert_into(recipes)
        .values(&new_recipe)
        .get_result(conn)
        .expect("error saving new recipe");

    //ingredients
    for ingredient in &info.add_ingredient{

        //ingredient
        let new_ingredient = NewIngredient{
            recipe_id: &this_recipe.id,
        };
        let this_ingredient: Ingredient = diesel::insert_into(ingredients)
        .values(&new_ingredient)
        .get_result(conn)
        .expect("error saving new ingredient");

        //label
        let new_label = NewLabel{
            ingredient_id: &this_ingredient.id,
            label: &ingredient.label.as_str(),
        };
        let _this_label: Label = diesel::insert_into(labels)
        .values(&new_label)
        .get_result(conn)
        .expect("error saving new label");

        //qty
        let new_qty = NewQty{
            ingredient_id: &this_ingredient.id,
            quantity: &ingredient.qty.as_str(),
        };
        let _this_qty: Qty = diesel::insert_into(qtys)
        .values(&new_qty)
        .get_result(conn)
        .expect("error saving new qty");

        //unit
        let new_unit = NewUnit{
            ingredient_id: &this_ingredient.id,
            unit_description: &ingredient.unit.as_str(),
        };
        let _this_qty: Label = diesel::insert_into(units)
        .values(&new_unit)
        .get_result(conn)
        .expect("error saving new qty");

    };
    Ok(this_recipe)


}