use std::fs::File;
use std::io::prelude::*;
use std;

pub struct FileCheck {
}

impl FileCheck {
    pub fn check () -> anyhow::Result<Vec<String>> {
        let Ok(file) = File::open("url.txt") else {
            let err_msg = String::from("ファイルが見つかりません。");
            return Err(anyhow::anyhow!(err_msg));
        };
        let ite = std::io::BufReader::new(file).lines();
        let mut s_v:Vec<String> = vec![];
        for s in ite {
            s_v.push(s?);
        }
        println!("{:?}",s_v);
        return Ok(s_v);
    }
}