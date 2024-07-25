use std::error::Error;
use std::time::Duration;
use reqwest::{Client, ClientBuilder};
use reqwest::header::{HeaderMap, HeaderValue};
use crate::{
    dto::{TickerData}
};

pub struct PolygonService{
    pub client: Client
}

impl PolygonService {
    pub fn create_client()->Result<Client, Box<dyn Error>>{
        let mut default_headers = HeaderMap::new();
        default_headers.append("Authorization", HeaderValue::from_str("Bearer jnUVdJ5Vb1lzNbzShbVbJXp1tPEaCXA6").unwrap());

        let builder = ClientBuilder::new()
            .connect_timeout(Duration::new(30,0))
            .default_headers(default_headers);

        match builder.build() {
            Ok(client) => Ok(client),
            Err(err) => Err(err.into())
        }
    }

    async fn get(&self, uri:&str)->Result<String, Box<dyn Error>>{
        let response = self.client.get(String::from("https://api.polygon.io/v2/") + uri)
            .send().await?
            .text().await?;

        Ok(response)
    }

    pub async fn get_ticker_data(&self, symbol:&str) -> Result<TickerData, Box<dyn Error>>{
        let response = self.get(format!("aggs/ticker/{}/range/1/day/2023-01-09/2023-01-09", symbol).as_str())
            .await?;

        Ok( serde_json::from_str(response.as_str())?)
    }

}