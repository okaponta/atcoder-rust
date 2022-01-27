use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    let mut ans = 0;
    let mut count = vec![0i64; 2019];
    count[0] = 1;
    let mut rem = 0; // 後ろから桁を数えてったときのmod2019
    let mut ten = 1; // 10^nのmod2019を保持しておく
    for c in s.iter().rev() {
        let cint = c.to_digit(10).unwrap() as usize;
        rem = (cint * ten + rem) % 2019;
        // i桁目以降でmod2019が同じ数だけインクリメントする
        ans += count[rem];
        count[rem] += 1;
        ten = ten * 10 % 2019;
    }
    println!("{}", ans);
}
