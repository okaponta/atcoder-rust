use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s:Chars,
        k:usize,
    }
    s.reverse();
    for _ in 0..16 {
        s.push('0');
    }
    let mut ans = 0;
    let mut dig = 1;
    let mut kuriagari = 0;
    for i in 0..16 {
        let d = s[i].to_digit(10).unwrap() as usize + kuriagari;
        if i < k {
            if 5 <= d {
                kuriagari = 1;
            } else {
                kuriagari = 0;
            }
        } else {
            kuriagari = 0;
            ans += d * dig;
        }
        dig *= 10;
    }
    println!("{}", ans);
}
