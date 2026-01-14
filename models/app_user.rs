use sqlx::FromRow;
use chrono;
use super::cart::Cart;

#[derive(Debug, Clone, FromRow)]
pub struct AppUser {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,

    // has_many relations
    #[sqlx(skip)]
    pub carts: Option<Vec<Cart>>,
}
