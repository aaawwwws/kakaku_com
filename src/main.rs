mod module;
use module::{kakaku::Kakaku, calcu::Calcu, product::{Product, self}};
use std::env;
use anyhow;
use polars::df;
use polars::prelude::NamedFrom;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args:Vec<String> = env::args().collect();
    let mut urls = Kakaku::new(&args[1..]); //スライス(Vecの参照を渡す).
    urls.scraping().await?;
    let product = urls.body()?;
    let df = df!(
        "タイトル" => product.get_titles(),
        "価格" => product.get_values(),
        "url" => product.get_urls()
    ).unwrap();
    println!("{:?}",&df);
    println!("合計金額{:?}円",df.column("価格").unwrap().sum::<u64>().unwrap());
    return Ok(()); 
}
