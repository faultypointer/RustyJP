use std::cmp::Ordering;
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Kana {
    pub kana: String,
    pub roumaji: String,
    pub score: f64,
    kanatype: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Kanji {
    pub kanji: String,
    pub onyomi: Vec<String>,
    pub kunyomi: Vec<String>,
    pub meanings: Vec<String>,
    pub sentences: Vec<String>,
    pub score: f64,
}


pub enum ItemType {
    Hira,
    Kata,
    Kanji,
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

impl Ord for Kanji {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.partial_cmp(&other.score).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for Kanji {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


impl Eq for Kanji {}




pub trait Item {}
impl Item for Kana {}
impl Item for Kanji {}
