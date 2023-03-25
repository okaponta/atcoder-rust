use proconio::input;

fn main() {
    input! {
        n: usize,
        words: [String; n],
    }
    let key_words = ["and", "not", "that", "the", "you"];
    for w in words {
        if key_words.contains(&&*w) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
