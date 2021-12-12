use std::collections::HashMap;
use std::fmt;

use reqwest::header::{HeaderMap, IF_MATCH};
use reqwest::{Method, StatusCode};

pub struct Client {
    username: String,
    web_service_access_key: String,
    odata_base_url: String,
    odata_url: String,
}

impl Client {
    pub fn new(
        base_url: String,
        tenant_id: String,
        environment: String,
        company_name: String,
        username: String,
        web_service_access_key: String,
    ) -> Client {
        let base_url_ = format!("{}/{}/{}/ODataV4/", base_url, tenant_id, environment);
        Client {
            odata_base_url: base_url_.clone(),
            odata_url: format!("{}Company('{}')/", base_url_, company_name),
            username,
            web_service_access_key,
        }
    }

    async fn make_request(
        &self,
        method: Method,
        url: String,
        headers: HeaderMap,
        _parameters: Option<HashMap<String, String>>,
        _data: Option<HashMap<String, String>>,
    ) -> Result<String, StatusCode> {
        let request = reqwest::Client::new()
            .request(method, url)
            .basic_auth(&self.username, Some(&self.web_service_access_key))
            .headers(headers);

        let response = request.send().await;

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

        Ok(response.unwrap().text().await.unwrap())
    }

    // params: dict[str, str] = None,
    pub async fn make_odata_request(
        &self,
        method: Method,
        resource_name: String,
        resource_values: Option<Vec<UrlKeyValue>>,
        _resource_data: HashMap<String, String>,
        etag: Option<String>,
    ) -> Result<String, StatusCode> {
        let mut headers = HeaderMap::new();
        if etag.is_some() {
            headers.insert(IF_MATCH, etag.unwrap().parse().unwrap());
        }

        let response = &self
            .make_request(
                method,
                format!(
                    "{}{}",
                    &self.odata_url,
                    build_resource_url(resource_name, resource_values)
                ),
                headers,
                None,
                None,
            )
            .await;
        response.clone()
    }

    pub async fn make_unbound_request(
        &self,
        method: Method,
        codeunit: String,
        procedure: String,
        _data: HashMap<String, String>,
    ) -> Result<String, StatusCode> {
        let response = &self
            .make_request(
                method,
                format!("{}/{}_{}/", &self.odata_base_url, codeunit, procedure),
                HeaderMap::new(),
                None,
                None,
            )
            .await;
        response.clone()
    }
}

pub enum UrlKeyValue {
    Text(String),
    Number(i16),
}

impl fmt::Display for UrlKeyValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UrlKeyValue::Text(text) => write!(f, "'{}'", text),
            UrlKeyValue::Number(number) => write!(f, "{}", number),
        }
    }
}

pub fn build_resource_url(
    resource_name: String,
    resource_values: Option<Vec<UrlKeyValue>>,
) -> String {
    let mut resource_url = resource_name;
    if let Some(values) = resource_values {
        if !values.is_empty() {
            if values.len() == 1 {
                resource_url.push_str(format!("({})", values[0]).as_str());
            } else {
                resource_url.push('(');
                let mut value_iter = values.iter();
                let last_value = value_iter.next_back().unwrap().to_string();
                for value in value_iter {
                    resource_url.push_str(value.to_string().as_str());
                    resource_url.push(',');
                }
                resource_url.push_str(last_value.as_str());
                resource_url.push(')');
            }
        }
    }
    resource_url
}
