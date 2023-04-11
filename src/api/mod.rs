use serde::{Serialize, Deserialize, de::DeserializeOwned};

use crate::util::types::Pagination;
use anyhow::Result;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListResponse<T> {
    pub pagination: Pagination,
    pub data: Vec<T>
}

pub async fn list<T>(biz_id: String, url: String) -> Result<Vec<T>> 
where
    T: DeserializeOwned + Into<T>
{
    let client = reqwest::Client::new();
    let url = format!("{}/{}", crate::vars::API_URL, url);

    let res = client.get(url).bearer_auth(biz_id).send().await?.text().await?;

    let res: ListResponse<T> = serde_json::from_str(&res).unwrap();

    let items: Vec<T> = res
        .data
        .into_iter()
        .map(|item| T::into(item))
        .collect();

    Ok(items)
}