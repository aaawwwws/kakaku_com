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
        let i:usize = 0;
        let mut index:Vec<usize> = vec![];
        for url in self._urls.iter() {
            if !url.contains("kakaku.com") {
                println!("非対応サイトのURLを検出しました。「{}」は無視されます。",url);
                index.push(i.clone());
                continue;
            }else{
                let body = reqwest::get(url).await?.bytes().await?;
                bodys.push(body.to_vec())
            }
        }
        for delete in index.iter() {
            self._urls.remove(*delete);
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
                return Err(anyhow!("エラー".to_string()));
            };
            let Some(element) = document.select(&selector).next() else {
                let ok = String::from("elementが見つかりません。");
                return Err(anyhow!(ok));
            };
            let value_str = element.text().next().unwrap();
            let value_string = value_str.to_string();
            element_texts.push(value_string)
        }
        Ok(element_texts)
    }
}
