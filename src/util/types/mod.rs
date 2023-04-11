use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub access_token: String
}

#[derive(Serialize, Deserialize, Debug, Clone, Tabled)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub visibility: String,
    pub created_at: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Company {
    pub id: String,
    pub title: String,
    pub route: String,
    pub image_url: String,
    pub access_token: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Membership {
    /// Membership ID
    pub id: String,
    /// Product ID
    pub product: String,
    /// ID of the User that owns the Membership
    pub user: String,
    /// Plan ID
    pub plan: String,
    /// Timestamp of the Expiral
    pub timestamp: i32
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Plan {
    pub id: String,
    pub internal_notes: String,
    pub renewal_price: f32,
    pub initial_price: f32
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pagination {
    pub current_page: i32,
    pub total_page: i32,
    pub total_count: i32,
}