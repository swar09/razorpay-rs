use std::{collections::HashMap, env};
mod models;
mod payment;
mod order;
mod refund;
mod settlement;
use reqwest::{
    Client, ClientBuilder, Request,
    header::{self, HeaderMap},
};

// async fn client_builder() -> Result<Client, reqwest::Error> {
//     let builder = ClientBuilder::new();
//     let builder = builder.default_headers(HeaderMap::new());
//     Ok(builder.build()?)
// }

// async fn request_builder<'a>(
//     client: &Client,
//     url: &str,
//     (key, value): (&str, &str),
//     query: &[(&str, &str)],
//     body: String,
// ) -> Result<Request, reqwest::Error> {
//     let builder = client
//         .get(url)
//         .header(key, value)
//         .headers(header::HeaderMap::new())
//         .body(body)
//         .query(query);
//     Ok(builder.build()?)
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let key = env::var("API_KEY").expect("API_KEY");
    let value = env::var("SECRET").expect("SECRET");


    // let id = ""; // id 
    // let url = format!("https://api.razorpay.com/v1/payments/{id}/capture");

    // // body
    // let mut map = HashMap::new();
    // map.insert("amount", "1000");
    // map.insert("currency", "INR");

    // // init client
    // let client = client_builder().await?;
    // // let request = re

    // // client.execute(request);

    // // let val: HashMap<String, String> = response.try_into().expect("");
    // // let value: Value = response.json().await?;
    // // println!("{:?}", value);
    Ok(())
}
