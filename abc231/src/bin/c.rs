use proconio::input;

fn main() {
    input! {
       n:usize,q:usize,
       mut a:[i32;n],
       x:[i32;q],
    }
    a.sort();

    for xi in x {
        if xi <= a[0] {
            println!("{}", n);
            continue;
        }
        let mut l = 0;
        let mut r = n;
        while r - l > 1 {
            let m = (l + r) / 2;
            if a[m as usize] < xi {
                l = m
            } else {
                r = m
            }
        }
        println!("{}", n - r);
    }
}
