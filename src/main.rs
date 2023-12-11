mod module;
use module::kakaku::Kakaku;
use std::env;
use anyhow;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args:Vec<String> = env::args().collect();
    let mut urls = Kakaku::new(&args[1..]); //スライス(Vecの参照を渡す).
    urls.scraping().await?;
    urls.parser()?;
    return Ok(()); 
}
