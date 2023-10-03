use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut map = vec![0; 100000];
    for a in a {
        map[a] += 1;
    }
    let mut ans = 0;
    for i in 1..99999 {
        ans = ans.max(map[i - 1] + map[i] + map[i + 1]);
    }
    println!("{}", ans);
}
