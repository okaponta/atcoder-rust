use proconio::input;

fn main() {
    input! {
        n:usize,
        plus:i64,
        minus:i64,
        a:[i64;n],
    }
    let mut lower = 0;
    let mut upper = 1_000_000_001;
    while upper - lower > 1 {
        let med = (lower + upper) / 2;
        if is_ok(med, &a, plus, minus) {
            lower = med;
        } else {
            upper = med;
        }
    }
    println!("{}", lower);
}

fn is_ok(med: i64, a: &Vec<i64>, plus: i64, minus: i64) -> bool {
    let mut count = 0;
    for ai in a {
        if (ai - med) < 0 {
            count -= ((med - ai) + plus - 1) / plus;
        } else if 0 < (ai - med) {
            count += (ai - med) / minus;
        }
    }
    count >= 0
}
