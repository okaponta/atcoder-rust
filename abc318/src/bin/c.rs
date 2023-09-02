use proconio::input;

fn main() {
    input! {
        n:usize,
        d:usize,
        p:usize,
        mut f:[usize;n],
    }
    let mut s = f.iter().sum::<usize>();
    let mut i = 0;
    let mut j = 0;
    f.sort();
    f.reverse();
    let mut ans = s;
    while j < n {
        j += d;
        s += p;
        while i < j && i < n {
            s -= f[i];
            i += 1;
        }
        ans = ans.min(s);
    }
    println!("{}", ans);
}
