use sqlx::FromRow;
use chrono;
use rust_decimal::Decimal;
use super::cart_product::CartProduct;

#[derive(Debug, Clone, FromRow)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub price: Decimal,
    pub stock: i32,
    pub created_at: chrono::NaiveDateTime,

    // has_many relations
    #[sqlx(skip)]
    pub cart_products: Option<Vec<CartProduct>>,
}
