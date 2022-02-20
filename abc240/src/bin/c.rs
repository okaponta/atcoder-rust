use proconio::input;

fn main() {
    input! {
        n:usize,x:usize,
        ab:[(usize,usize);n],
    }
    let mut able = vec![false; x + 200];
    able[0] = true;
    for (a, b) in ab {
        let mut next = vec![false; x + 200];
        for i in 0..x + 1 {
            if able[i] {
                next[i + a] = true;
                next[i + b] = true;
            }
        }
        able = next;
    }
    println!("{}", if able[x] { "Yes" } else { "No" });
}
