use super::DbPool;
use serde::{Deserialize, Serialize};
use actix_web::{get, post, web, HttpResponse, Responder, Error};
use diesel::prelude::*;

use crate::models::{Recipe, Ingredient, Unit, Label, Qty};


#[derive(Debug, Serialize, Deserialize)]
pub struct FullRecipe{
    title: String,
    rank: i32,
    directions: String,
    ingredients: Vec<String>,
}




type DbError = Box<dyn std::error::Error + Send + Sync>;

#[get("/drink/{title}")]
async fn show(pool: web::Data<DbPool>, title: web::Path<String>) -> Result<HttpResponse, Error>{

    let the_recipe = web::block(move ||{
        let conn = pool.get()?;
        find_recipe(&conn, &title)
    }).await?
    .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(the_recipe))
}

fn find_recipe(conn: &PgConnection, title: &str) -> Result<FullRecipe, DbError>{
    use crate::schema::recipes::dsl::*;
   

    let the_recipe = recipes.filter(title.eq(title))
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
        the_unit.description,
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
    
    
