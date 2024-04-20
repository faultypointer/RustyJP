use crate::item::{ Kana, Item };
use std::fs::File;
use std::io::{ BufWriter };
use std::io;
use clearscreen::clear;


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
    println!("0. Exit");
}

pub fn kana_menu() {
    clear().unwrap();
    println!("1. View Hiragana Table");
    println!("2. View Katakana Table");
    println!("3. Test");
    println!("4. View Hiragana Score");
    println!("5. View katakana Score");
    println!("0. Back");
}

pub fn kana_test_menu() {
    clear().unwrap();
    println!("1. Hiragana");
    println!("2. Katakana");
    println!("3. Both");
    println!("0. Back");
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

pub fn stall() {
    println!("\n\nPress any key to go back.");
    _ = menu_input();
}

pub fn print_kana_table(kana_table: &[Kana]) {
    for (i, item) in kana_table.iter().enumerate() {
        print!("{}", item.kana_chunk());
        if (i+1) %  5 == 0 {
            println!();
        }
    }
    stall();

}

pub fn kana_score(kana_table: &mut [Kana]) {
    kana_table.sort();
    for item in kana_table.iter() {
        println!("{} {}\tScore: {:.2}", item.kana, item.roumaji, item.score);
    }
    stall();
}


pub fn input_test_size() -> u8 {
    println!("Enter the number of items for this test:");
    println!("|> ");
    let mut buff = String::new();
    io::stdin()
        .read_line(&mut buff)
        .expect("Failed to read test size");
    match buff.trim().parse() {
        Ok(val) => val,
        Err(_) => {
            println!("Invalid numbe entered. used default 20");
            std::thread::sleep(std::time::Duration::from_millis(100));
            20
        },
    }
}

pub fn write_json(kana_table: &Vec<Kana>, item: Item) {
    let file = match item {
        Item::Hira => File::create("items/hiragana.json").expect("failed to open hiragana file for writing"),
        Item::Kata => File::create("items/katakana.json").expect("failed to open katakana file for writing"),
    };
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, kana_table).expect("failed to write to json");
}
