use crate::item::Kana;
use clearscreen::clear;
use std::io;


fn welcome() {
    println!(r"
         ____            _             _ ____  
        |  _ \ _   _ ___| |_ _   _    | |  _ \ 
        | |_) | | | / __| __| | | |_  | | |_) |
        |  _ <| |_| \__ \ |_| |_| | |_| |  __/ 
        |_| \_\\__,_|___/\__|\__, |\___/|_|    
                             |___/             


    ");
}

pub fn home_menu() {
    clear().unwrap();
    welcome();
    println!("1. Kana");
    println!("2. Kana Test");
    println!("3. Kanji");
    println!("4. Kanji Test");
    println!("5. Exit");
}

pub fn kana_menu() {
    clear().unwrap();
    println!("1. View Hiragana Table");
    println!("2. View Katakana Table");
    println!("3. Test");
    println!("4. Back");
}

pub fn kana_test_menu() {
    clear().unwrap();
    println!("1. Hiragana");
    println!("2. Katakana");
    println!("3. Both");
    println!("4. Back");
}

pub fn menu_input() -> i8 {
    println!("\n|> ");
    let mut buff = String::new();
    io::stdin()
        .read_line(&mut buff)
        .expect("Failed to read input");
    buff.trim().parse().unwrap_or(-1) 
}

pub fn invalid_input() {
    println!("Invalid option");
    std::thread::sleep(std::time::Duration::from_millis(100));
}

pub fn kana_table(hiragana: &Vec<Kana>) {
    for (i, item) in hiragana.iter().enumerate() {
        print!("{}", item.kana_chunk());
        if (i+1) %  5 == 0 {
            println!("");
        }
    }

    println!("\n\nPress any key to go back.");
    _ = menu_input();
}
