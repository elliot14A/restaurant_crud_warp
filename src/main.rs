use mobc::{Connection, Pool};
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use std::convert::Infallible;
use tokio_postgres::NoTls;
use warp::{Filter, Rejection};

mod data;
mod db;
mod error;
mod handler;

type Result<T> = std::result::Result<T, Rejection>;
type DBCon = Connection<PgConnectionManager<NoTls>>;
type DBPool = Pool<PgConnectionManager<NoTls>>;

#[tokio::main]
async fn main() {
    let db_pool = db::create_pool().expect("database pool can be created");

    db::init_db(&db_pool)
        .await
        .expect("database can be initialized");

    let health_route = warp::path!("health")
        .and(with_db(db_pool.clone()))
        .and_then(handler::health_handler);
    let recipe = warp::path!("recipes");
    let recipe_route = recipe
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db_pool.clone()))
        .and_then(handler::create_recipe_handler)
        .or(recipe
            .and(warp::get())
            .and(with_db(db_pool.clone()))
            .and_then(handler::fetch_recipes_handler)
        );

    let routes = health_route
        .or(recipe_route)
        .with(warp::cors().allow_any_origin())
        .recover(error::handle_rejection);

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

fn with_db(db_pool: DBPool) -> impl Filter<Extract = (DBPool,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}
