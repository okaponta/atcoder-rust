use proconio::input;

fn main() {
    input! {
        n:usize,
        s:String,
    }
    let rev = s
        .chars()
        .rev()
        .map(|c| if c == 'd' { 'p' } else { 'd' })
        .collect::<String>();
    let mut ans = s.clone();
    for i in 0..n {
        for j in i + 1..=n {
            let new = s[0..i].to_string() + &rev[(n - j)..(n - i)] + &s[j..n];
            ans = ans.min(new);
        }
    }
    println!("{}", ans);
}
