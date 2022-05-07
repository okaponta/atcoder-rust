use proconio::input;

fn main() {
    input! {
        t:usize,
        lr:[(usize,usize);t],
    }
    for (l, r) in lr {
        solve(l, r);
    }
}

fn solve(l: usize, r: usize) {
    let mut l_dig = 0;
    let mut r_dig = 0;
    let mut p = 1;
    for i in 0..11 {
        if l_dig == 0 && l < p {
            l_dig = i;
        }
        if r_dig == 0 && r < p {
            r_dig = i;
        }
        p *= 10;
    }
    if l_dig == r_dig {
        println!("{}", r - l + 1);
        return;
    }
    let base = pow10(r_dig - 1);
    if r < base * 2 {
        let r_left = r / 10;
        let r_right = r - base;
        let min = (r_left.max(r_right) + 1).max(l);
        println!("{}", r - min + 1);
        return;
    }
    println!("{}", r - base + 1);
}

fn pow10(x: usize) -> usize {
    let mut res = 1;
    for _ in 0..x {
        res *= 10;
    }
    res
}
