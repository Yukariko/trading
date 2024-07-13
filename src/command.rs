use serde_json::json;
use serde::{Serialize, Deserialize};

pub enum Period {
    Day,
    Week,
    Month,
    Year,
}

impl Period {
    fn as_str(&self) -> &'static str {
        match self {
            Period::Day => "D",
            Period::Week => "W",
            Period::Month => "M",
            Period::Year => "Y",
        }
    }
}

pub enum Sender {
    GET,
    POST,
}

pub struct Command<T> {
    pub path : &'static str,
    pub tr_id : &'static str,
    pub sender : Sender,
    pub body : T,
}

#[derive(Serialize, Deserialize, Default)]
#[allow(non_snake_case)]
pub struct Price {
    fid_cond_mrkt_div_code: String,
    fid_input_iscd: String,
}

#[derive(Serialize, Deserialize, Default)]
#[allow(non_snake_case)]
pub struct DailyPrice {
    fid_cond_mrkt_div_code: String,
    fid_input_iscd: String,
    fid_period_div_code: String,
    fid_org_adj_prc: String,
}

#[derive(Serialize, Deserialize, Default)]
#[allow(non_snake_case)]
pub struct Balance {
    fid_cond_mrkt_div_code: String,
    fid_input_iscd: String,
    CAN0: String,
    ACNT_PRDT_CD: String,
    AFHR_FLPR_YN: String,
    OFL_YN: String,
    INQR_DVSN: String,
    UNPR_DVSN: String,
    FUND_STTL_ICLD_YN: String,
    FNCG_AMT_AUTO_RDPT_YN: String,
    PRCS_DVSN: String,
    CTX_AREA_FK100: String,
    CTX_AREA_NK100: String,
}

#[derive(Serialize, Deserialize, Default)]
#[allow(non_snake_case)]
pub struct OrderBuy {
    CAN0: String,
    ACNT_PRDT_CD: String,
    PDNO: String,
    ORD_DVSN: String,
    ORD_QTY: String,
    ORD_UNPR: String,
}

#[derive(Serialize, Deserialize, Default)]
#[allow(non_snake_case)]
pub struct OrderSell {
    CAN0: String,
    ACNT_PRDT_CD: String,
    PDNO: String,
    ORD_DVSN: String,
    ORD_QTY: String,
    ORD_UNPR: String,
}

#[derive(Serialize, Deserialize, Default)]
#[allow(non_snake_case)]
pub struct DailyValue {
    fid_cond_mrkt_div_code: String,
    fid_input_iscd: String,
    FID_INPUT_DATE_1: String,
    FID_INPUT_DATE_2: String,
    fid_period_div_code: String,
    fid_org_adj_prc: String,
}

pub trait ApiCommand {
    fn path(&self) -> &str;
    fn tr_id(&self) -> &str;
    fn sender(&self) -> &Sender;
    fn body(&self) -> Option<serde_json::Value>;
}

impl<T> ApiCommand for Command<T>
where T: Serialize {
    fn path(&self) -> &str {
        self.path
    }
    fn tr_id(&self) -> &str {
        self.tr_id
    } 
    fn sender(&self) -> &Sender {
        &self.sender
    }
    fn body(&self) -> Option<serde_json::Value> {
        Some(serde_json::to_value(&self.body).unwrap())
    }

}

impl Command<Price> {
    pub fn new() -> Self {
        Command {
            path: "/uapi/domestic-stock/v1/quotations/inquire-price",
            tr_id: "FHKST01010100",
            sender: Sender::GET,
            body: Price {
                fid_cond_mrkt_div_code: "J".to_string(),
                ..Price::default()
            },
        }
    }

    pub fn ticker(mut self, ticker: String) -> Self {
        self.body.fid_input_iscd = ticker;
        self
    }
}

impl Command<DailyPrice> {
    pub fn new() -> Self {
        Command {
            path: "/uapi/domestic-stock/v1/quotations/inquire-daily-price",
            tr_id: "FHKST01010400",
            sender: Sender::GET,
            body: DailyPrice {
                fid_cond_mrkt_div_code: "J".to_string(),
                fid_org_adj_prc: "0".to_string(),
                ..DailyPrice::default()
            },
        }
    }

