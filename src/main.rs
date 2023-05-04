use dotenv::dotenv;
use std::env;
use reqwest::{Client, header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE, ACCEPT_CHARSET}};
use serde::Deserialize;
use serde_json::json;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

struct Session {
    app_key : String,
    app_secret : String,
    domain : String,
    token : Token,
}

#[derive(Deserialize, Debug, Default)]
struct Token {
    access_token : String,
    #[serde(alias = "access_token_token_expired")]
    expired : String,
    token_type : String,
    expires_in : u32,
}

impl Session {
    fn new(app_key: String, app_secret: String, domain: String) -> Session {
        Session {
            app_key: app_key,
            app_secret: app_secret,
            domain: domain,
            token: Token::default(),
        }
    }

    async fn request_token(&self) -> Result<Token> {
        let path = "/oauth2/tokenP";
        let url = self.domain.clone() + path;
        let body = json!({
            "grant_type": "client_credentials",
            "appkey": self.app_key,
            "appsecret": self.app_secret
        });

        let response = Client::new()
            .post(url)
            .json(&body)
            .send().await.expect("request send failed");
        let res : Token = response.json().await?;
        Ok(res)
    }

    async fn __fetch(&self, path: &str, tr_id: &str, body: Option<&serde_json::Value>) -> Result<serde_json::Value> {
        let url = format!("{}{}", self.domain, path);
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_str("application/json").unwrap());
        headers.insert(ACCEPT, HeaderValue::from_str("text/plain").unwrap());
        headers.insert(ACCEPT_CHARSET, HeaderValue::from_str("UTF-8").unwrap());
        headers.insert("authorization", HeaderValue::from_str(&format!("{} {}", self.token.token_type, self.token.access_token)).unwrap());
        headers.insert("appkey", HeaderValue::from_str(&self.app_key).unwrap());
        headers.insert("appsecret", HeaderValue::from_str(&self.app_secret).unwrap());
        headers.insert("tr_id", HeaderValue::from_str(tr_id).unwrap());

        if let Some(body) = body {
            let response = Client::new()
                .post(url)
                .headers(headers)
                .json(body)
                .send().await.expect("request send failed");
            let res = response.json().await?;
            return Ok(res);
        } else {
            let response = Client::new()
                .get(url)
                .headers(headers)
                .send().await.expect("request send failed");
            let res = response.json().await?;
            return Ok(res);
        }
    }

    async fn get_price(&self, stock_no: &str) -> Result<serde_json::Value> {
        let path = format!("/uapi/domestic-stock/v1/quotations/inquire-price?fid_cond_mrkt_div_code=J&fid_input_iscd={}", stock_no);
        let res = self.__fetch(&path, "FHKST01010100", None).await.expect("__fetch failed");
        Ok(res)
    }

    async fn get_price_day(&self, stock_no: &str) -> Result<serde_json::Value> {
        let path = format!("/uapi/domestic-stock/v1/quotations/inquire-daily-price?fid_cond_mrkt_div_code=J&fid_input_iscd={}&fid_period_div_code=D&fid_org_adj_prc=0", stock_no);
        let res = self.__fetch(&path, "FHKST01010400", None).await.expect("__fetch failed");
        Ok(res)
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let app_key = env::var("APP_KEY").expect("APP_KEY must be set.");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET must be set.");
    let domain = env::var("DOMAIN").expect("DOMAIN must be set");
    let mut session = Session::new(app_key, app_secret, domain);
    session.token = session.request_token().await.expect("request token failed");

    let price = session.get_price("005930").await.expect("get price failed");
    println!("samsung : {}", price);
    let price = session.get_price_day("005930").await.expect("get price failed");
    println!("samsung : {}", price);
}
