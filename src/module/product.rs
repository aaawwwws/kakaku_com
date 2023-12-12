use super::calcu::Calcu;
#[derive(Debug)]
pub struct Product {
    _titles: Vec<String>,
    _values: Vec<u64>,
    _urls: Vec<String>,
    _total_value:u64
}

impl Product {
    pub fn new(titles: Vec<String>, values: &Vec<String>, urls: &Vec<String>) -> Self {
        let mut values_u64: Vec<u64> = vec![];
        for values in values.iter() {
            let mut value_string = values.to_string();
            value_string.remove(0);
            value_string.retain(|c| c != ',');
            let value_int: u64 = value_string.parse::<u64>().unwrap();
            values_u64.push(value_int);
        }
        let calc = Calcu::calcu(values_u64.clone());
        return Self {
            _titles: titles,
            _values: values_u64,
            _urls: urls.to_vec(),
            _total_value:calc
        };
    }
    pub fn get_titles (&self) -> &Vec<String> {
        &self._titles
    }
    pub fn get_values (&self) -> &Vec<u64> {
        &self._values
    }
    pub fn get_urls (&self) -> &Vec<String> {
        &self._urls
    }
}
