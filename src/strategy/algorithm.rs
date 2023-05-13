use crate::command::*;
use std::marker::PhantomData;

pub struct Algorithm<T: ?Sized> {
    kind: PhantomData<T>,
    results : Option<Vec<serde_json::Value>>,
}

impl<T: ?Sized> Algorithm<T> {
    pub fn new() -> Algorithm<T> {
        Algorithm {
            kind: PhantomData,
            results: None,
        }
    }

    pub fn parse(&mut self, results: Vec<serde_json::Value>) -> bool {
        for value in &results {
            let object = value.as_object().unwrap();
            if object["rt_cd"].as_str().unwrap() != "0" {
                return false;
            }
        }
        self.results = Some(results);
        true
    }
}

pub trait Momentum {
    fn generate(&self, stock_no: &str, period: Period) -> Vec<Command>;

    fn get_value(&self, skip: usize, time: usize) -> Option<f64>;
}

impl Momentum for Algorithm<dyn Momentum> {
    fn generate(&self, stock_no: &str, period: Period) -> Vec<Command> {
        let daily_price_cmd = <Command as DailyPriceCommand>::new(stock_no, &period);
        vec!(daily_price_cmd)
    }

    fn get_value(&self, skip: usize, time: usize) -> Option<f64> {
        if skip >= time || skip + time >= 30 {
            return None
        }
        if let Some(results) = &self.results {
            let object = results[0].as_object().unwrap();
            let arr = object["output"].as_array().unwrap();
            if arr.len() < skip + time {
                return None
            }

            let ps = arr[skip]["stck_clpr"].as_str().unwrap().parse::<f64>().unwrap();
            let pst  = arr[skip+time]["stck_clpr"].as_str().unwrap().parse::<f64>().unwrap();

            Some(ps / pst - 1.0)
        } else {
            None
        }
    }
}

pub trait ValueMomentum {
    fn generate(&self, stock_no: &str, period: Period) -> Vec<Command>;

    fn get_value(&self, skip: usize, time: usize) -> Option<f64>;
}

impl ValueMomentum for Algorithm<dyn ValueMomentum> {
    fn generate(&self, stock_no: &str, period: Period) -> Vec<Command> {
        let daily_value_cmd = <Command as DailyValueCommand>::new(stock_no, &period, "20220101", "20220131");
        let daily_value_cmd2 = <Command as DailyValueCommand>::new(stock_no, &period, "20230101", "20230131");
        vec!(daily_value_cmd, daily_value_cmd2)
    }

    fn get_value(&self, skip: usize, time: usize) -> Option<f64> {
        if let Some(results) = &self.results {
            let object = results[0].as_object().unwrap();
            let object2 = results[1].as_object().unwrap();
            let ps = object["output1"]["per"].as_str().unwrap().parse::<f64>().unwrap();
            let pst = object2["output1"]["per"].as_str().unwrap().parse::<f64>().unwrap();

            Some(ps / pst - 1.0)
        } else {
            None
        }
    }

}
