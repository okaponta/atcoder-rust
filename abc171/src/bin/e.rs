use proconio::input;

fn main() {
    input! {
       n:i32,
       a:[i32;n]
    }
    let xor = a.iter().fold(0, |acc, x| acc ^ x);
    println!(
        "{}",
        a.iter()
            .map(|e| format!("{}", xor ^ e))
            .collect::<Vec<_>>()
            .join(" ")
    );
}
