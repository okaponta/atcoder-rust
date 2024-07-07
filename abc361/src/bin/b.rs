use proconio::input;

fn main() {
    input! {
        a:[usize;6],
        b:[usize;6],
    }
    if f(a[0], a[3], b[0], b[3]) && f(a[1], a[4], b[1], b[4]) && f(a[2], a[5], b[2], b[5]) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn f(a1: usize, a2: usize, b1: usize, b2: usize) -> bool {
    if b1 <= a1 && a1 < b2 {
        return true;
    }
    if a2 <= b2 && b1 < a2 {
        return true;
    }
    if a1 <= b1 && b2 <= a2 {
        return true;
    }
    false
}
