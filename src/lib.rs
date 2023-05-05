use reqwest::{Client, header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE, ACCEPT_CHARSET}};
use serde::Deserialize;
use serde_json::{json, Map};
pub mod command;
use command::{Command, Sender};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub struct Session {
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
    pub async fn new(app_key: String, app_secret: String, domain: String) -> Result<Session> {
        let mut session = Session {
            app_key: app_key,
            app_secret: app_secret,
            domain: domain,
            token: Token::default(),
        };
        session.token = session.request_token().await.expect("request token failed");
        Ok(session)
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

    fn make_url(mut url: String, body: &serde_json::Value) -> String {
        let map : &Map<String, serde_json::Value> = body.as_object().unwrap();
        let mut first = true;
        for (key, value) in map.iter() {
            if first {
                url.push('?');
                first = false;
            } else {
                url.push('&');
            }
            url.push_str(key);
            url.push('=');
            url.push_str(value.as_str().unwrap());
        }
        url
    }

    async fn __fetch(&self, path: &str, tr_id: &str, sender: &Sender, body: &Option<serde_json::Value>) -> Result<serde_json::Value> {
        let mut url = format!("{}{}", self.domain, path);
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_str("application/json").unwrap());
        headers.insert(ACCEPT, HeaderValue::from_str("text/plain").unwrap());
        headers.insert(ACCEPT_CHARSET, HeaderValue::from_str("UTF-8").unwrap());
        headers.insert("authorization", HeaderValue::from_str(&format!("{} {}", self.token.token_type, self.token.access_token)).unwrap());
        headers.insert("appkey", HeaderValue::from_str(&self.app_key).unwrap());
        headers.insert("appsecret", HeaderValue::from_str(&self.app_secret).unwrap());
        headers.insert("tr_id", HeaderValue::from_str(tr_id).unwrap());

        let request = match sender {
            Sender::POST => {
                let request = Client::new()
                    .post(url)
                    .headers(headers);
                if let Some(body) = body {
                    request.json(&body)
                } else {
                    request
                }
            },
            Sender::GET => {
                if let Some(body) = body {
                    url = Self::make_url(url, &body);
                }
                let request = Client::new()
                    .get(url)
                    .headers(headers);
                request
            }
        };

        let response = request.send().await.expect("request send failed");
        let result = response.json().await?;
        Ok(result)
    }

    pub async fn execute(&self, command: &Command) -> Result<serde_json::Value> {
        let res = self.__fetch(command.base.path, command.base.tr_id, &command.base.sender, &command.args).await.expect("__fetch failed");
        Ok(res)
    }
}
