use proconio::input;

fn main() {
    input! {
       n:i64,k:i64,
       a:[i64;n],
    }
    let base = k / n;
    let remain = k % n;
    let mut ac = a.clone();
    ac.sort();
    let a_small = ac.iter().nth(remain as usize).unwrap();
    for a in a {
        if &a < a_small {
            println!("{}", base + 1);
        } else {
            println!("{}", base);
        }
    }
}
