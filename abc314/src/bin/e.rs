use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
    }
    let mut c = vec![];
    let mut p = vec![];
    let mut s = vec![];
    for _ in 0..n {
        input! {
            ci:usize,
            pi:usize,
            mut si:[usize;pi],
        }
        si = si.into_iter().filter(|i| i != &0).collect();
        let z = pi - si.len();
        c.push((ci * pi) as f64 / (pi - z) as f64);
        p.push(pi - z);
        s.push(si);
    }
    let mut ans = vec![0.0; m + 101];
    for i in (0..m).rev() {
        let mut tmp = 1e9;
        for j in 0..n {
            let mut ss = c[j];
            for k in 0..p[j] {
                ss += ans[i + s[j][k]] / p[j] as f64;
            }
            if ss < tmp {
                tmp = ss;
            }
        }
        ans[i] = tmp;
    }
    println!("{}", ans[0]);
}
