use proconio::input;

fn main() {
    input! {
       n:usize,w:i64,
       stp:[(usize,usize,i64);n],
    }
    let mut time = vec![0; 200001];
    for (s, t, p) in stp {
        time[s] += p;
        time[t] -= p;
    }
    let mut want = 0;
    for ti in time {
        want += ti;
        if want > w {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
