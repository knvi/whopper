use serde::{Serialize, Deserialize};

use crate::util::types::{Product, Pagination, Membership, Plan};

pub mod product;
pub mod membership;
pub mod plan;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListResponse<T> {
    pub pagination: Pagination,
    pub data: Vec<T>
}

type ListProductRespone = ListResponse<Product>;
type ListMembershipResponse = ListResponse<Membership>;
type ListPlanResponse = ListResponse<Plan>;

// TODO: MUST: ADD GENERICS FOR LIST FUNCTIONS

