use std::fs::File;
use polars::prelude::NamedFrom;
use polars::{
    df,
    io::{csv::CsvWriter, SerWriter},
};
use anyhow::anyhow;

pub struct DataFrame {
    _df: polars::frame::DataFrame,
}

impl DataFrame {
    pub fn new(titles: &Vec<String>, values: &Vec<u64>, urls: &Vec<String>) -> Self {
        let df = df!(
            "タイトル" => titles,
            "価格" => values,
            "url" => urls
        )
        .unwrap();
        return Self { _df: df };
    }

    pub fn create_file(&mut self) -> anyhow::Result<()> {
        let Ok(buffere) = File::create("data.csv") else {
            let error_str = String::from("ファイルの生成に失敗しました。");
            return Err(anyhow!(error_str))
        };
        let mut test = CsvWriter::new(buffere);
        let _ = test.finish(&mut self._df);
        return Ok(());
    }

    pub fn df_view (&self) {
        let value_str = self._df.column("価格").unwrap().sum::<u64>().unwrap().to_string();
        let value_str_len = value_str.len();
        let last_three = value_str.get(value_str_len - 3 .. value_str_len).unwrap();
        let remainder = value_str.get(0..value_str_len - 3).unwrap();
        let result:String = format!("{},{}",remainder,last_three);
        println!("{}",last_three);
        println!("{:?}",&self._df);
        println!("合計金額{}円",&result);
    }
}
