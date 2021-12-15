use end_points::{EndPoints, Methods};
use lazy_static::lazy_static;
use reqwest::Body;
use serde::Serialize;
use serde_json::Value;
pub mod client;
pub mod end_points;
pub mod implements;
#[derive(Debug)]
pub enum Error {
    ReqwestError(reqwest::Error),
    SerdeError(serde_json::Error),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::ReqwestError(e)
    }
}
impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::SerdeError(e)
    }
}

// pub async fn simple_request<T, V>(
//     url: EndPoints,
//     query: Option<&T>,
//     body: Option<V>,
// ) -> Result<Value, Error>
// where
//     T: Serialize + ?Sized,
//     V: Into<Body>,
// {
//     lazy_static! {
//         static ref CLIENT: reqwest::Client = reqwest::Client::new();
//     }
//     let mut req = match url.method() {
//         Methods::Get => CLIENT.get(url.path()),
//         Methods::Post => CLIENT.post(url.path()),
//         Methods::Put => CLIENT.put(url.path()),
//         Methods::Patch => CLIENT.patch(url.path()),
//         Methods::Delete => CLIENT.delete(url.path()),
//     };
//     if let Some(query) = query {
//         req = req.query(query)
//     }
//     if let Some(body) = body {
//         req = req.body(body)
//     }
//     Ok(serde_json::from_str(&req.send().await?.text().await?)?)
// }

#[feature(test)]
mod test {
    use std::slice::SliceIndex;

    use super::implements;
    use crate::client::Client;
    use crate::end_points::*;
    use reqwest::Body;
    use serde::Serialize;

    #[tokio::test]
    pub async fn aaa() {
        let client = Client {};
        println!("{:#?}", client.get_::<String, String>(None, None).await)
    }

    #[test]
    pub fn test() {
        let point = EndPoints::GetUsersusernameEventsOrgsorg("test".to_owned(), "aa".to_owned());
        println!("{}", point.path());
        println!("{}", point.path());
        println!("{}", point.path());
        println!("{}", point.path());
        println!("{}", point.path());
        println!("{}", point.path());
        println!("{}", point.path());
    }
    // pub fn test2() {
    //     let client = reqwest::Client::new();
    //     client.get("a").query(query)
    // }
    pub fn request<T, V>(url: EndPoints, query: Option<&T>, body: Option<V>)
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        let client = reqwest::Client::new();
        let mut req = match url.method() {
            Methods::Get => client.get(url.path()),
            Methods::Post => client.post(url.path()),
            Methods::Put => client.put(url.path()),
            Methods::Patch => client.patch(url.path()),
            Methods::Delete => client.delete(url.path()),
        };
        if let Some(query) = query {
            req = req.query(query)
        }
        if let Some(body) = body {
            req = req.body(body)
        }
        req.send();
    }
}
