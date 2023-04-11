use crate::util::types::Membership;
use anyhow::Result;

use super::ListMembershipResponse;

pub async fn list(biz_id: String) -> Result<Vec<Membership>> {
    let client = reqwest::Client::new();
    let url = format!("{}/memberships", crate::vars::API_URL);

    let res = client.get(url).bearer_auth(biz_id).send().await?.text().await?;

    let res: ListMembershipResponse = serde_json::from_str(&res).unwrap();

    let products: Vec<Membership> = res
        .data
        .into_iter()
        .map(|membership| Membership {
            id: membership.id,
            product: membership.product,
            user: membership.user,
            plan: membership.plan,
            timestamp: membership.timestamp,
        })
        .collect();

    Ok(products)
}
