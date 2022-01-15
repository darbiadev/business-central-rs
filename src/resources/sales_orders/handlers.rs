use crate::resources::sales_orders::models::SalesOrder;
use crate::{Client, UrlKeyValue};

pub async fn get_order(client: Client, order_number: &str) -> Result<SalesOrder, reqwest::Error> {
    let sales_order = client
        .make_odata_request::<SalesOrder>(
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
    Ok(sales_order)
}
