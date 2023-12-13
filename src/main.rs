mod module;
use anyhow;
use module::{data_frame::DataFrame, kakaku::Kakaku,file_check};
use std::{env, fs::File};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let urls = file_check::FileCheck::check()?;
    let mut urls = Kakaku::new(&urls[0..]); //スライス(Vecの参照を渡す).
    urls.scraping().await?;
    let product = urls.body()?;
    let mut df = DataFrame::new(
        product.get_titles(),
        product.get_values(),
        product.get_urls(),
    );
    df.create_file()?;
    df.df_view();
    let mut line = String::new();
    let mut loop_end = true;
    while loop_end {
        line.clear();
        std::io::stdin().read_line(&mut line).expect("error");
        loop_end = !line.trim().eq("end");
        println!("{}", &line);
    }
    return Ok(());
}
