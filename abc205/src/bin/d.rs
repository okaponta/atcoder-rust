use proconio::input;

fn main() {
    input! {
       n:i64,q:i64,
       a:[i64;n],
       k:[i64;q],
    }
    for ki in k {
        if ki < a[0] {
            println!("{}", ki);
            continue;
        }
        let mut l = 0;
        let mut r = n;
        while r - l > 1 {
            let m = (l + r) / 2;
            if a[m as usize] < ki + m + 1 {
                l = m
            } else {
                r = m
            }
        }
        println!("{}", ki + l + 1);
    }
}
