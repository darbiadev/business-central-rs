use std::collections::HashMap;

use reqwest::header::{HeaderMap, IF_MATCH};
use reqwest::{Client, Method};
use serde::de::DeserializeOwned;

pub mod resources;

pub struct BusinessCentralServices {
    username: String,
    web_service_access_key: String,
    odata_base_url: String,
    odata_url: String,
    client: Client,
}

impl BusinessCentralServices {
    pub fn new(
        base_url: String,
        tenant_id: String,
        environment: String,
        company_name: String,
        username: String,
        web_service_access_key: String,
    ) -> BusinessCentralServices {
        let base_url_ = format!("{}/{}/{}/ODataV4/", base_url, tenant_id, environment);
        BusinessCentralServices {
            odata_base_url: base_url_.clone(),
            odata_url: format!("{}Company('{}')/", base_url_, company_name),
            username,
            web_service_access_key,
            client: Client::new(),
        }
    }

    // params: dict[str, str] = None,
    pub async fn make_odata_request<T>(
        &self,
        method: Method,
        resource_name: String,
        resource_values: Vec<UrlKeyValue>,
        _resource_data: HashMap<String, String>,
        etag: Option<String>,
    ) -> reqwest::Result<T>
    where
        T: DeserializeOwned,
    {
        let mut headers = HeaderMap::new();
        if etag.is_some() {
            headers.insert(IF_MATCH, etag.unwrap().parse().unwrap());
        }

        let url = format!(
            "{}{}",
            &self.odata_url,
            build_resource_url(resource_name, resource_values)
        );

        let request = self
            .client
            .request(method, url)
            .basic_auth(&self.username, Some(&self.web_service_access_key))
            .headers(headers);

        request.send().await?.json::<T>().await
    }

    pub async fn make_unbound_request(
        &self,
        method: Method,
        codeunit: String,
        procedure: String,
        _data: HashMap<String, String>,
    ) -> reqwest::Result<reqwest::Response> {
        let headers = HeaderMap::new();
        let url = format!("{}/{}_{}/", &self.odata_base_url, codeunit, procedure);

        reqwest::Client::new()
            .request(method, url)
            .basic_auth(&self.username, Some(&self.web_service_access_key))
            .headers(headers)
            .send()
            .await
    }
}

pub enum UrlKeyValue {
    Text(String),
    Number(i16),
}

impl std::fmt::Display for UrlKeyValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            UrlKeyValue::Text(text) => write!(f, "'{}'", text),
            UrlKeyValue::Number(number) => write!(f, "{}", number),
        }
    }
}

fn build_resource_url(resource_name: String, resource_values: Vec<UrlKeyValue>) -> String {
    let mut resource_url = resource_name;
    if !resource_values.is_empty() {
        if resource_values.len() == 1 {
            resource_url.push_str(format!("({})", resource_values[0]).as_str());
        } else {
            resource_url.push('(');
            let mut value_iter = resource_values.iter();
            let last_value = value_iter.next_back().unwrap().to_string();
            for value in value_iter {
                resource_url.push_str(value.to_string().as_str());
                resource_url.push(',');
            }
            resource_url.push_str(last_value.as_str());
            resource_url.push(')');
        }
    }
    resource_url
}
