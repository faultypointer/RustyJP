use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug)]
pub struct Kana {
    kana: String,
    roumaji: String,
    score: f64,
    kanatype: String,
}

impl Kana {
    pub fn kana_chunk(&self) -> String {
        format!("{} {}\t", self.kana, self.roumaji)
    }
}
