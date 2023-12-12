use proconio::input;

fn main() {
    input! {
        k:usize,
        g:usize,
        m:usize,
    }
    let mut state = vec![0; 2];
    for _ in 0..k {
        if state[0] == g {
            state[0] = 0;
        } else if state[1] == 0 {
            state[1] = m;
        } else {
            let a = g - state[0];
            let b = state[1];
            if a < b {
                state[1] -= a;
                state[0] += a;
            } else {
                state[1] -= b;
                state[0] += b;
            }
        }
    }
    println!("{} {}", state[0], state[1]);
}
