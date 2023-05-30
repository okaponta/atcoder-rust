use proconio::input;

fn main() {
    input! {
        a:usize,
        b:usize,
        c:usize,
        x:usize,
    }
    let mut count = 0;
    for i in 0..=a {
        for j in 0..=b {
            let tmp = 500 * i + 100 * j;
            if x < tmp || (x - tmp) % 50 != 0 {
                continue;
            }
            if (x - tmp) / 50 <= c {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
