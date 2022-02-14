use proconio::input;

fn main() {
    input! {
       n:usize,k:usize,
       a:[usize;n],
    }
    let mut lower = 0;
    let mut upper = 1 << 60;
    while upper - lower > 1 {
        let med = (lower + upper) / 2;
        if is_med_over(&a, med, k) {
            lower = med;
        } else {
            upper = med;
        }
    }
    println!("{}", upper);
}

fn is_med_over(a: &[usize], med: usize, k: usize) -> bool {
    // a/kを切り上げ
    let times = a.iter().map(|&a| ((a + med - 1) / med) - 1).sum::<usize>();
    times > k
}
