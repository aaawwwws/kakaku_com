mod module;
use anyhow;
use module::{data_frame::DataFrame, kakaku::Kakaku,file_check};
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    const MAX_LENGTH: usize = 20;
    let urls = file_check::FileCheck::check()?;
    if  MAX_LENGTH <= urls.len() {
        return Err(anyhow::anyhow!("最大20件までです。"));
    }
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
    println!("終了する場合「end」と打ち込んでください");
    while loop_end {
        line.clear();
        std::io::stdin().read_line(&mut line).expect("error");
        loop_end = !line.trim().eq("end");
    }
    return Ok(());
}
