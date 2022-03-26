use proconio::input;

fn main() {
    input! {
        n:usize,k:i64,
        a:[i64;n],
        b:[i64;n],
    }
    let mut prev1 = a[0];
    let mut prev2 = b[0];
    for i in 1..n {
        let mut next1 = -1;
        let mut next2 = -1;
        if ((prev1 - a[i]).abs()).min((prev2 - a[i]).abs()) <= k {
            next1 = a[i];
        }
        if ((prev1 - b[i]).abs()).min((prev2 - b[i]).abs()) <= k {
            next2 = b[i];
        }
        if next1 == -1 && next2 == -1 {
            println!("No");
            return;
        }
        prev1 = next1;
        prev2 = next2;
    }
    println!("Yes");
}
