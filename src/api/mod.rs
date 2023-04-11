use serde::{Serialize, Deserialize};

use crate::util::types::{Product, Pagination};

pub mod product;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListResponse {
    pub pagination: Pagination,
    pub data: Vec<Product>
}