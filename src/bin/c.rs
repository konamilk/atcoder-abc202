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
    // let source = AutoSource::from("9
    // 2 2 3 4 5 5 2 1 9
    // 1 2 3 4 8 7 7 6 3
    // 3 3 1 2 4 4 5 6 2");
//     let source = AutoSource::from("3
// 1 2 2
// 3 1 2
// 2 3 2
// ");
    input!{
        // from source,
        n: usize,
        a: [usize;n],
        b: [usize;n],
        c: [usize;n],
    };

    let mut a_bucket = vec![0i64;n+1];
    let mut c_bucket = vec![0i64;n+1];

    for i in 0..n {
        a_bucket[a[i]] += 1;
        c_bucket[c[i]] += 1;
    }

    let mut ans = 0i64;

    for i in 0..n{
        ans += a_bucket[b[i]] * c_bucket[i+1]
    }

    println!("{}", ans);
    //
    // let mut ans_2 = 0i64;
    //
    // for i in 0..n{
    //     for j in 0..n{
    //         if a[i] == b[c[j] - 1]{
    //             ans_2 += 1
    //         }
    //     }
    // }
    //
    // println!("{}", ans_2)
}

