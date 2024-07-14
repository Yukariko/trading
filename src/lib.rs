use reqwest::{Client, header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE, ACCEPT_CHARSET}};
use serde::{Serialize, Deserialize};
use serde_json::json;
use std::path::Path;
use std::fs;
use std::ops::Add;
use chrono::{Utc, NaiveDateTime, FixedOffset};
use command::{ApiCommand, Sender};
pub mod command;
pub mod database;
pub mod strategy;
pub mod time_runner;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub struct Session {
    app_key : String,
    app_secret : String,
    domain : String,
    token : Token,
    client : Client,
    header : HeaderMap,
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct Token {
    access_token : String,
    #[serde(alias = "access_token_token_expired")]
    expired : String,
    token_type : String,
    expires_in : u32,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct WsKey {
    approval_key : String,
}

const TOKEN_FILE_PATH: &str = "token.json";

impl Token {
    fn is_expired(&self) -> bool {
        let fixed = FixedOffset::east_opt(3600 * 9).unwrap();
        let now: NaiveDateTime = Utc::now().naive_utc().add(fixed);
        let expired = NaiveDateTime::parse_from_str(&self.expired, "%Y-%m-%d %H:%M:%S")
            .expect("Failed to parse specific time");

        now > expired
    }
}

impl Session {
    pub async fn new(app_key: String, app_secret: String, domain: String) -> Result<Session> {
        let mut session = Session {
            app_key,
            app_secret,
            domain,
            token: Token::default(),
            client: Client::new(),
            header: HeaderMap::new(),
        };

        session.load_access_token().await;
        session.header.insert(CONTENT_TYPE, HeaderValue::from_str("application/json").unwrap());
        session.header.insert(ACCEPT, HeaderValue::from_str("text/plain").unwrap());
        session.header.insert(ACCEPT_CHARSET, HeaderValue::from_str("UTF-8").unwrap());
        session.header.insert("authorization", HeaderValue::from_str(&format!("{} {}", session.token.token_type, session.token.access_token)).unwrap());
        session.header.insert("appkey", HeaderValue::from_str(&session.app_key).unwrap());
        session.header.insert("appsecret", HeaderValue::from_str(&session.app_secret).unwrap());
        Ok(session)
    }
    
    async fn read_token_from_file(&self) -> Option<Token> {
        if Path::new(TOKEN_FILE_PATH).exists() {
            let data = fs::read_to_string(TOKEN_FILE_PATH).ok()?;
            let token: Token = serde_json::from_str(&data).ok()?;
            Some(token)
        } else {
            None
        }
    }

    async fn save_token_to_file(&self) {
        let data = serde_json::to_string(&self.token).unwrap();
        fs::write(TOKEN_FILE_PATH, data).expect("Unable to write token to file");
    }

    async fn load_access_token(&mut self) {
        match self.read_token_from_file().await {
            Some(token) => {
                self.token = token;
                if self.token.is_expired() {
                    self.token = self.request_token().await
                        .expect("request token failed");
                    self.save_token_to_file().await;
                }
            },
            None => {
                self.token = self.request_token().await
                    .expect("request token failed");
                self.save_token_to_file().await;
            },
        };
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

    pub async fn request_ws_key(&self) -> Result<WsKey> {
        let url = format!("{}/oauth2/Approval", self.domain);
        let body = json!({
            "grant_type": "client_credentials",
            "appkey": self.app_key,
            "secretkey": self.app_secret,
        });

        let response = self.client
            .post(url)
            .json(&body)
            .send().await.expect("request send failed");
        let res : WsKey = response.json().await?;
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

    pub async fn execute(&self, command: &Box<dyn ApiCommand>) -> Result<serde_json::Value> {
        let res = self.__fetch(command.path(), command.tr_id(), command.sender(), &command.body()).await.expect("__fetch failed");
        Ok(res)
    }

    pub async fn execute_vec(&self, commands: &Vec<Box<dyn ApiCommand>>) -> Result<Vec<serde_json::Value>> {
        let mut results = Vec::<serde_json::Value>::with_capacity(commands.len());
        for command in commands {
            let res = self.execute(command).await.expect("execute failed");
            results.push(res);
        }
        Ok(results)
    }
}

pub struct WsSession {
    key : WsKey,
    domain : String,
    body : serde_json::Value,
}

impl WsSession {
    pub async fn new(key: WsKey, domain: String) -> Result<Self> {
        let body = json!({
            "header" : {
                "approval_key" : key.approval_key,
                "custtype" : "P",
                "tr_type" : "1",
                "content-type" : "utf-8",
            },
            "body" : {
                "input" : {
                    "tr_id" : "",
                    "tr_key" : "",
                }
            }
        });
        let ws_session = WsSession {
            key,
            domain,
            body,
        };

        Ok(ws_session)
    }

}
