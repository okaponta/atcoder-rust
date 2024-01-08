use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        mut a:[i64;n],
        q:usize,
        b:[i64;q],
    }
    a.sort();
    for b in b {
        let mut lower = 0;
        let mut upper = n - 1;
        while upper - lower > 1 {
            let med = (lower + upper) / 2;
            if a[med] < b {
                lower = med;
            } else {
                upper = med;
            }
        }
        let ans = (a[upper] - b).abs().min((a[lower] - b).abs());
        println!("{}", ans);
    }
}
