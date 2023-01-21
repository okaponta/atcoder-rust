use proconio::input;

fn main() {
    input! {
        n:usize,
        x:usize,
        ab:[(usize,usize);n],
    }
    let mut ans = vec![false; x + 1];
    ans[0] = true;
    for (a, b) in ab {
        for i in (0..x).rev() {
            if !ans[i] {
                continue;
            }
            for j in 1..=b {
                if x < i + j * a {
                    break;
                }
                ans[i + j * a] = true;
            }
        }
    }
    println!("{}", if ans[x] { "Yes" } else { "No" });
}
