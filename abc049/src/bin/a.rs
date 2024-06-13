use proconio::input;

fn main() {
    input! {
        c: char,
    }
    let v = vec!['a', 'e', 'i', 'o', 'u'];
    println!("{}", if v.contains(&c) { "vowel" } else { "consonant" });
}
