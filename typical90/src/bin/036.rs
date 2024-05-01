use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,
        q:usize,
        xy:[(i64,i64);n],
        qi:[Usize1;q],
    }
    // マンハッタン距離が一定の点を結ぶと菱形みたいな形だが45度回転して正方形になる。これが一番遠ければOK。
    // なので、一番遠くの4点を記録しておく
    let mut xmin = 1 << 60;
    let mut xmax = -1 << 60;
    let mut ymin = 1 << 60;
    let mut ymax = -1 << 60;
    for &(x, y) in &xy {
        xmin = xmin.min(x + y);
        xmax = xmax.max(x + y);
        ymin = ymin.min(x - y);
        ymax = ymax.max(x - y);
    }
    for i in qi {
        let x = xy[i].0 + xy[i].1;
        let y = xy[i].0 - xy[i].1;
        let ans = (x - xmin).max(xmax - x).max(y - ymin).max(ymax - y);
        println!("{}", ans);
    }
}
