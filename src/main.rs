use dotenv::dotenv;
use std::env;
use trading::Session;
use trading::command::*;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let app_key = env::var("APP_KEY").expect("APP_KEY must be set.");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET must be set.");
    let domain = env::var("DOMAIN").expect("DOMAIN must be set");
    let session = Session::new(app_key, app_secret, domain).await.expect("create session failed");

    // samsung's price
    let price_cmd = <Command as PriceCommand>::new("005930");
    let price = session.execute(&price_cmd).await.expect("price cmd failed");
    println!("samsung : {}", price);

    // repeat test
    let price = session.execute(&price_cmd).await.expect("price cmd failed");
    println!("samsung : {}", price);

    // daily price of samsung
    let daily_price_cmd = <Command as DailyPriceCommand>::new("005930");
    let price = session.execute(&daily_price_cmd).await.expect("daily price cmd failed");
    println!("samsung : {}", price);

    // my balance
    let account_no = env::var("ACCOUNT_NO").expect("ACCOUNT_NO must be set");
    let account_cd = env::var("ACCOUNT_CD").expect("ACCOUNT_CD must be set");
    let balance_cmd = <Command as BalanceCommand>::new(&account_no, &account_cd);
    let balance = session.execute(&balance_cmd).await.expect("balance cmd failed");
    println!("balance : {}", balance);

    // buy samsung 1
    let order_buy_cmd = <Command as OrderBuyCommand>::new(&account_no, &account_cd, "005930", "1");
    let buy = session.execute(&order_buy_cmd).await.expect("buy cmd failed");
    println!("buy : {}", buy);

    // sell samsung 1
    let order_sell_cmd = <Command as OrderSellCommand>::new(&account_no, &account_cd, "005930", "1");
    let sell = session.execute(&order_sell_cmd).await.expect("sell cmd failed");
    println!("buy : {}", sell);
}
