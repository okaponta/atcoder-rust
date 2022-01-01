use proconio::input;

fn main() {
    input! {
       mut n:i64,
    }
    let mut ans = vec![];
    while n > 0 {
        n -= 1;
        ans.push(b'a' + (n % 26) as u8);
        n /= 26;
    }
    println!(
        "{}",
        ans.iter().rev().map(|&s| s as char).collect::<String>()
    );
}
