use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i32, i32); n]
    }

    let two = xy
        .iter()
        .tuple_combinations()
        .map(|(a, b)| circum_from_two(a.0, a.1, b.0, b.1))
        .filter(|&(x, y, r)| is_inside(x, y, r, &xy))
        .fold(0.0 / 0.0, |m: f64, e| m.min(e.2));
    let three = xy
        .iter()
        .permutations(3)
        .map(|p| (p[0], p[1], p[2]))
        .filter(|(a, b, c)| !is_on_line(a.0, a.1, b.0, b.1, c.0, c.1))
        .map(|(a, b, c)| circum_from_three(a.0, a.1, b.0, b.1, c.0, c.1))
        .filter(|&(x, y, r)| is_inside(x, y, r + 1e-8, &xy))
        .fold(0.0 / 0.0, |m: f64, e| m.min(e.2));
    println!("{}", two.min(three));
}

fn is_on_line(x0: i32, y0: i32, x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
    (y1 - y0) * (x2 - x0) == (y2 - y0) * (x1 - x0)
}

fn is_inside(x0: f64, y0: f64, r: f64, xy: &Vec<(i32, i32)>) -> bool {
    !xy.iter()
        .any(|&(x, y)| (x as f64 - x0).hypot(y as f64 - y0) > r)
}

fn circum_from_two(x0: i32, y0: i32, x1: i32, y1: i32) -> (f64, f64, f64) {
    (
        (x0 + x1) as f64 / 2.,
        (y0 + y1) as f64 / 2.,
        ((x0 - x1) as f64).hypot((y0 - y1) as f64) / 2. + 1e-8,
    )
}

fn circum_from_three(x0: i32, y0: i32, x1: i32, y1: i32, x2: i32, y2: i32) -> (f64, f64, f64) {
    let (x1, y1) = ((x1 - x0) as f64, (y1 - y0) as f64);
    let (x2, y2) = ((x2 - x0) as f64, (y2 - y0) as f64);
    let q = (x1 * x1 * x2 - x2 * x2 * x1 + y1 * y1 * x2 - y2 * y2 * x1) / (y1 * x2 - y2 * x1) / 2.;
    let p = (x1 * x1 + y1 * y1 - 2. * q * y1) / x1 / 2.;
    let r = p.hypot(q) + 1e-8;
    (p + x0 as f64, q + y0 as f64, r)
}
