use crate::utils;
use std::io;
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
}

impl App {
    pub fn new() -> Self {
        Self {
            state: AppState::Home,
            is_open: true,
        }
    }
    pub fn run(&mut self) {
        while self.is_open {
            match self.state {
                AppState::Home => {
                    utils::home_menu();
                    let mut buff = String::new();
                    io::stdin()
                        .read_line(&mut buff)
                        .expect("Failed to read input");
                    let buff: i8 = match buff.trim().parse() {
                        Ok(val) => val,
                        Err(_) => -1,
                    };
                    match buff {
                        1 | 2 => todo!(),
                        // 3 | 4 => todo!(),
                        5 => self.is_open = false,
                        _ => { 
                            println!("Invalid option");
                            std::thread::sleep(std::time::Duration::from_millis(100));
                        },
                    }
                },
                _ => todo!(),
            }
        }
    }
}
