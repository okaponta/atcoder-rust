use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        xy:[(i64,i64);n],
        m:usize,
        uv:[(i64,i64);m],
        q:usize,
        ab:[(i64,i64);q],
    }
    let mut v1 = vec![];
    for i in 0..n {
        if i == n - 1 {
            v1.push((xy[0].0 - xy[i].0, xy[0].1 - xy[i].1));
        } else {
            v1.push((xy[i + 1].0 - xy[i].0, xy[i + 1].1 - xy[i].1));
        }
    }
    let mut outer1 = vec![];
    for i in 0..n {
        let mut max = -1 << 60;
        for j in 0..m {
            let x = xy[i].0 + uv[j].0;
            let y = xy[i].1 + uv[j].1;
            max = max.max(outer_product(v1[i].0, v1[i].1, x, y));
        }
        outer1.push(max);
    }
    for i in 0..q {
        let mut flag = true;
        for j in 0..n {
            let outer2 = outer_product(v1[j].0, v1[j].1, ab[i].0, ab[i].1);
            if outer1[j] > outer2 {
                flag = false;
                break;
            }
        }
        println!("{}", if flag { "Yes" } else { "No" });
    }
}

fn outer_product(x0: i64, y0: i64, x1: i64, y1: i64) -> i64 {
    x0 * y1 - y0 * x1
}
