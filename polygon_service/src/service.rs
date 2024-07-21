use std::error::Error;
use std::time::Duration;
use reqwest::{Client, ClientBuilder};
use reqwest::header::{HeaderMap, HeaderValue};

pub struct PolygonService{
    pub client: Client
}

impl PolygonService {
    pub fn create_client()->Result<Client, Box<dyn Error>>{
        let mut default_headers = HeaderMap::new();
        default_headers.append("Authorization", HeaderValue::from_str("Bearer YbmdoFrD8vVQ42lFB56D7TkYurxcKOYK").unwrap());

        let builder = ClientBuilder::new()
            .connect_timeout(Duration::new(30,0))
            .default_headers(default_headers);

        Ok(builder.build().unwrap())
    }

    async fn get(&self, uri:&str)->Result<String, Box<dyn Error>>{
        let response = self.client.get(String::from("https://api.polygon.io/v2/") + uri)
            .send()
            .await?
            .text()
            .await?;

        Ok(response)
    }

    pub async fn get_ticker_data(&self, symbol:&str) -> Result<(), Box<dyn Error>>{
        let _response = self.get(format!("aggs/ticker/{}/range/1/day/2023-01-09/2023-01-09", symbol).as_str())
            .await?;

        Ok(())
    }

}