use crate::command::*;
use crate::database::DataBase;
pub mod algorithm;
use algorithm::*;

pub enum Strategy {
    Test(Account),
    PriceMomentum(Account),
}

#[derive(Clone)]
pub struct Account {
    pub account_no: String,
    pub account_cd: String,
    pub amount: i32,
}

pub trait TestStrategyIterator {
    type Item;
    fn next(&mut self, idx: usize, account: &Account) -> Option<Self::Item>;
}

impl TestStrategyIterator for DataBase {
    type Item = Vec<Command>;
    fn next(&mut self, idx: usize, account: &Account) -> Option<Self::Item> {
        let price_cmd = <Command as PriceCommand>::new("005930");
        // daily price of samsung
        let daily_price_cmd = <Command as DailyPriceCommand>::new("005930", &Period::Day);
        // my balance
        let balance_cmd = <Command as BalanceCommand>::new(&account.account_no, &account.account_cd);
        // buy samsung 1
        let order_buy_cmd = <Command as OrderBuyCommand>::new(&account.account_no, &account.account_cd, "005930", "1");
        // sell samsung 1
        let order_sell_cmd = <Command as OrderSellCommand>::new(&account.account_no, &account.account_cd, "005930", "1");
        Some(vec![price_cmd, daily_price_cmd, balance_cmd, order_buy_cmd, order_sell_cmd])
    }
}

pub trait PriceMomentumStrategyIterator {
    type Item;
    fn next(&mut self, idx: usize, account: &mut Account) -> Option<Self::Item>;
}

impl PriceMomentumStrategyIterator for DataBase {

    type Item = Vec<Command>;
    fn next(&mut self, idx: usize, account: &mut Account) -> Option<Self::Item> {

        let mut res : Vec<(f64, String)> = Vec::new();
        for i in 0..self.stock_list.len() {
            let stock = self.stock_list[i].clone();
            let columns = self.get_columns(&stock);
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
        let columns = self.get_columns(best_stock);
        let before_price = columns[idx].close_price;
        let before_date = &columns[idx].date;
        let current_price = columns[0].close_price;

        println!("{} {} : {} {} -> {} ({})", best_stock, best_momentum, before_date, before_price, current_price, current_price - before_price);
        account.amount += current_price;
        account.amount -= before_price;
        println!("{}", account.amount);
        None
    }
}
