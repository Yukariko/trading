use trading::Session;
use trading::command::*;
pub mod algorithm;
use algorithm::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub struct Strategy {
    session : Session,
}

impl Strategy {
    pub fn new(session: Session) -> Strategy {
        Strategy {
            session : session,
        }
    }

    pub async fn run_test(&self, account_no: String, account_cd: String, ammount: u32) -> Result<()> {
        // samsung's price
        let price_cmd = <Command as PriceCommand>::new("005930");
        let price = self.session.execute(&price_cmd).await.expect("price cmd failed");
        println!("samsung : {}", price);

        // repeat test
        let price = self.session.execute(&price_cmd).await.expect("price cmd failed");
        println!("samsung : {}", price);

        // daily price of samsung
        let daily_price_cmd = <Command as DailyPriceCommand>::new("005930", &Period::Day);
        let price = self.session.execute(&daily_price_cmd).await.expect("daily price cmd failed");
        println!("samsung : {}", price);

        // my balance
        let balance_cmd = <Command as BalanceCommand>::new(&account_no, &account_cd);
        let balance = self.session.execute(&balance_cmd).await.expect("balance cmd failed");
        println!("balance : {}", balance);

        // buy samsung 1
        let order_buy_cmd = <Command as OrderBuyCommand>::new(&account_no, &account_cd, "005930", "1");
        let buy = self.session.execute(&order_buy_cmd).await.expect("buy cmd failed");
        println!("buy : {}", buy);

        // sell samsung 1
        let order_sell_cmd = <Command as OrderSellCommand>::new(&account_no, &account_cd, "005930", "1");
        let sell = self.session.execute(&order_sell_cmd).await.expect("sell cmd failed");
        println!("buy : {}", sell);

        Ok(())
    }

    pub async fn run_price_momentum(&self, account_no: String, account_cd: String, ammount: u32) -> Result<()> {
        let mut momentum = Algorithm::<dyn Momentum>::new();
        let commands = momentum.generate("005930", Period::Month);
        let res = self.session.execute_vec(&commands).await.expect("execute vec failed");
        if !momentum.parse(res) {
            return Err("parse failed".into());
        }
        if let Some(value) = momentum.get_value(1, 12) {
            println!("{}", value);
        }

        Ok(())
    }

    pub async fn run_value_momentum(&self, account_no: String, account_cd: String, ammount: u32) -> Result<()> {
        let mut momentum = Algorithm::<dyn ValueMomentum>::new();
        let commands = momentum.generate("005930", Period::Month);
        let res = self.session.execute_vec(&commands).await.expect("execute vec failed");
        if !momentum.parse(res) {
            return Err("parse failed".into());
        }
        if let Some(value) = momentum.get_value(1, 12) {
            println!("{}", value);
        }
        Ok(())
    }
}
