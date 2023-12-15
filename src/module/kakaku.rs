use super::product;
use super::product::Product;
use anyhow::anyhow;
use encoding_rs::SHIFT_JIS;
use scraper::Html;
use scraper::Selector;
pub struct Kakaku {
    _urls: Vec<String>,
    _bodys: Option<Vec<Vec<u8>>>,
}

impl Kakaku {
    //スライスを引数に取る.
    pub fn new(urls: Vec<String>) -> Self {
        let sel = Self {
            _urls: urls,
            _bodys: None,
        }; //自分のインスタンスを生成.
        return sel;
    }

    pub async fn scraping(&mut self) -> anyhow::Result<()> {
        let mut bodys: Vec<Vec<u8>> = vec![];
        self._urls.retain(|url|url.contains("kakaku.com"));
        for i in self._urls.iter() {
            let body = reqwest::get(i).await?.bytes().await?;
            bodys.push(body.to_vec())
        }
        println!("{:?}", self._urls);
        self._bodys = Some(bodys);
        return Ok(());
    }

    pub fn body(&self) -> anyhow::Result<Product> {
        let Some(bodys) = &self._bodys else {
            return Err(anyhow!("エラー"));
        };
        let titles = self.element_text(bodys, "h2")?;
        let values = self.element_text(bodys, ".priceTxt")?;
        let product = product::Product::new(titles, &values, &self._urls);
        Ok(product)
    }

    pub fn element_text(&self, bodys: &Vec<Vec<u8>>, class: &str) -> anyhow::Result<Vec<String>> {
        let mut element_texts: Vec<String> = vec![];
        for body in bodys.iter() {
            let (body_str, _, _) = SHIFT_JIS.decode(body);
            let document = Html::parse_document(&body_str);
            let selector_str: String = String::from(class);
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
