use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        q:usize,
        xyzw:[(Usize1,Usize1,Usize1,usize);q],
    }
    println!(
        "{}",
        (0..60).fold(1, |s, i| {
            s * (0..1 << n).filter(|&j| ok(i, j, &xyzw)).count() % 1000000007
        })
    );
}

fn ok(i: usize, j: usize, xyzw: &Vec<(usize, usize, usize, usize)>) -> bool {
    for &(x, y, z, w) in xyzw {
        if j >> x & 1 | j >> y & 1 | j >> z & 1 != w >> i & 1 {
            return false;
        }
    }
    true
}
