use proconio::input;

fn main() {
    input! {
        mut a:usize,
        mut b:usize,
        mut c:usize,
    }
    let aa = a as u32;
    let bb = b as u32;
    let mut bit = vec![0; 60];
    for i in 0..60 {
        bit[i] = c & 1;
        c /= 2;
    }
    let odd = bit.iter().filter(|i| i == &&1).count();
    if (a + b - odd) % 2 != 0 {
        println!("-1");
        return;
    }
    let mut both = (a + b - odd) / 2;
    let mut x = 0;
    let mut y = 0;
    let mut d = 1usize;
    for i in 0..60 {
        if bit[i] == 0 {
            if 0 < both {
                x += d;
                y += d;
                both -= 1;
            }
        } else {
            if a < b {
                y += d;
                b -= 1;
            } else {
                x += d;
                a -= 1;
            }
        }
        d *= 2;
    }
    if x.count_ones() != aa || y.count_ones() != bb {
        println!("-1");
        return;
    }
    println!("{} {}", x, y);
}
