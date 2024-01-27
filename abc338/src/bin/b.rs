use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    let mut c = vec![0; 26];
    for s in s {
        c[s as usize - 'a' as usize] += 1;
    }
    let max = c.iter().max().unwrap();
    for i in 0..26 {
        if c[i] == *max {
            println!("{}", (b'a' + i as u8) as char);
            return;
        }
    }
}
