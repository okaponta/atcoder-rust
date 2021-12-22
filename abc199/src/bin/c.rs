use proconio::{input, marker::Chars};

fn flip_index(i: usize, n: usize) -> usize {
    if i >= n {
        return i - n;
    }
    return i + n;
}

fn main() {
    input! {
       n:usize,
       mut s:Chars,
       q:i32,
       tab:[(i32,usize,usize);q],
    }
    let mut flip = false;
    for e in tab {
        if e.0 == 2 {
            flip = !flip;
            continue;
        }
        if flip {
            s.swap(flip_index(e.1 - 1, n), flip_index(e.2 - 1, n));
        } else {
            s.swap(e.1 - 1, e.2 - 1);
        }
    }
    if flip {
        let mut later = s.split_off(n);
        later.append(&mut s);
        println!("{}", later.iter().collect::<String>());
    } else {
        println!("{}", s.iter().collect::<String>());
    }
}
