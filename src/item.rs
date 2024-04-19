use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Kana {
    pub kana: String,
    pub roumaji: String,
    score: f64,
    kanatype: String,
}

impl Kana {
    pub fn kana_chunk(&self) -> String {
        format!("{} {}\t", self.kana, self.roumaji)
    }
}
