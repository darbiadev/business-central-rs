use std::fmt::format;
use std::slice::from_raw_parts;

use reqwest::{Method, StatusCode};
use serde::de::DeserializeOwned;

struct Pieces {
    base_url: String,
    odata_base_url: String,
    odata_url: String,
    username: String,
    web_service_access_key: String,
}

impl Pieces {
    fn from_pieces(
        base_url: String,
        tenant_id: String,
        environment: String,
        company_name: String,
        username: String,
        web_service_access_key: String,
    ) -> Pieces {
        let base_url_ = format!("{}/{}/{}/", base_url, tenant_id, environment);
        Pieces {
            base_url: base_url_.clone(),
            odata_base_url: base_url_.clone(),
            odata_url: format!("{}Company('{}')/", base_url_.clone(), company_name),
            username,
            web_service_access_key,
        }
    }
}

// Build the URL for all calls
async fn build<T>(
    username: &str,
    web_service_access_key: &str,
    method: Method,
    url: &str,
) -> Result<T, StatusCode>
where
    T: DeserializeOwned,
{
    let response = reqwest::Client::new()
        .request(method, url)
        .basic_auth(username, Some(web_service_access_key))
        .send()
        .await;

    match &response {
        Ok(r) => {
            if r.status() != StatusCode::OK {
                return Err(r.status());
            }
        }
        Err(e) => {
            return if e.is_status() {
                Err(e.status().unwrap())
            } else {
                Err(StatusCode::BAD_REQUEST)
            };
        }
    }

    // Parse the response body as JSON
    let content = response.unwrap().json::<T>().await;

    match content {
        Ok(s) => Ok(s),
        Err(e) => {
            println!("{:?}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

// Make call without parameters nor filters
pub async fn call<T>(
    base_url: String,
    tenant_id: String,
    environment: String,
    company_name: String,
    username: String,
    web_service_access_key: String,
    method:Method,
    path:String,
) -> Result<T, StatusCode>
where
    T: DeserializeOwned,
{
    let pieces = Pieces::from_pieces(
        base_url,
        tenant_id,
        environment,
        company_name,
        username,
        web_service_access_key,
    );

    build(
        pieces.username.as_str(),
        pieces.web_service_access_key.as_str(),
        method,
        format!("{}/{}", pieces.odata_url, path).as_str(),
    )
    .await
}
