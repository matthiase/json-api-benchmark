use crate::error::ApplicationError;
use crate::species::Species;
use crate::AppData;
use actix_web::{web, HttpResponse};

pub async fn index(context: web::Data<AppData>) -> Result<HttpResponse, ApplicationError> {
    let species = fetch_species(&context.db).await?;
    Ok(HttpResponse::Ok().json(species))
}

async fn fetch_species(pool: &sqlx::postgres::PgPool) -> Result<Vec<Species>, sqlx::Error> {
    let rows = sqlx::query!("SELECT * FROM species LIMIT 3")
        .fetch_all(pool)
        .await?;

    let results = rows
        .iter()
        .map(|row| Species {
            id: Some(i64::from(row.id)),
            name: row.name.clone(),
        })
        .collect();

    Ok(results)
}
