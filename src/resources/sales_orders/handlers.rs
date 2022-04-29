use crate::resources::sales_orders::models::SalesOrder;
use crate::{BusinessCentralServices, UrlKeyValue};
use serde_json::Value;

pub async fn get_generic(
    client: BusinessCentralServices,
    path: &str,
    resource_values: Vec<UrlKeyValue>,
) -> Result<Value, reqwest::Error> {
    let response = client
        .make_odata_request(
            reqwest::Method::GET,
            path.to_string(),
            resource_values,
            Default::default(),
            None,
        )
        .await?;
    Ok(response)
}

pub async fn get_order(
    client: BusinessCentralServices,
    order_number: &str,
) -> Result<SalesOrder, reqwest::Error> {
    let response = client
        .make_odata_request(
            reqwest::Method::GET,
            "salesOrder".to_string(),
            vec![
                UrlKeyValue::Text(String::from("Order")),
                UrlKeyValue::Text(order_number.to_string()),
            ],
            Default::default(),
            None,
        )
        .await?;
    let sales_order = serde_json::from_value(response).unwrap();
    Ok(sales_order)
}
