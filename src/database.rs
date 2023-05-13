use std::collections::HashMap;
use serde::Deserialize;
use std::fs::File;

#[derive(Debug, Deserialize)]
pub struct Column {
    date : String,
    open_price : u32,
    high_price : u32,
    low_price : u32,
    close_price : u32,
    volume : u32,
    volume_cash : u64,
    performance : f32,
}

pub struct DataBase<'a> {
    db : HashMap<&'a str, Vec<Column>>,
}

impl<'a> DataBase<'a> {
    pub fn new() -> DataBase<'a> {
        DataBase {
            db : HashMap::new()
        }
    }

    fn load_data(stock_no: &str) -> Vec<Column> {
        let file_path = format!("./data/{}.csv", stock_no);
        let file = File::open(file_path).unwrap();
        let mut reader = csv::Reader::from_reader(file);
        reader.deserialize::<Column>()
            .map(Result::unwrap)
            .collect()
    }

    pub fn get_columns(&mut self, stock_no: &'a str) -> &Vec<Column> {
        let columns = self.db.entry(stock_no).or_insert_with(|| DataBase::load_data(stock_no));
        columns
    }
}
