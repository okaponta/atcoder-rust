use proconio::input;

fn main() {
    input! {
      _:usize,x:u32,s:String,
    }
    let mut ans = x;
    for c in s.chars() {
        if c == 'x' {
            ans = ans.saturating_sub(1);
        } else {
            ans += 1;
        }
    }
    println!("{}", ans);
}
