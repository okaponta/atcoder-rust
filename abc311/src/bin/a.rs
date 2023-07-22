use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        s:Chars,
    }
    let mut flg = vec![false; 3];
    for i in 0..n {
        flg[(s[i] as u8 - b'A') as usize] = true;
        if flg.iter().all(|&b| b) {
            println!("{}", i + 1);
            return;
        }
    }
}
