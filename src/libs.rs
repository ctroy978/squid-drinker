


use serde::{Deserialize, Serialize};
use actix_web::{ Result};
use diesel::prelude::*;


use crate::DbPool;
use crate::models::{Recipe};


#[derive(Debug, Serialize, Deserialize)]
pub struct FullRecipe{
    title: String,
    rank: i32,
    booz: String,
    directions: String,
    ingredients: Vec<String>,
}



type DbError = Box<dyn std::error::Error + Send + Sync>;



pub fn find_booz(conn: &PgConnection, search_for: &str) -> Result<Vec<Recipe>, DbError>{
    use crate::schema::recipes::dsl::*;
   

    let the_recipe = recipes.filter(booz.eq(search_for))
        .load::<Recipe>(conn)?;
        
    Ok(the_recipe)
}