use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[(i64,i64);n],
        b:[(i64,i64);n],
    }
    println!("{}", closest(b) / closest(a));
}

fn closest(mut points: Vec<(i64, i64)>) -> f64 {
    points.sort_by_key(|p| (p.0));
    let square = closest_pair(&points, 0, points.len()).0;
    (square as f64).sqrt()
}

fn closest_pair(points: &Vec<(i64, i64)>, i: usize, n: usize) -> (i64, Vec<(i64, i64)>) {
    if n <= 1 {
        return (1 << 60, vec![points[i]]);
    }
    let m = n / 2;
    let x = points[i + m].0;
    let (d1, qs1) = closest_pair(points, i, m);
    let (d2, qs2) = closest_pair(points, i + m, n - m);
    let mut d = d1.min(d2);
    let qs = merge(qs1, qs2);
    let mut b: Vec<(i64, i64)> = vec![];
    for i in 0..n {
        if (qs[i].0 - x).abs() * (qs[i].0 - x).abs() >= d {
            continue;
        }
        for j in 0..b.len() {
            let dx = qs[i].0 - b[b.len() - j - 1].0;
            let dy = qs[i].1 - b[b.len() - j - 1].1;
            if dy * dy >= d {
                break;
            }
            d = d.min(dx * dx + dy * dy);
        }
        b.push(qs[i]);
    }
    (d, qs)
}

// 自前でマージソート実装
fn merge(left: Vec<(i64, i64)>, right: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut qs = vec![];
    let mut s = 0;
    let mut t = 0;
    while s < left.len() && t < right.len() {
        if left[s].1 < right[t].1 {
            qs.push(left[s]);
            s += 1;
        } else {
            qs.push(right[t]);
            t += 1;
        }
    }
    while s < left.len() {
        qs.push(left[s]);
        s += 1;
    }
    while t < right.len() {
        qs.push(right[t]);
        t += 1;
    }
    qs
}
