mod module;
use module::{kakaku::Kakaku, calcu::Calcu, product::{Product, self}};
use std::env;
use anyhow;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args:Vec<String> = env::args().collect();
    let mut urls = Kakaku::new(&args[1..]); //スライス(Vecの参照を渡す).
    urls.scraping().await?;
    let vec = urls.body()?;
    println!("{:?}",&vec);
    return Ok(()); 
}
