use crate::util::types::Plan;
use anyhow::Result;

use super::ListPlanResponse;

pub async fn list(biz_id: String) -> Result<Vec<Plan>> {
    let client = reqwest::Client::new();
    let url = format!("{}/memberships", crate::vars::API_URL);

    let res = client.get(url).bearer_auth(biz_id).send().await?.text().await?;

    let res: ListPlanResponse = serde_json::from_str(&res).unwrap();

    let plans: Vec<Plan> = res
        .data
        .into_iter()
        .map(|plan| Plan {
            id: plan.id,
            internal_notes: plan.internal_notes,
            renewal_price: plan.renewal_price,
            initial_price: plan.initial_price,
        })
        .collect();

    Ok(plans)
}
