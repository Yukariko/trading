use trading::Session;
use trading::command::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

struct Account {
    account_no : String,
    account_cd : String,
    ammount : u32,
}

pub struct Strategy {
    session : Session,
    account : Account,
}

impl Strategy {
    pub fn new(mut session: Session, account_no: String, account_cd: String) -> Strategy {
        Strategy {
            session : session,
            account : Account {
                account_no : account_no,
                account_cd : account_cd,
                ammount : 0
            }
        }
    }
    pub async fn run_test(&self) -> Result<()> {
        let account_no = &self.account.account_no;
        let account_cd = &self.account.account_cd;
        // samsung's price
        let price_cmd = <Command as PriceCommand>::new("005930");
        let price = self.session.execute(&price_cmd).await.expect("price cmd failed");
        println!("samsung : {}", price);

        // repeat test
        let price = self.session.execute(&price_cmd).await.expect("price cmd failed");
        println!("samsung : {}", price);

        // daily price of samsung
        let daily_price_cmd = <Command as DailyPriceCommand>::new("005930");
        let price = self.session.execute(&daily_price_cmd).await.expect("daily price cmd failed");
        println!("samsung : {}", price);

        // my balance
        let balance_cmd = <Command as BalanceCommand>::new(account_no, account_cd);
        let balance = self.session.execute(&balance_cmd).await.expect("balance cmd failed");
        println!("balance : {}", balance);

        // buy samsung 1
        let order_buy_cmd = <Command as OrderBuyCommand>::new(account_no, account_cd, "005930", "1");
        let buy = self.session.execute(&order_buy_cmd).await.expect("buy cmd failed");
        println!("buy : {}", buy);

        // sell samsung 1
        let order_sell_cmd = <Command as OrderSellCommand>::new(account_no, account_cd, "005930", "1");
        let sell = self.session.execute(&order_sell_cmd).await.expect("sell cmd failed");
        println!("buy : {}", sell);

        Ok(())
    }
}
