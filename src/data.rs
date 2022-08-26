use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Recipe {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub dishsize: i32,
    pub ingredients: Vec<Ingredient>
}

#[derive(Serialize, Deserialize)]
pub struct Ingredient {
    pub id: i32,
    pub name: String,
    pub quantity: i32,
    pub quantity_unit: String
}

#[derive(Deserialize)]
pub struct CreateRecipeRequest {
    pub name: String,
    pub description: String,
    pub dishsize: i32,
    pub ingredients: Vec<CreateIngredientRequest>
}

#[derive(Deserialize)]
pub struct CreateIngredientRequest {
    pub name: String,
    pub quantity: i32,
    pub quantity_unit: String
}