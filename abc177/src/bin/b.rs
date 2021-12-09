use proconio::input;

fn main() {
    input! {
       s:String,t:String,
    }
    let mut ans = 10000;
    for i in 0..=s.len() - t.len() {
        let mut count = 0;
        for j in 0..t.len() {
            if s.chars().nth(i + j).unwrap() == t.chars().nth(j).unwrap() {
                count += 1;
            }
        }
        ans = ans.min(t.len() - count);
    }
    println!("{}", ans);
}
