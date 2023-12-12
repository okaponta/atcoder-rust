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
            if g < state[1] {
                state[1] -= g;
                state[0] = g;
            } else {
                state[0] = state[1];
                state[1] = 0;
            }
        }
    }
    println!("{} {}", state[0], state[1]);
}
