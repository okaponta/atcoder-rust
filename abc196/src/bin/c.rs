use proconio::input;

fn main() {
    input! {
       n:i64,
    }
    for i in 0..1000001 {
        let num: i64 = format!("{}{}", i, i).parse().unwrap();
        if num > n {
            println!("{}", i - 1);
            return;
        }
    }
}
