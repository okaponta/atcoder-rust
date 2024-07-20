use proconio::input;

fn main() {
    input! {
        n:usize,
        t:usize,
        p:usize,
        l:[usize;n],
    }
    for i in 0..200 {
        let mut count = 0;
        for j in 0..n {
            if t <= l[j] + i {
                count += 1;
            }
        }
        if p <= count {
            println!("{i}");
            return;
        }
    }
}
