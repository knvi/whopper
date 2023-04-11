use crate::util::types::Product;
use anyhow::Result;

use super::ListProductRespone;

pub async fn list(biz_id: String) -> Result<Vec<Product>> {
    let client = reqwest::Client::new();
    let url = format!("{}/products", crate::vars::API_URL);

    let res = client.get(url).bearer_auth(biz_id).send().await?.text().await?;

    let res: ListProductRespone = serde_json::from_str(&res).unwrap();

    let products: Vec<Product> = res
        .data
        .into_iter()
        .map(|product| Product {
            id: product.id,
            created_at: product.created_at,
            name: product.name,
            visibility: product.visibility,
        })
        .collect();

    Ok(products)
}
