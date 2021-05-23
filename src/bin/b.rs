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
    // let source = AutoSource::from("0");
    input!{
        // from source,
        s: Chars
    };

    let mut ans = vec![];

    for i in 0..s.len(){
        match s[i]{
            '6' => ans.push('9'),
            '9' => ans.push('6'),
            _ => ans.push(s[i])
        }
    }

    println!("{}", ans.iter().rev().collect::<String>())
}
