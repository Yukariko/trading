use crate::strategy::*;
use crate::strategy::algorithm::Etc;
use crate::Session;
use crate::database::DataBase;

pub struct TimeRunner {
    session: Session,
    database: DataBase,
    date_list : Vec<u32>,
}

impl TimeRunner {
    pub fn new(session: Session) -> TimeRunner {
        let mut database = DataBase::new();
        let date_list = database.get_date_list();

        TimeRunner {
            session,
            database,
            date_list,
        }
    }

    pub fn run_back_test(&mut self, start_date: u32, end_date: u32, mut strategies: Vec<Strategy>) {
        let start_idx = DataBase::find_date(&self.date_list, start_date);
        let end_idx = DataBase::find_date(&self.date_list, end_date);
        for idx in start_idx..=end_idx {
            let idx = self.date_list.len() - idx - 1;
            for iter in &mut strategies {
                let res = match iter {
                    Strategy::Test(account) => <DataBase as TestStrategyIterator>::next(&mut self.database, idx, account),
                    Strategy::PriceMomentum(ref mut account) => <DataBase as PriceMomentumStrategyIterator>::next(&mut self.database, idx, account),
                };

                if let Some(commands) = res {

                }
            }
        }

        for iter in &mut strategies {
            let account = match iter {
                Strategy::Test(account) => account,
                Strategy::PriceMomentum(account) => account
            };
            let stock_to_cash = self.database.calc_all_cell(&account.stocks);
            println!("stocks : {}, amount : {}, Total : {}", stock_to_cash, account.amount, stock_to_cash + account.amount);
        }
    }
}
