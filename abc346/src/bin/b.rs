use proconio::input;

fn main() {
    input! {
        w:usize,
        b:usize,
    }
    let p = "wbwbwwbwbwbw".to_string().chars().collect::<Vec<char>>();
    for i in 0..12 {
        let mut cw = 0;
        let mut cb = 0;
        for j in 0..w + b {
            if p[(i + j) % 12] == 'w' {
                cw += 1;
            } else {
                cb += 1;
            }
        }
        if w == cw && b == cb {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
