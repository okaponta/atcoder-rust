use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        s:String,
    }
    let mut map = HashMap::new();
    map.insert("tourist".to_string(), 3858);
    map.insert("ksun48".to_string(), 3679);
    map.insert("Benq".to_string(), 3658);
    map.insert("Um_nik".to_string(), 3648);
    map.insert("apiad".to_string(), 3638);
    map.insert("Stonefeang".to_string(), 3630);
    map.insert("ecnerwala".to_string(), 3613);
    map.insert("mnbvmar".to_string(), 3555);
    map.insert("newbiedmy".to_string(), 3516);
    map.insert("semiexp".to_string(), 3481);
    println!("{}", map[&s]);
}
