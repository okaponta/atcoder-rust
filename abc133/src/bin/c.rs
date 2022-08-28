use proconio::input;

fn main() {
    input! {
        mut l:usize,
        mut r:usize,
    }
    r = r.min(l + 2019);
    let mut ans = 2019 * 2019;
    for i in l..=r {
        for j in i + 1..=r {
            ans = ans.min(((i % 2019) * (j % 2019)) % 2019);
        }
    }
    println!("{}", ans);
}
