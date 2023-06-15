use num::Complex;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        mut ab:[(usize,usize);n],
    }
    ab.insert(0, (0, 0));
    let a = ab.iter().map(|&ab| ab.0 as f64).collect();
    let b = ab.iter().map(|&ab| ab.1 as f64).collect();
    let c = convolve(a, b);
    let ans: Vec<usize> = c.iter().map(|&z| z.round() as usize).collect();
    for i in 1..=2 * n {
        println!("{}", ans[i])
    }
}

fn convolve(mut a: Vec<f64>, mut b: Vec<f64>) -> Vec<f64> {
    let s = a.len() + b.len() - 1;
    let t = s.next_power_of_two();
    a.resize(t, 0.0);
    b.resize(t, 0.0);
    let mut fa = fft_real(a);
    let fb = fft_real(b);
    for i in 0..t {
        fa[i] *= fb[i];
    }
    fa = inverse_fft(fa);
    (0..s).into_iter().map(|i| fa[i].re).collect::<Vec<_>>()
}

fn fft_real(a: Vec<f64>) -> Vec<Complex<f64>> {
    fft(a
        .into_iter()
        .map(|f| Complex::new(f, 0.0))
        .collect::<Vec<_>>())
}

pub fn fft(mut a: Vec<Complex<f64>>) -> Vec<Complex<f64>> {
    let n = a.len();
    assert!(
        n.count_ones() == 1,
        "the length of array should be power of two"
    );
    let bit = n.trailing_zeros() as usize;

    for si in (0..bit).rev() {
        let s = 1_usize << si;
        let zeta = Complex::from_polar(&1.0, &(2.0 * std::f64::consts::PI / (s << 1) as f64));
        for ii in 0..(n / (s << 1)) {
            let i = ii * (s << 1);
            let mut z_i = Complex::new(1.0, 0.0);
            for j in 0..s {
                let t = a[i + j] - a[s + i + j];
                a[i + j] = a[i + j] + a[s + i + j];
                a[s + i + j] = t * z_i;
                z_i *= zeta;
            }
        }
    }

    a
}

pub fn inverse_fft(mut a: Vec<Complex<f64>>) -> Vec<Complex<f64>> {
    let n = a.len();
    assert!(
        n.count_ones() == 1,
        "the length of array should be power of two"
    );
    let bit = n.trailing_zeros() as usize;

    for si in 0..bit {
        let s = 1_usize << si;
        let zeta = Complex::from_polar(&1.0, &(-2.0 * std::f64::consts::PI / (s << 1) as f64));
        for ii in 0..(n / (s << 1)) {
            let i = ii * (s << 1);
            let mut z_i = Complex::new(1.0, 0.0);
            for j in 0..s {
                let t = a[s + i + j] * z_i;
                a[s + i + j] = a[i + j] - t;
                a[i + j] = a[i + j] + t;
                z_i *= zeta;
            }
        }
    }

    let inv_n = Complex::new(1_f64 / n as f64, 0f64);
    a.iter().map(|&x| x * inv_n).collect()
}
