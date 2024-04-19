use crate::item::Kana;

use rand::thread_rng;
use rand::seq::SliceRandom;

pub fn kana_test(kana_table: &mut Vec<Kana>) {
    kana_table.shuffle(&mut thread_rng()); // TODO based on score
    for (i, item) in kana_table.iter().enumerate() {
        if i >= 20 { // TODO let user set the number of item in a test
            break;
        }
        println!("{}\tRoumaji?: ", item.kana);
        let mut ans = String::new();
        std::io::stdin().read_line(&mut ans);
        ans.pop().expect("error while getting answer");
        if ans.trim() == item.roumaji {
            println!("Correct !!");
            std::thread::sleep(std::time::Duration::from_millis(50))
            // score stuff TODO
        } else {
            println!("Incorrect!! Correct Roumaji: {}", item.roumaji);
            std::thread::sleep(std::time::Duration::from_millis(100));
            // score stuff TODO
        }
    }
}
