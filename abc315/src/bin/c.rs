use proconio::input;

fn main() {
    input! {
        n:usize,
        fs:[(usize,usize);n],
    }
    let mut max = (0, 0);
    for i in 0..n {
        if max.0 < fs[i].1 {
            max = (fs[i].1, i);
        }
    }
    let taste = fs[max.1].0;
    let mut ans = max.0;
    for i in 0..n {
        if i == max.1 {
            continue;
        }
        if fs[i].0 == taste {
            ans = ans.max(max.0 + fs[i].1 / 2);
        } else {
            ans = ans.max(max.0 + fs[i].1);
        }
    }
    println!("{}", ans);
}
