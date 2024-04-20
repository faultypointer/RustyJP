use std::cmp::Ordering;
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Kana {
    pub kana: String,
    pub roumaji: String,
    pub score: f64,
    kanatype: String,
}

pub enum Item {
    Hira,
    Kata,
    // Kanji, TODO
}

impl Kana {
    pub fn kana_chunk(&self) -> String {
        format!("{} {}\t", self.kana, self.roumaji)
    }
}

impl Ord for Kana {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.partial_cmp(&other.score).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for Kana {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Kana {}
