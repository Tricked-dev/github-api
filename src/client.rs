use crate::{
    end_points::{EndPoints, Methods},
    Error,
};
use reqwest::{Body, header};
use serde::{de::DeserializeOwned, Serialize};

pub struct Client {}
impl Client {
    pub async fn req<T, V, A: DeserializeOwned>(
        &self,
        url: EndPoints,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<A, Error>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
          let mut headers = header::HeaderMap::new();
            headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_str("tricked.pro/v2").unwrap(),
        );
        let client = reqwest::Client::builder().default_headers(headers).build().unwrap();
        let path = format!("https://api.github.com{}", url.path());
        let mut req = match url.method() {
            Methods::Get => client.get(path),
            Methods::Post => client.post(path),
            Methods::Put => client.put(path),
            Methods::Patch => client.patch(path),
            Methods::Delete => client.delete(path),
        };
        if let Some(query) = query {
            req = req.query(query)
        }
        if let Some(body) = body {
            req = req.body(body)
        }
        let txt = &req.send().await?.text().await?;
        println!("{}", txt);
        Ok(serde_json::from_str(txt)?)
    }
}
