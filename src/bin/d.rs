use proconio::input;
#[allow(unused_imports)]
use proconio::source::auto::AutoSource;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};
#[allow(unused_imports)]
use num::integer::{sqrt, gcd, lcm};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};

fn main() {
    // let source = AutoSource::from("3 2 5");
    input!{
        // from source,
        mut a: i64,
        mut b: i64,
        mut k: i64
    };

    let n = a+b;

    let mut ans = vec![];

    while ans.len() < n as usize{
        let boundary = binom_knuth(a+b, a) * a / (a+b);
        if k > boundary{
            ans.push('b');
            k -= boundary;
            b -= 1;
        }
        else {
            ans.push('a');
            a = a-1;
        }
    }

    println!("{}", ans.iter().collect::<String>())

}


pub fn binom_knuth(n: i64, mut k: i64) -> i64 {
    if n < k {
        return 0
    }
    k = std::cmp::min(k, n-k);
    (0..n + 1)
        .rev()
        .zip(1..k + 1)
        .fold(1, |mut r, (n, d)| { r *= n; r /= d; r })
}