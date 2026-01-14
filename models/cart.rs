use sqlx::FromRow;
use chrono;
use super::app_user::AppUser;
use super::cart_product::CartProduct;

#[derive(Debug, Clone, FromRow)]
pub struct Cart {
    pub id: i32,
    pub app_user_id: i32,
    pub created_at: Option<chrono::NaiveDateTime>,

    // belongs_to relations
    #[sqlx(skip)]
    pub app_user: Option<AppUser>,

    // has_many relations
    #[sqlx(skip)]
    pub cart_products: Option<Vec<CartProduct>>,
}
