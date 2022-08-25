use std::convert::Infallible;
use warp::Filter;

use crate::types::DBPool;

pub fn with_db(db_pool: DBPool) -> impl Filter<Extract = (DBPool,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}
