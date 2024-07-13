use crate::command::*;
use crate::database::DataBase;
pub mod algorithm;
use algorithm::*;
use std::collections::HashMap;

pub enum Strategy {
    Test(Account),
    PriceMomentum(Account),
}

#[derive(Clone)]
pub struct Account {
    pub account_no: String,
    pub account_cd: String,
    pub amount: i32,
    pub stocks: HashMap<String, u32>,
}

impl Account {
    pub fn new(account_no: String, account_cd: String, amount: i32) -> Self {
        Account {
            account_no,
            account_cd,
            amount,
            stocks: HashMap::new(),
        }
    }

    pub fn buy_stock(&mut self, stock_no: &str) {
        *self.stocks.entry(stock_no.to_owned()).or_insert(0) += 1;
    }
}

pub trait TestStrategyIterator {
    type Item;
    fn next(&mut self, idx: usize, account: &Account) -> Option<Self::Item>;
}

impl TestStrategyIterator for DataBase {
    type Item = Vec<Box<dyn ApiCommand>>;
    fn next(&mut self, idx: usize, account: &Account) -> Option<Self::Item> {
        let price_cmd = Command::<Price>::new()
            .ticker("005930".to_string());

        // daily price of samsung
        let daily_price_cmd = Command::<DailyPrice>::new()
            .ticker("005930".to_string())
            .period(Period::Day);
        // my balance
        let balance_cmd = Command::<Balance>::new()
            .account_no(account.account_no.clone())
            .account_cd(account.account_cd.clone());
        // buy samsung 1
        let order_buy_cmd = Command::<OrderBuy>::new()
            .account_no(account.account_no.clone())
            .account_cd(account.account_cd.clone())
            .ticker("005930".to_string())
            .count("1".to_string());
        // sell samsung 1
        let order_sell_cmd = Command::<OrderSell>::new()
            .account_no(account.account_no.clone())
            .account_cd(account.account_cd.clone())
            .ticker("005930".to_string())
            .count("1".to_string());
        Some(vec![
            Box::new(price_cmd),
            Box::new(daily_price_cmd),
            Box::new(balance_cmd),
            Box::new(order_buy_cmd),
            Box::new(order_sell_cmd)
        ])
    }
}

pub trait PriceMomentumStrategyIterator {
    type Item;
    fn next(&mut self, idx: usize, account: &mut Account) -> Option<Self::Item>;
}

impl PriceMomentumStrategyIterator for DataBase {

    type Item = Vec<Box<dyn ApiCommand>>;
    fn next(&mut self, idx: usize, account: &mut Account) -> Option<Self::Item> {

        let mut res : Vec<(f64, &str)> = Vec::new();
        for stock in &self.stock_list {
            let columns = self.get_columns(&stock).unwrap();
            if columns.len() <= idx {
                continue;
            }
            let slice = &columns[idx..];
            if let Some(momentum) = slice.get_momentum(1, 240) {
                res.push((momentum, stock));
            }
        }

        res.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let (best_momentum, best_stock) = &res[res.len() - 1];

        if idx == 0 {
            let order_buy_cmd = Command::<OrderBuy>::new()
                .account_no(account.account_no.clone())
                .account_cd(account.account_cd.clone())
                .ticker(best_stock.to_string())
                .count("1".to_string());
            return Some(vec![Box::new(order_buy_cmd)])
        }

        let columns = self.get_columns(best_stock).unwrap();
        let before_price = columns[idx - 1].open_price;
        if account.amount >= before_price {
            account.amount -= before_price;
            account.buy_stock(best_stock);
        }

        None
    }
}
