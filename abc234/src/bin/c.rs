use proconio::input;

fn main() {
    input! {
       mut k:i64,
    }
    let mut ans = vec![];
    while k > 0 {
        if k % 2 == 0 {
            ans.push('0');
        } else {
            ans.push('2');
        }
        k /= 2;
    }
    println!("{}", ans.iter().rev().collect::<String>());
}
