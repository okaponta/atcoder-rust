use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
       n:u64,
    }
    let mut count = 0;
    let mut amax: u64 = 1;
    while amax * amax * amax <= n {
        amax += 1;
    }
    for a in 1..=amax {
        let bmax = (n / a).sqrt();
        for b in a..=bmax {
            count += (n / (a * b)) - b + 1;
        }
    }
    println!("{}", count);
}
