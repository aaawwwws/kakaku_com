use anyhow::{anyhow};
use reqwest::Body;
use scraper::Html;
use scraper::Selector;
use core::iter;
pub struct Kakaku {
    _urls: Vec<String>,
    _bodys:Option<Vec<String>>
}

impl Kakaku {
    //スライスを引数に取る.
    pub fn new(url: &[String]) -> Self {
        let sel = Self {
            _urls: url.to_vec(),
            _bodys: None
        }; //自分のインスタンスを生成.
        return sel;
    }

    pub async fn scraping(&mut self) -> anyhow::Result<()> {
        let mut bodys:Vec<String> = vec![];
        for url in self._urls.iter() {
            let body = reqwest::get(url).await?.text().await?;
            bodys.push(body)
        }
        self._bodys = Some(bodys);
        return Ok(());
    }

    pub fn parser (&self) -> anyhow::Result<()> {
        let mut veec:Vec<String> = vec![];
        let Some(bodys) = &self._bodys else{
            return Err(anyhow!("エラー"));
        };
        for body in bodys.iter(){
            let document = Html::parse_document(body);
            let selector_str:String = String::from(".priceTxt");
            let Ok(selector) = Selector::parse(&selector_str) else {
                return Err(anyhow!("エラー"));
            };
            let Some(element) = document.select(&selector).next() else {
                return Err(anyhow!("エレメントが見つかりません。"));
            };
            let value_str = element.text().next().unwrap();
            let mut value_string = value_str.to_string();
            value_string.remove(0);
            value_string.retain(|c| c != ',');
            let value_int:u64 = value_string.parse::<u64>().unwrap();
            println!("{}",&value_int);
        }
        Ok(())
    }
}
