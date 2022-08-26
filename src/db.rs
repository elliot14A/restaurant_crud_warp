use crate::{data::*, error, error::Error::*, DBCon, DBPool};
use mobc::Pool;
use mobc_postgres::{
    tokio_postgres::{self, Row},
    PgConnectionManager,
};
use std::time::Duration;
use std::fs;
use std::{str::FromStr, vec};
use tokio_postgres::{Config, Error, NoTls};

type Result<T> = std::result::Result<T, error::Error>;

const DB_POOL_MAX_OPEN: u64 = 32;
const DB_POOL_MAX_IDLE: u64 = 8;
const DB_POOL_TIMEOUT_SECONDS: u64 = 15;
const INIT_SQL: &str = "./db.sql";
const TABLE1: &str = "recipes";
const TABLE2: &str = "ingredients";
const TABLE3: &str = "ingredientstorecipes";

pub async fn init_db(db_pool: &DBPool) -> Result<()> {
    let init_file = fs::read_to_string(INIT_SQL)?;
    let con = get_db_con(db_pool).await?;
    con.batch_execute(init_file.as_str())
        .await
        .map_err(DBInitError)?;
    Ok(())
}

pub async fn get_db_con(db_pool: &DBPool) -> Result<DBCon> {
    db_pool.get().await.map_err(DBPoolError)
}

pub fn create_pool() -> std::result::Result<DBPool, mobc::Error<Error>> {
    let config = Config::from_str("postgres://postgres:123456@127.0.0.1:5432/postgres")?;

    let manager = PgConnectionManager::new(config, NoTls);
    Ok(Pool::builder()
        .max_open(DB_POOL_MAX_OPEN)
        .max_idle(DB_POOL_MAX_IDLE)
        .get_timeout(Some(Duration::from_secs(DB_POOL_TIMEOUT_SECONDS)))
        .build(manager))
}

pub async fn create_recipe(db_pool: &DBPool, body: CreateRecipeRequest) -> Result<Recipe> {
    let con = get_db_con(db_pool).await?;
    let query1 = format!(
        "INSERT INTO {} (name, description, dishsize) VALUES ($1, $2, $3) RETURNING *",
        TABLE1
    );
    let query2: String = format!("INSERT INTO {} (name) VALUES ($1) RETURNING *", TABLE2);
    let query3: String = format!(
        "INSERT INTO {} (rec_id, ing_id, quantity, quantity_unit) VALUES ($1, $2, $3, $4) RETURNING *",
        TABLE3
    );
    let row1 = con
        .query_one(
            query1.as_str(),
            &[&body.name, &body.description, &body.dishsize],
        )
        .await
        .map_err(DBQueryError)?;
    let mut ingredients: Vec<Ingredient> = vec![];
    for i in body.ingredients {
        let row = con
            .query_one(query2.as_str(), &[&i.name])
            .await
            .map_err(DBQueryError)?;
        let id: i32 = row.get(0);
        let name: String = row.get(1);
        ingredients.push(Ingredient {
            id,
            name,
            quantity: i.quantity.clone(),
            quantity_unit: i.quantity_unit.clone(),
        });
        con.query_one(
            query3.as_str(),
            &[
                &row1.get::<usize, i32>(0),
                &row.get::<usize, i32>(0),
                &i.quantity,
                &i.quantity_unit,
            ],
        )
        .await
        .map_err(DBQueryError)?;
    }
    let id: i32 = row1.get(0);
    let name: String = row1.get(1);
    let description: String = row1.get(2);
    let dishsize: i32 = row1.get(3);

    Ok(Recipe {
        id,
        name,
        description,
        dishsize,
        ingredients,
    })
}
pub async fn fetch_recipes(db_pool: &DBPool) -> Result<Vec<Recipe>> {
    let con = get_db_con(db_pool).await?;
    let query1 = format!("SELECT * FROM {}", TABLE3);
    let query2: String = format!("SELECT * FROM {} WHERE ID=$1", TABLE1);
    let query3: String = format!("SELECT * FROM {} WHERE ID=$1", TABLE2);
    let rows: Vec<Row> = con
        .query(query1.as_str(), &[])
        .await
        .map_err(DBQueryError)?;
    let mut recipies: Vec<Recipe> = vec![];
    for i in rows {
        let row = con
            .query_one(query2.as_str(), &[&i.get::<usize, i32>(0)])
            .await
            .map_err(DBQueryError)?;
        let row2 = con
            .query_one(query3.as_str(), &[&i.get::<usize, i32>(1)])
            .await
            .map_err(DBQueryError)?;
        let mut exist = false;
        for j in &mut recipies {
            if j.id == row.get::<usize, i32>(0) {
                exist = true;
                let ingredient = Ingredient {
                    id: row2.get::<usize, i32>(0),
                    name: row2.get::<usize, String>(1),
                    quantity: i.get::<usize, i32>(2),
                    quantity_unit: i.get::<usize, String>(3),
                };
                j.ingredients.push(ingredient)
            }
        }
        if !exist {
            let ingredient = Ingredient {
                id: row2.get::<usize, i32>(0),
                name: row2.get::<usize, String>(1),
                quantity: i.get::<usize, i32>(2),
                quantity_unit: i.get::<usize, String>(3),
            };
            let recipe = Recipe {
                id: row.get::<usize, i32>(0),
                name: row.get::<usize, String>(1),
                description: row.get::<usize, String>(2),
                dishsize: row.get::<usize, i32>(3),
                ingredients: vec![ingredient],
            };
            recipies.push(recipe);
        }
    }

    Ok(recipies)
}
