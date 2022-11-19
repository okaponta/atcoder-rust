use proconio::input;

fn main() {
    input! {
        h:usize,
        m:usize,
    }
    let mut min = h * 60 + m;
    loop {
        if is_ans(min) {
            println!("{} {}", min / 60, min % 60);
            return;
        }
        min += 1;
        if min == 24 * 60 {
            min = 0;
        }
    }
}

fn is_ans(min: usize) -> bool {
    let h = min / 60;
    let m = min % 60;
    let a = h / 10;
    let b = h % 10;
    let c = m / 10;
    if a == 2 {
        return c < 4;
    }
    return b < 6;
}
