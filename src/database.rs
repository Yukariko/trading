use std::{collections::HashMap, cmp::Ordering};
use serde::Deserialize;
use std::fs::File;
use std::iter::Skip;
use std::slice::Iter;

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
}

impl DataBase {
    pub fn new() -> DataBase {
        DataBase {
            db : HashMap::new(),
            stock_list : Self::load_list(),
        }
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
    pub fn get_date_list(&mut self) -> Vec<u32> {
        let columns = Self::load_data("005930");
        let mut results : Vec<u32> = Vec::with_capacity(columns.len());
        for column in columns {
            let date = column.date.replace("-", "");
            let date_to_int = date.parse::<u32>().unwrap();
            results.push(date_to_int);
        }
        results
    }

    // search date from date_list with lower bound
    pub fn find_date(date_list: &Vec<u32>, date: u32) -> usize {
        let position = date_list
            .binary_search_by(|d| match d.cmp(&date) {
                Ordering::Equal => Ordering::Greater,
                ord => ord,
            })
            .or_else(|idx| Ok::<usize, usize>(idx))
            .unwrap();
        position
    }

    // search date from date_list with lower bound
    pub fn iter_date(date_list: &Vec<u32>, date: u32) -> Skip<Iter<u32>> {
        let position = Self::find_date(date_list, date);
        date_list.iter().skip(position)
    }

    fn load_data(stock_no: &str) -> Vec<Column> {
        let file_path = format!("./data/{}.csv", stock_no);
        let file = File::open(file_path).unwrap();
        let mut reader = csv::ReaderBuilder::new()
            .delimiter(b' ')
            .from_reader(file);
        reader.deserialize::<Column>()
            .map(Result::unwrap)
            .collect()
    }

    pub fn get_columns(&mut self, stock_no: &str) -> &Vec<Column> {
        let columns = self.db.entry(stock_no.to_owned()).or_insert_with(|| { let mut data = DataBase::load_data(stock_no); data.reverse(); data });
        columns
    }
}
