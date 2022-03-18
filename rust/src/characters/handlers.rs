use crate::characters::Character;
use actix_web::{Error, HttpResponse};

pub async fn index() -> Result<HttpResponse, Error> {
    let character = Character {
        id: Some(i64::from(1234)),
        name: Some(String::from("Luke Skywalker")),
    };
    Ok(HttpResponse::Ok().json(character))
}
