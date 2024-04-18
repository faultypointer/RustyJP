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
    println!("5. Exit");

    println!("\n|> ");
}
