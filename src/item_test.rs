use crate::item::Kana;
use crate::utils;
use rand::Rng;

pub fn kana_test(kana_table: &mut [Kana]) {
    let test_size = utils::input_test_size();
    kana_table.sort();
    for (i, item) in kana_table.iter_mut().enumerate() {
        if i >= test_size.into() { // TODO let user set the number of item in a test
            break;
        }
        println!("{}\tRoumaji?: ", item.kana);
        let mut ans = String::new();
        std::io::stdin().read_line(&mut ans).expect("Failed to read answer");
        ans.pop().expect("error while getting answer");
        if ans.trim() == item.roumaji {
            item.score += (item.score.abs() * 0.2) + rand::thread_rng().gen_range(0.0..5.0);
            println!("Correct !!");
            std::thread::sleep(std::time::Duration::from_millis(50))
        } else {
            item.score -= (item.score.abs() * 0.2) + rand::thread_rng().gen_range(0.0..5.0);

            println!("Incorrect!! Correct Roumaji: {}", item.roumaji);
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
}
