use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{config::Config, util::types::Pagination};
use anyhow::Result;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListResponse<T> {
    pub pagination: Pagination,
    pub data: Vec<T>,
}

/// Returns a list of items
pub async fn list<T>(biz_id: Option<String>, url: String) -> Result<Vec<T>>
where
    T: DeserializeOwned + Into<T>,
{
    let client = reqwest::Client::new();

    if biz_id.is_some() {
        let url = format!("{}/{}", crate::vars::API_URL, url);

        let res = client
            .get(url)
            .bearer_auth(biz_id.unwrap())
            .send()
            .await?
            .text()
            .await?;

        let response: ListResponse<T> = serde_json::from_str(&res).unwrap();

        let items: Vec<T> = response
            .data
            .into_iter()
            .map(|item| T::into(item))
            .collect();

        Ok(items)
    } else {
        let config = Config::load();
        let id = config.get_user().unwrap().to_owned();
        let id = id.access_token;

        let url = format!("{}/me/{}", crate::vars::API_URL, url);

        let res = client.get(url).bearer_auth(id).send().await?.text().await?;

        let response: ListResponse<T> = serde_json::from_str(&res).unwrap();

        let items: Vec<T> = response
            .data
            .into_iter()
            .map(|item| T::into(item))
            .collect();

        Ok(items)
    }
}
