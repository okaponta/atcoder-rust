use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n:usize,
        m:usize,
        mut a:[usize;n],
        mut b:[usize;m],
    }
    a.sort();
    b.sort();
    let mut lower = 0;
    let mut upper = 1 << 60;
    while upper - lower > 1 {
        let med = (lower + upper) / 2;
        if is_ok(med, &a, &b) {
            upper = med;
        } else {
            lower = med;
        }
    }
    println!("{}", upper);
}

fn is_ok(med: usize, a: &Vec<usize>, b: &Vec<usize>) -> bool {
    let a = a.upper_bound(&med);
    let b = b.len() - b.lower_bound(&med);
    b <= a
}
