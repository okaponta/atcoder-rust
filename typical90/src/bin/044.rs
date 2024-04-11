use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        q:usize,
        mut a:[usize;n],
        txy:[(u8,usize,usize);q],
    }
    let mut offset = 0;
    for (t, x, y) in txy {
        if t == 1 {
            // swap
            a.swap((n + x - offset - 1) % n, (n + y - offset - 1) % n);
        } else if t == 2 {
            // shift
            offset += 1;
            offset %= n;
        } else {
            // print
            println!("{}", a[(n + x - offset - 1) % n]);
        }
    }
}
