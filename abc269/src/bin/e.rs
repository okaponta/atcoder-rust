fn main() {
    let n: usize = read().parse().unwrap();
    let mut rooks = n;
    let min = 1;
    let max = n;
    let mut imin = 1;
    let mut imax = n;
    let mut jmin = 1;
    let mut jmax = n;
    loop {
        if imax != imin {
            // iを決める
            rooks = (rooks + 1) / 2;
            let ihalf = imin + rooks - 1;
            println!("? {} {} {} {}", imin, ihalf, min, max);
            let t1: usize = read().parse().unwrap();
            if t1 != rooks {
                imax = ihalf;
            } else {
                imin = ihalf + 1;
            }
            if imax == imin {
                // i決まった
                rooks = n;
            }
        } else {
            // jを決める
            rooks = (rooks + 1) / 2;
            let jhalf = jmin + rooks - 1;
            println!("? {} {} {} {}", min, max, jmin, jhalf);
            let t2: usize = read().parse().unwrap();
            if t2 != rooks {
                jmax = jhalf;
            } else {
                jmin = jhalf + 1;
            }
        }
        if imax == imin && jmin == jmax {
            break;
        }
    }
    println!("! {} {}", imin, jmin);
}

fn read() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).ok();
    buf.split_whitespace().next().unwrap().to_string()
}
