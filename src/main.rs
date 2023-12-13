mod module;
use module::{kakaku::Kakaku, calcu::Calcu, product::{Product, self}, data_frame::{DataFrame}};
use std::env;
use anyhow;
use polars::{df, io::{SerWriter, csv::CsvWriter}};
use polars::prelude::NamedFrom;
use std::fs::File;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args:Vec<String> = env::args().collect();
    let mut urls = Kakaku::new(&args[1..]); //スライス(Vecの参照を渡す).
    urls.scraping().await?;
    let product = urls.body()?;
    let mut df = DataFrame::new(product.get_titles(),product.get_values(),product.get_urls());
    df.create_file()?;
    df.df_view();
    return Ok(()); 
}
