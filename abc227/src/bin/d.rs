use proconio::input;

fn main() {
    input! {
       n:usize,k:u128,
       a:[u128;n],
    }
    let mut lower = 0;
    let mut upper = 1 << 60;
    while upper - lower > 1 {
        let med = (lower + upper) / 2;
        if is_ok(&a, med, k) {
            lower = med;
        } else {
            upper = med;
        }
    }
    println!("{}", lower);
}

fn is_ok(a: &[u128], med: u128, k: u128) -> bool {
    a.iter().map(|i| i.min(&med)).sum::<u128>() >= med * k
}
