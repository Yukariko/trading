use dotenv::dotenv;
use std::env;
use reqwest::Client;
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
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let app_key = env::var("APP_KEY").expect("APP_KEY must be set.");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET must be set.");
    let domain = env::var("DOMAIN").expect("DOMAIN mustbe set");
    let mut session = Session::new(app_key, app_secret, domain);
    session.token = session.request_token().await.expect("request token failed");
    println!("{}, {}, {}, {:?}", session.app_key, session.app_secret, session.domain, session.token);
}
