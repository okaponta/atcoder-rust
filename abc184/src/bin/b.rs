use proconio::input;

fn main() {
    input! {
      n:usize,x:i32,s:String,
    }
    let mut ans = x;
    for i in 0..n {
        if s.chars().nth(i).unwrap() == 'x' {
            if ans != 0 {
                ans -= 1;
            }
        } else {
            ans += 1;
        }
    }
    println!("{}", ans);
}
