use proconio::input;
use superslice::Ext;
fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [i64;n],
    }
    a.sort();
    let mut upper = 10i64.pow(18) + 1;
    let mut lower = -upper;
    while upper - lower > 1 {
        let med = (lower + upper) / 2;
        if is_med_over(&a, med, n, k) {
            lower = med;
        } else {
            upper = med;
        }
    }
    println!("{}", upper);
}

fn is_med_over(a: &Vec<i64>, med: i64, n: usize, k: usize) -> bool {
    if med >= 0 {
        return is_med_over_pos(a, med, n, k);
    }
    return is_med_over_neg(a, med, n, k);
}

fn is_med_over_pos(a: &Vec<i64>, med: i64, n: usize, k: usize) -> bool {
    let mut count = 0;
    for (i, &ai) in a.iter().enumerate() {
        let next_i = i + 1;
        if ai == 0 {
            count += n - next_i;
        }
        if ai > 0 {
            count += a[next_i..].upper_bound(&(med / ai));
        }
        if ai < 0 {
            count += n - next_i - a[i + 1..].lower_bound(&(med / ai));
        }
    }
    count < k
}

fn is_med_over_neg(a: &Vec<i64>, med: i64, n: usize, k: usize) -> bool {
    let mut count = 0;
    for (i, &ai) in a.iter().enumerate() {
        let next_i = i + 1;
        if ai == 0 {
            continue;
        }
        if ai > 0 {
            count += a[next_i..].upper_bound(&((med - ai + 1) / ai));
        }
        if ai < 0 {
            count += n - next_i - a[i + 1..].lower_bound(&((med + ai + 1) / ai));
        }
    }
    count < k
}
