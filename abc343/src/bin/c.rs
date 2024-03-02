use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    let mut max = 1;
    let mut i = 1;
    while i * i * i <= n {
        let k = i * i * i;
        let s = k.to_string();
        let t = s.chars().rev().collect::<String>();
        if s == t {
            max = k;
        }
        i += 1;
    }
    println!("{}", max);
}
