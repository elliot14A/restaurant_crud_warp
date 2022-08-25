mod data;
mod db;
mod handler;
mod types;
mod utils;

use warp::{hyper::StatusCode, Filter};

#[tokio::main]
async fn main() {
    let db_pool = db::create_pool().expect("dbpool can be create");
    db::init_db(&db_pool)
        .await
        .expect("database can be initialized");
    let health_route = warp::path!("health").map(|| StatusCode::OK);
    let routes = health_route.with(warp::cors().allow_any_origin());
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
