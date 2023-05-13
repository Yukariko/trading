use reqwest::{Client, header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE, ACCEPT_CHARSET}};
use serde::Deserialize;
use serde_json::json;
pub mod command;
pub mod database;
pub mod strategy;
pub mod time_runner;
use command::{Command, Sender};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub struct Session {
    app_key : String,
    app_secret : String,
    domain : String,
    token : Token,
    client : Client,
    header : HeaderMap,
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
            client: Client::new(),
            header: HeaderMap::new(),
        };
        session.token = session.request_token().await.expect("request token failed");

        session.header.insert(CONTENT_TYPE, HeaderValue::from_str("application/json").unwrap());
        session.header.insert(ACCEPT, HeaderValue::from_str("text/plain").unwrap());
        session.header.insert(ACCEPT_CHARSET, HeaderValue::from_str("UTF-8").unwrap());
        session.header.insert("authorization", HeaderValue::from_str(&format!("{} {}", session.token.token_type, session.token.access_token)).unwrap());
        session.header.insert("appkey", HeaderValue::from_str(&session.app_key).unwrap());
        session.header.insert("appsecret", HeaderValue::from_str(&session.app_secret).unwrap());
        Ok(session)
    }

    async fn request_token(&self) -> Result<Token> {
        let url = format!("{}/oauth2/tokenP", self.domain);
        let body = json!({
            "grant_type": "client_credentials",
            "appkey": self.app_key,
            "appsecret": self.app_secret
        });

        let response = self.client
            .post(url)
            .json(&body)
            .send().await.expect("request send failed");
        let res : Token = response.json().await?;
        Ok(res)
    }

    async fn __fetch(&self, path: &str, tr_id: &str, sender: &Sender, body: &Option<serde_json::Value>) -> Result<serde_json::Value> {
        let url = format!("{}{}", self.domain, path);
        let mut header = self.header.clone();
        header.insert("tr_id", HeaderValue::from_str(tr_id).unwrap());
        let request = match sender {
            Sender::POST => {
                let request = self.client
                    .post(url)
                    .headers(header);
                if let Some(body) = body {
                    request.json(&body)
                } else {
                    request
                }
            },
            Sender::GET => {
                let request = self.client
                    .get(url)
                    .headers(header);
                if let Some(body) = body {
                    request.query(&body)
                } else {
                    request
                }
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

    pub async fn execute_vec(&self, commands: &Vec<Command>) -> Result<Vec<serde_json::Value>> {
        let mut results = Vec::<serde_json::Value>::with_capacity(commands.len());
        for command in commands {
            let res = self.__fetch(command.base.path, command.base.tr_id, &command.base.sender, &command.args).await.expect("__fetch failed");
            results.push(res);
        }
        Ok(results)
    }
}
