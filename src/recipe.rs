use actix_web::{get, HttpResponse};
use actix_web::web::Path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SmallRecipe {
    pub id: String,
    pub title: String,
    pub time: String,
    pub number_of_steps: u32,
    pub number_of_ingredients: u32,
    pub creator_user_id: String,
    pub recipe_type: String,
    pub image: String,
    pub color: String,
}

impl SmallRecipe {
    pub fn new(
        id: String,
        title: String,
        time: String,
        number_of_steps: u32,
        number_of_ingredients: u32,
        creator_user_id: String,
        recipe_type: String,
        image: String,
        color: String,
    ) -> Self {
        Self {
            id,
            title,
            time,
            number_of_steps,
            number_of_ingredients,
            creator_user_id,
            recipe_type,
            image,
            color,
        }
    }
}

fn db_recipes() -> Vec<SmallRecipe> {
    vec![
        SmallRecipe::new(
            "1".to_owned(),
            "Tomatsuppe".to_owned(),
            "20 minutter".to_owned(),
            3,
            3,
            "justMarlen".to_owned(),
            "Suppe".to_owned(),
            "tomatoImage".to_owned(),
            "#F4D0E9".to_owned(),
        )
    ]
}

#[get("/recipes")]
pub async fn getAll() -> HttpResponse {
    let recipes = db_recipes();

    HttpResponse::Ok()
        //.content_type(APPLICATION_JSON)
        .json(recipes)
}