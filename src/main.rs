use dotenv::dotenv;
use std::env;
use trading::Session;
use trading::time_runner::TimeRunner;
use trading::strategy::*;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let app_key = env::var("APP_KEY").expect("APP_KEY must be set.");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET must be set.");
    let domain = env::var("DOMAIN").expect("DOMAIN must be set");
    let session = Session::new(app_key, app_secret, domain).await.expect("create session failed");

    let mut runner = TimeRunner::new(session);

    let account_no = env::var("ACCOUNT_NO").expect("ACCOUNT_NO must be set");
    let account_cd = env::var("ACCOUNT_CD").expect("ACCOUNT_CD must be set");

    let account = Account::new(account_no, account_cd, 2500000);

    let strategies = vec![Strategy::Test(account.clone()), Strategy::PriceMomentum(account.clone())];

    runner.run_back_test(20230101, 20230512, strategies);
}
