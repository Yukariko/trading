use std::{collections::HashMap, cmp::Ordering};
use serde::Deserialize;
use std::fs::File;

#[derive(Debug, Deserialize)]
pub struct Column {
    pub date : String,
    pub open_price : i32,
    pub high_price : i32,
    pub low_price : i32,
    pub close_price : i32,
    pub volume : u32,
    pub amount : u64,
    pub performance : f32,
}

pub struct DataBase {
    db : HashMap<String, Vec<Column>>,
    pub stock_list : Vec<String>,
    pub date_list : Vec<u32>,
}

impl DataBase {
    pub fn new() -> DataBase {
        let stock_list = Self::load_list();
        let mut db : HashMap<String, Vec<Column>> = HashMap::new();
        for stock in &stock_list {
            let column = Self::load_data(stock);
            db.insert(stock.to_owned(), column);
        }

        let mut database = DataBase {
            db,
            stock_list,
            date_list: Vec::new(),
        };

        database.date_list = database.load_date_list();

        database
    }

    fn load_list() -> Vec<String> {
        let file_path = "./data/list.csv";
        let file = File::open(file_path).unwrap();
        let mut reader = csv::Reader::from_reader(file);
        let mut records : Vec<String> = Vec::new();
        for record in reader.records() {
            let record = record.unwrap();
            if record.len() > 0 {
                records.push(record[0].to_string());
            }
        }
        records
    }

    // must be fix
    fn load_date_list(&self) -> Vec<u32> {
        let columns = self.get_columns("005930").unwrap();
        let mut results : Vec<u32> = Vec::with_capacity(columns.len());
        for column in columns.iter().rev() {
            let date = column.date.replace("-", "");
            let date_to_int = date.parse::<u32>().unwrap();
            results.push(date_to_int);
        }
        results
    }

    fn load_data(stock_no: &str) -> Vec<Column> {
        let file_path = format!("./data/{}.csv", stock_no);
        let file = File::open(file_path).unwrap();
        let mut reader = csv::ReaderBuilder::new()
            .delimiter(b' ')
            .from_reader(file);
        let mut columns : Vec<Column> = reader.deserialize::<Column>()
            .map(Result::unwrap)
            .collect();
        columns.reverse();
        columns
    }


    // search date from date_list with lower bound
    pub fn idx_from_date(&self, date: u32) -> usize {
        let position = self.date_list
            .binary_search_by(|d| match d.cmp(&date) {
                Ordering::Equal => Ordering::Greater,
                ord => ord,
            })
            .or_else(|idx| Ok::<usize, usize>(idx))
            .unwrap();
        position
    }

    pub fn get_columns(&self, stock_no: &str) -> Option<&Vec<Column>> {
        let columns = self.db.get(stock_no);
        columns
    }
}
