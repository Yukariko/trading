use crate::command::*;
use crate::database::DataBase;
pub mod algorithm;
use algorithm::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub type StrategyIterator<'a> = dyn Iterator<Item=Vec<Command>> + 'a;

#[derive(Clone)]
pub struct Account {
    pub account_no: String,
    pub account_cd: String,
    pub ammount: u32,
}

pub trait Strategy {
    fn new(account: Account) -> Box<dyn Strategy> where Self: Sized;
    fn iter<'a>(&'a self, date: u32, database: &'a DataBase) -> Box<StrategyIterator>;
}

pub struct TestStrategy {
    account: Account,
}

impl Strategy for TestStrategy {
    fn new(account: Account) -> Box<dyn Strategy> {
        Box::new(TestStrategy {
            account: account
        })
    }

    fn iter<'a>(&'a self, date: u32, database: &'a DataBase) -> Box<StrategyIterator> {
        Box::new(TestStrategyIter {
            date: date,
            account: self.account.clone(),
            database: database,
            strategy: self,
        })
    }
}

pub struct TestStrategyIter<'a> {
    date: u32,
    account: Account,
    database: &'a DataBase<'a>,
    strategy: &'a TestStrategy,
}

impl Iterator for TestStrategyIter<'_> {
    type Item = Vec<Command>;

    fn next(&mut self) -> Option<Self::Item> {
        let price_cmd = <Command as PriceCommand>::new("005930");
        // daily price of samsung
        let daily_price_cmd = <Command as DailyPriceCommand>::new("005930", &Period::Day);
        // my balance
        let balance_cmd = <Command as BalanceCommand>::new(&self.account.account_no, &self.account.account_cd);
        // buy samsung 1
        let order_buy_cmd = <Command as OrderBuyCommand>::new(&self.account.account_no,
                                                              &self.account.account_cd, "005930", "1");
        // sell samsung 1
        let order_sell_cmd = <Command as OrderSellCommand>::new(&self.account.account_no,
                                                                &self.account.account_cd, "005930", "1");
        Some(vec![price_cmd, daily_price_cmd, balance_cmd, order_buy_cmd, order_sell_cmd])
    }
}

pub struct PriceMomentumStrategy {
    account: Account,
}

impl Strategy for PriceMomentumStrategy {
    fn new(account: Account) -> Box<dyn Strategy> {
        Box::new(Self {
            account: account
        })
    }

    fn iter<'a>(&'a self, date: u32, database: &'a DataBase) -> Box<StrategyIterator> {
        Box::new(PriceMomentumStrategyIter {
            date: date,
            account: self.account.clone(),
            database: database,
            strategy: self,
        })
    }
}

pub struct PriceMomentumStrategyIter<'a> {
    date: u32,
    account: Account,
    database: &'a DataBase<'a>,
    strategy: &'a PriceMomentumStrategy,
}

impl Iterator for PriceMomentumStrategyIter<'_> {
    type Item = Vec<Command>;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

