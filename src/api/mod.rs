use serde::{Serialize, Deserialize};

use crate::util::types::{Product, Pagination};

pub mod product;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListResponse<T> {
    pub pagination: Pagination,
    pub data: Vec<T>
}

type ListProductRespone = ListResponse<Product>;