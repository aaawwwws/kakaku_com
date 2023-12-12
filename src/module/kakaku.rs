use std::str::Bytes;

use anyhow::anyhow;
use scraper::Html;
use scraper::Selector;
use encoding_rs::SHIFT_JIS;
use super::product;
use super::product::Product;
pub struct Kakaku {
    _urls: Vec<String>,
    _bodys:Option<Vec<Vec<u8>>>
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
        let mut bodys:Vec<Vec<u8>> = vec![];
        for url in self._urls.iter() {
            let body = reqwest::get(url).await?.bytes().await?;
            bodys.push(body.to_vec())
        }
        self._bodys = Some(bodys);
        return Ok(());
    }

    pub fn body (&self) -> anyhow::Result<Product> {
        let Some(bodys) = &self._bodys else{
            return Err(anyhow!("エラー"));
        };
        let titles = self.element_text(bodys,"h2")?;
        let values = self.element_text(bodys,".priceTxt")?;
        let product = product::Product::new(titles, &values, &self._urls);
        Ok(product)
    }

    pub fn element_text (&self, bodys:&Vec<Vec<u8>>, class:&str) ->anyhow::Result<Vec<String>> {
        let mut element_texts: Vec<String> = vec![];
        for body in bodys.iter(){
            let (body_str, _ ,_) = SHIFT_JIS.decode(body);
            let document = Html::parse_document(&body_str);
            let selector_str:String = String::from(class);
            let Ok(selector) = Selector::parse(&selector_str) else {
                return Err(anyhow!("エラー"));
            };
            let Some(element) = document.select(&selector).next() else {
                return Err(anyhow!("エレメントが見つかりません。"));
            };
            let value_str = element.text().next().unwrap();
            let value_string = value_str.to_string();
            element_texts.push(value_string)
        }
        println!("{:?}",element_texts);
        Ok(element_texts)
    }
}
