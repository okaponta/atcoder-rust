use proconio::input;

fn main() {
    input! {
        n:usize,
        ta:[(usize,usize);n],
    }
    let mut t = ta[0].0;
    let mut a = ta[0].1;
    for i in 1..n {
        let r1 = (ta[i].0 + t - 1) / ta[i].0;
        let mut t1 = ta[i].0 * r1;
        let mut a1 = ta[i].1 * r1;
        let r2 = (ta[i].1 + a - 1) / ta[i].1;
        let mut t2 = ta[i].0 * r2;
        let mut a2 = ta[i].1 * r2;
        if t1 + a1 < t2 + a2 {
            std::mem::swap(&mut t1, &mut t2);
            std::mem::swap(&mut a1, &mut a2);
        }
        if t <= t1 && a <= a1 {
            t = t1;
            a = a1;
        } else {
            t = t2;
            a = a2;
        }
    }
    println!("{}", t + a);
}
