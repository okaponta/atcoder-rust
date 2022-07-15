use nalgebra::{Matrix3, Matrix3x1};
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,
        xy:[(i64,i64);n],
        m:usize,
    }
    let mut op = vec![];
    for _ in 0..m {
        input! {
            t: usize
        }
        if t > 2 {
            input! {
                p: i64
            }
            op.push((t, p));
        } else {
            op.push((t, 0));
        }
    }
    input! {
        q: usize,
        ab: [(usize, Usize1); q]
    }
    // 単位行列
    let mut mat = vec![Matrix3::new(1, 0, 0, 0, 1, 0, 0, 0, 1)];
    for (t, p) in op {
        let v = match t {
            // 時計回りに90度
            1 => Matrix3::new(0, 1, 0, -1, 0, 0, 0, 0, 1),
            // 半時計回りに90度
            2 => Matrix3::new(0, -1, 0, 1, 0, 0, 0, 0, 1),
            // x=pで線対象
            3 => Matrix3::new(-1, 0, 2 * p, 0, 1, 0, 0, 0, 1),
            // y=pで線対象
            _ => Matrix3::new(1, 0, 0, 0, -1, 2 * p, 0, 0, 1),
        };
        // あとからベクトルに適用したいので、前にかけていく
        mat.push(&v * mat.last().unwrap());
    }
    for (a, b) in ab {
        let (x, y) = xy[b];
        let pv = Matrix3x1::new(x, y, 1);
        let v = mat[a] * &pv;
        println!("{} {}", v[0], v[1]);
    }
}
