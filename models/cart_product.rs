use sqlx::FromRow;
use super::cart::Cart;
use super::product::Product;

#[derive(Debug, Clone, FromRow)]
pub struct CartProduct {
    pub cart_id: i32,
    pub product_id: i32,
    pub quantity: Option<i32>,

    // belongs_to relations
    #[sqlx(skip)]
    pub cart: Option<Cart>,
    #[sqlx(skip)]
    pub product: Option<Product>,
}
