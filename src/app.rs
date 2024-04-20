use crate::utils;
use crate::item::{ Kana, Item };
use crate::item_test;
use std::fs::File;
use std::io::{ BufReader };

pub enum AppState {
    Home,
    Kana,
    KanaTest,
    // Kanji,
    // KanjiTest
}


pub struct App {
    state: AppState,
    is_open: bool,
    hiragana: Vec<Kana>,
    katakana: Vec<Kana>,
}

impl App {
    pub fn new() -> Self {
        let hira_file = File::open("items/hiragana.json").expect("Failed to open hiragana json file"); 
        let kata_file = File::open("items/katakana.json").expect("Failed to open katakana json file");
        let hira_reader = BufReader::new(hira_file);
        let kata_reader = BufReader::new(kata_file);
        Self {
            state: AppState::Home,
            is_open: true,
            hiragana: serde_json::from_reader(hira_reader).expect("Error while parsing hiragana json"),
            katakana: serde_json::from_reader(kata_reader).expect("Error while parsing katakana json"),
        }
    }
    pub fn run(&mut self) {
        while self.is_open {
            match self.state {
                AppState::Home => {
                    utils::home_menu();
                    match utils::menu_input() {
                        1 => self.state = AppState::Kana,
                        2 => self.state = AppState::KanaTest,
                        // 3 | 4 => todo!(),
                        0 => self.is_open = false,
                        _ => utils::invalid_input(),
                    }
                },
                AppState::Kana => {
                    utils::kana_menu();
                    match utils::menu_input() {
                        1 => utils::print_kana_table(&self.hiragana),
                        2 => utils::print_kana_table(&self.katakana),
                        3 => self.state = AppState::KanaTest,
                        4 => utils::kana_score(&mut self.hiragana),
                        5 => utils::kana_score(&mut self.katakana),
                        0 => self.state = AppState::Home,
                        _ => utils::invalid_input(),
                    }
                },
                AppState::KanaTest => {
                    utils::kana_test_menu();
                    match utils::menu_input() {
                        1 => {
                            item_test::kana_test(&mut self.hiragana);
                            utils::write_json(&self.hiragana, Item::Hira);
                        },
                        2 => {
                            item_test::kana_test(&mut self.katakana);
                            utils::write_json(&self.katakana, Item::Kata);
                        },
                        3 => {
                            let mut both = self.hiragana.clone();
                            both.append(&mut self.katakana.clone());
                            item_test::kana_test(&mut both);
                            utils::write_json(&self.hiragana, Item::Hira);
                            utils::write_json(&self.katakana, Item::Kata);
                        },
                        0 => self.state = AppState::Home,
                        _ => utils::invalid_input(),
                    }
                },
                // _ => todo!(),
            }
        }
    }
}
