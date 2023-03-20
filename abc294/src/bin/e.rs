use proconio::input;

fn main() {
    input! {
        _:usize,
        n1:usize,
        n2:usize,
        mut vl1:[(i64,usize);n1],
        mut vl2:[(i64,usize);n2],
    }
    let mut i1 = 0;
    let mut i2 = 0;
    let mut ans = 0;
    while i1 != n1 && i2 != n2 {
        let seg = vl1[i1].1.min(vl2[i2].1);
        if vl1[i1].0 == vl2[i2].0 {
            ans += seg;
        }
        vl1[i1].1 -= seg;
        vl2[i2].1 -= seg;
        if vl1[i1].1 == 0 {
            i1 += 1;
        }
        if vl2[i2].1 == 0 {
            i2 += 1;
        }
    }
    println!("{}", ans);
}
