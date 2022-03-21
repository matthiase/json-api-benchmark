use crate::error::ApplicationError;
use crate::species::Species;
use crate::AppData;
use actix_web::{web, HttpResponse};
use rand::Rng;

const SPECIES_MAX_ROWS: i32 = 37;

pub async fn index(context: web::Data<AppData>) -> Result<HttpResponse, ApplicationError> {
    let species = fetch_species(&context.db).await?;
    Ok(HttpResponse::Ok().json(species))
}

async fn fetch_species(pool: &sqlx::postgres::PgPool) -> Result<Vec<Species>, sqlx::Error> {
    let mut rng = rand::thread_rng();
    let random_id: i32 = rng.gen_range(1..=SPECIES_MAX_ROWS);
    let rows = sqlx::query!("SELECT * FROM species WHERE id > $1", random_id)
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
