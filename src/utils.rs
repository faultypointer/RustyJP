use crate::item::{ Item, ItemType, Kana, Kanji};
use std::fs::File;
use std::io::BufWriter;
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



pub fn kanji_menu() {
    clear().unwrap();
    println!("1. View Kanji");
    println!("2. Add Kanji");
    println!("3. Remove Kanji");
    println!("4. Kanji Test");
    println!("5. View Kanji Score");
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

pub fn print_kanji_table(kanji_table: &[Kanji]) {
    for (i, item) in kanji_table.iter().enumerate() {
        println!("\n{}: {}", i+1, item.kanji);
        print!("onyomi: ");
        for onyo in item.onyomi.iter() {
            print!("{} ", onyo);
        }
        print!("\nkunyomi: ");
        for kunyo in item.kunyomi.iter() {
            print!("{} ", kunyo);
        }
        print!("\nmeanings: ");
        for mean in item.meanings.iter() {
            print!("{} ", mean);
        }
        println!();
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

pub fn add_kanji(kanji_table: &mut Vec<Kanji>) {
    // Kanji bn
    loop { 
    let mut kanji = String::new();
    let mut on: Vec<String> = Vec::new();
    let mut kun: Vec<String> = Vec::new();
    let mut mean: Vec<String> = Vec::new();
    let mut sent: Vec<String> = Vec::new();
    println!("Enter the kanji:");
    io::stdin().read_line(&mut kanji).expect("Error reading input");
    kanji = String::from(kanji.trim());
    if kanji.is_empty() {
        break;
    }
    println!("Enter the onyomi readings");
    get_vec_string_input(&mut on);
    println!("Enter the kunyomi readings");
    get_vec_string_input(&mut kun);
    println!("Enter the meanings");
    get_vec_string_input(&mut mean);
    println!("Enter the sentences with the kanji: ");
    get_vec_string_input(&mut sent);
    kanji_table.push( Kanji {
        kanji,
        onyomi: on,
        kunyomi: kun,
        meanings: mean,
        sentences: sent,
        score: -10.0,
    });
    }
}

fn get_vec_string_input(vector: &mut Vec<String>) {
    loop {
        let mut buff = String::new();
        io::stdin().read_line(&mut buff).expect("Error reading input");
        buff = String::from(buff.trim());
        if buff.is_empty() {
            break;
        }
        vector.push(buff);
    }

}

pub fn write_json<T>(kana_table: &Vec<T>, item: ItemType)
where T : Item + serde::Serialize,
{
    let file = match item {
        ItemType::Hira => File::create("items/hiragana.json").expect("failed to open hiragana file for writing"),
        ItemType::Kata => File::create("items/katakana.json").expect("failed to open katakana file for writing"),
        ItemType::Kanji => File::create("items/kanji.json").expect("failed to open kanji file for writing"),
    };
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, kana_table).expect("failed to write to json");
}

//
// pub fn json_in_kanji_dir() -> io::Result<Vec<String>> {
//     let mut files = Vec::new();
//     for entry in fs::read_dir("items/kanji")? {
//         let entry = entry?;
//         let path = entry.path();
//         if path.is_file() {
//             files.push(path.into_os_string().into_string().unwrap());
//         }
//     }
//     Ok(files)
// }
