use itertools::iproduct;
use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
        xy:[(i64,i64);n],
    }
    let mut ans = 1 << 62;
    for (i, j, l, m) in iproduct!(0..n, 0..n, 0..n, 0..n) {
        let xmin = xy[i].0.min(xy[j].0);
        let xmax = xy[i].0.max(xy[j].0);
        let ymin = xy[l].1.min(xy[m].1);
        let ymax = xy[l].1.max(xy[m].1);
        if k <= xy
            .iter()
            .filter(|&&(x, y)| xmin <= x && x <= xmax && ymin <= y && y <= ymax)
            .count()
        {
            ans = ans.min((xmax - xmin) * (ymax - ymin));
        }
    }
    println!("{}", ans);
}
