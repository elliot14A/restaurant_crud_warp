use crate::{data::CreateRecipeRequest, db, error::Error::*, DBPool, Result};

use serde::Deserialize;
use warp::{http::StatusCode, reject, reply::json, Reply};

pub async fn health_handler(db_pool: DBPool) -> Result<impl Reply> {
    let db = db::get_db_con(&db_pool)
        .await
        .map_err(|e| reject::custom(e))?;
    db.execute("SELECT 1", &[])
        .await
        .map_err(|e| reject::custom(DBQueryError(e)))?;
    Ok(StatusCode::OK)
}

pub async fn create_recipe_handler(
    body: CreateRecipeRequest,
    db_pool: DBPool,
) -> Result<impl Reply> {
    Ok(json(
        &db::create_recipe(&db_pool, body)
            .await
            .map_err(|e| reject::custom(e))?,
    ))
}

#[derive(Deserialize)]
pub struct SearchQuery {
    search : Option<String>
}

pub async fn fetch_recipes_handler(query: SearchQuery,db_pool: DBPool, ) -> Result<impl Reply> {
     Ok(json(
            &db::fetch_recipes_all(&db_pool, query.search)
                .await
                .map_err(|e| reject::custom(e))?))
     
}
