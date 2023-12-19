use proconio::input;

fn main() {
    input! {
        n:usize,
        a:usize,
        b:usize,
        h:[usize;n],
    }
    let mut lower = 0;
    let mut upper = 1 << 60;
    while upper - lower > 1 {
        let med = (lower + upper) / 2;
        if is_ok(med, &h, a, b) {
            upper = med;
        } else {
            lower = med;
        }
    }
    println!("{}", upper);
}

// たおせる
fn is_ok(med: usize, h: &Vec<usize>, a: usize, b: usize) -> bool {
    let mut count = 0;
    for &h in h {
        if h <= med * b {
            continue;
        }
        count += ((h - med * b) + (a - b) - 1) / (a - b);
    }
    count <= med
}
