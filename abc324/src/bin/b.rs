use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    let xn = n.next_power_of_two().trailing_zeros() as usize;
    for x in 0..=xn {
        let mut y = 0;
        while calc(x, y) <= n {
            if calc(x, y) == n {
                println!("Yes");
                return;
            }
            y += 1;
        }
    }
    println!("No");
}

fn calc(x: usize, y: usize) -> usize {
    2usize.pow(x as u32) * 3usize.pow(y as u32)
}
