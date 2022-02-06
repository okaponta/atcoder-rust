// 3点が同一線状にあるか判定
fn is_on_line(x0: i32, y0: i32, x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
    (y1 - y0) * (x2 - x0) == (y2 - y0) * (x1 - x0)
}

// 2点を渡すと、中心と半径を返却する
fn circum_from_two(x0: i32, y0: i32, x1: i32, y1: i32) -> (f64, f64, f64) {
    (
        (x0 + x1) as f64 / 2.,
        (y0 + y1) as f64 / 2.,
        ((x0 - x1) as f64).hypot((y0 - y1) as f64),
    )
}

// 3点を渡すと、中心と半径を返却する
fn circum_from_three(x0: i32, y0: i32, x1: i32, y1: i32, x2: i32, y2: i32) -> (f64, f64, f64) {
    let (x1, y1) = ((x1 - x0) as f64, (y1 - y0) as f64);
    let (x2, y2) = ((x2 - x0) as f64, (y2 - y0) as f64);
    let q = (x1 * x1 * x2 - x2 * x2 * x1 + y1 * y1 * x2 - y2 * y2 * x1) / (y1 * x2 - y2 * x1) / 2.;
    let p = (x1 * x1 + y1 * y1 - 2. * q * y1) / x1 / 2.;
    let r = p.hypot(q) + 1e-8;
    (p + x0 as f64, q + y0 as f64, r)
}

// (x^2 + y^2).sqrt() = x.hypot(y)

fn is_inside(x0: f64, y0: f64, r: f64, xy: &Vec<(i32, i32)>) -> bool {
    !xy.iter()
        .any(|&(x, y)| (x as f64 - x0).hypot(y as f64 - y0) > r)
}