    pub fn ticker(mut self, ticker: String) -> Self {
        self.body.fid_input_iscd = ticker;
        self
    }

    pub fn period(mut self, period: Period) -> Self {
        self.body.fid_period_div_code = period.as_str().to_string();
        self
    }
}

impl Command<Balance> {
    pub fn new() -> Self {
        Command {
            path: "/uapi/domestic-stock/v1/trading/inquire-balance",
            tr_id: "TTTC8434R",
            sender: Sender::GET,
            body: Balance {
                AFHR_FLPR_YN: "N".to_string(),
                INQR_DVSN: "01".to_string(),
                UNPR_DVSN: "01".to_string(),
                FUND_STTL_ICLD_YN: "N".to_string(),
                FNCG_AMT_AUTO_RDPT_YN: "N".to_string(),
                PRCS_DVSN: "00".to_string(),
                ..Balance::default()
            }
        }
    }

    pub fn account_no(mut self, account_no: String) -> Self {
        self.body.CAN0 = account_no;
        self
    }
    pub fn account_cd(mut self, account_cd: String) -> Self {
        self.body.ACNT_PRDT_CD = account_cd;
        self
    }
}

impl Command<OrderBuy> {
    pub fn new() -> Self {
        Command {
            path: "/uapi/domestic-stock/v1/trading/order-cash",
            tr_id: "TTTC0802U",
            sender: Sender::POST,
            body: OrderBuy {
                ORD_DVSN: "01".to_string(),
                ORD_UNPR: "0".to_string(),
                ..OrderBuy::default()
            }
        }
    }

    pub fn account_no(mut self, account_no: String) -> Self {
        self.body.CAN0 = account_no;
        self
    }

    pub fn account_cd(mut self, account_cd: String) -> Self {
        self.body.ACNT_PRDT_CD = account_cd;
        self
    }

    pub fn ticker(mut self, ticker: String) -> Self {
        self.body.PDNO = ticker;
        self
    }

    pub fn count(mut self, count: String) -> Self {
        self.body.ORD_QTY = count;
        self
    }
}

impl Command<OrderSell> {
    pub fn new() -> Self {
        Command {
            path: "/uapi/domestic-stock/v1/trading/order-cash",
            tr_id: "TTTC0801U",
            sender: Sender::POST,
            body: OrderSell {
                ORD_DVSN: "01".to_string(),
                ORD_UNPR: "0".to_string(),
                ..OrderSell::default()
            }
        }
    }

    pub fn account_no(mut self, account_no: String) -> Self {
        self.body.CAN0 = account_no;
        self
    }
    pub fn account_cd(mut self, account_cd: String) -> Self {
        self.body.ACNT_PRDT_CD = account_cd;
        self
    }
    pub fn ticker(mut self, ticker: String) -> Self {
        self.body.PDNO = ticker;
        self
    }
    pub fn count(mut self, count: String) -> Self {
        self.body.ORD_QTY = count;
        self
    }
}

impl Command<DailyValue> {
    pub fn new() -> Self {
        Command {
            path: "/uapi/domestic-stock/v1/quotations/inquire-daily-itemchartprice",
            tr_id: "FHKST03010100",
            sender: Sender::GET,
            body: DailyValue {
                fid_cond_mrkt_div_code: "J".to_string(),
                fid_org_adj_prc: "0".to_string(),
                ..DailyValue::default()
            }
        }
    }

    pub fn date(mut self, start: String, end: String) -> Self {
        self.body.FID_INPUT_DATE_1 = start;
        self.body.FID_INPUT_DATE_2 = end;
        self
    }
    pub fn period(mut self, period: Period) -> Self {
        self.body.fid_period_div_code = period.as_str().to_string();
        self
    }
    pub fn ticker(mut self, ticker: String) -> Self {
        self.body.fid_input_iscd = ticker;
        self
    }
}
