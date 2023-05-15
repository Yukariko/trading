use crate::database::{DataBase, Column};
use std::collections::HashMap;

pub trait Momentum {
    fn get_momentum(&self, skip: usize, time: usize) -> Option<f64>;
}

impl Momentum for &[Column] {
    fn get_momentum(&self, skip: usize, time: usize) -> Option<f64> {
        if self.len() <= time {
            return None
        }

        let ps = f64::from(self[skip].close_price);
        let pst = f64::from(self[time].close_price);

        Some(ps / pst - 1.0)
    }
}

pub trait Etc {
    fn calc_all_cell(&mut self, stocks: &HashMap<String, u32>) -> i32;
}

impl Etc for DataBase {
    fn calc_all_cell(&mut self, stocks: &HashMap<String, u32>) -> i32 {
        let mut res = 0;
        for (stock, amount) in stocks {
            res += self.get_columns(stock).unwrap()[0].open_price * (*amount as i32)
        }
        res
    }
}
