use serde_json::json;

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

pub enum CommandType {
    Price,
    DailyPrice,
    Balance,
    OrderBuy,
    OrderSell,
    DailyValue,
}

pub struct CommandBase {
    pub path : &'static str,
    pub tr_id : &'static str,
    pub sender : Sender
}

pub struct Command {
    pub base : CommandBase,
    pub args : Option<serde_json::Value>
}

impl Command {
    pub fn new(command: CommandType) -> Command {
        let command_base = match command {
            CommandType::Price => CommandBase {
                path: "/uapi/domestic-stock/v1/quotations/inquire-price",
                tr_id: "FHKST01010100",
                sender: Sender::GET
            },
            CommandType::DailyPrice => CommandBase {
                path: "/uapi/domestic-stock/v1/quotations/inquire-daily-price",
                tr_id: "FHKST01010400",
                sender: Sender::GET
            },
            CommandType::Balance => CommandBase {
                path: "/uapi/domestic-stock/v1/trading/inquire-balance",
                tr_id: "TTTC8434R",
                sender: Sender::GET
            },
            CommandType::OrderBuy => CommandBase {
                path: "/uapi/domestic-stock/v1/trading/order-cash",
                tr_id: "TTTC0802U",
                sender: Sender::POST
            },
            CommandType::OrderSell => CommandBase {
                path: "/uapi/domestic-stock/v1/trading/order-cash",
                tr_id: "TTTC0801U",
                sender: Sender::POST
            },
            CommandType::DailyValue => CommandBase {
                path: "/uapi/domestic-stock/v1/quotations/inquire-daily-itemchartprice",
                tr_id: "FHKST03010100",
                sender: Sender::GET
            }
        };

        Command {
            base : command_base,
            args : None
        }
    }

    pub fn args(&mut self, body: serde_json::Value) {
        self.args = Some(body);
    }
}

pub trait PriceCommand {
    fn new(stock_no: &str) -> Command {
        let mut command = Command::new(CommandType::Price);
        command.args(json!({
            "fid_cond_mrkt_div_code" : "J",
            "fid_input_iscd" : stock_no,
        }));
        command
    }
}

impl PriceCommand for Command {}

pub trait DailyPriceCommand {
    fn new(stock_no: &str, period: &Period) -> Command {
        let mut command = Command::new(CommandType::DailyPrice);
        command.args(json!({
            "fid_cond_mrkt_div_code" : "J",
            "fid_input_iscd" : stock_no,
            "fid_period_div_code" : period.as_str(),
            "fid_org_adj_prc" : "0",
        }));
        command
    }
}

impl DailyPriceCommand for Command {}

pub trait BalanceCommand {
    fn new(account_no: &str, account_cd: &str) -> Command {
        let mut command = Command::new(CommandType::Balance);
        command.args(json!({
            "CANO" : account_no,
            "ACNT_PRDT_CD" : account_cd,
            "AFHR_FLPR_YN" : "N",
            "OFL_YN" : "",
            "INQR_DVSN" : "01",
            "UNPR_DVSN" : "01",
            "FUND_STTL_ICLD_YN" : "N",
            "FNCG_AMT_AUTO_RDPT_YN" : "N",
            "PRCS_DVSN" : "00",
            "CTX_AREA_FK100" : "",
            "CTX_AREA_NK100" : "",
        }));
        command
    }
}

impl BalanceCommand for Command {}

pub trait OrderBuyCommand {
    fn new(account_no: &str, account_cd: &str, stock_no: &str, stock_cnt: &str) -> Command {
        let mut command = Command::new(CommandType::OrderBuy);
        command.args(json!({
            "CANO" : account_no,
            "ACNT_PRDT_CD" : account_cd,
            "PDNO" : stock_no,
            "ORD_DVSN" : "01",
            "ORD_QTY" : stock_cnt,
            "ORD_UNPR" : "0",
        }));
        command
    }
}

impl OrderBuyCommand for Command {}

pub trait OrderSellCommand {
    fn new(account_no: &str, account_cd: &str, stock_no: &str, stock_cnt: &str ) -> Command {
        let mut command = Command::new(CommandType::OrderSell);
        command.args(json!({
            "CANO" : account_no,
            "ACNT_PRDT_CD" : account_cd,
            "PDNO" : stock_no,
            "ORD_DVSN" : "01",
            "ORD_QTY" : stock_cnt,
            "ORD_UNPR" : "0",
        }));
        command
    }
}

impl OrderSellCommand for Command {}
pub trait DailyValueCommand {
    fn new(stock_no: &str, period: &Period, start: &str, end: &str) -> Command {
        let mut command = Command::new(CommandType::DailyValue);
        command.args(json!({
            "fid_cond_mrkt_div_code" : "J",
            "fid_input_iscd" : stock_no,
            "FID_INPUT_DATE_1" : start,
            "FID_INPUT_DATE_2" : end,
            "fid_period_div_code" : period.as_str(),
            "fid_org_adj_prc" : "0",
        }));
        command
    }
}

impl DailyValueCommand for Command {}
