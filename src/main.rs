#![allow(unused_macros)]

use itertools::Itertools;

macro_rules! rep {
    ($n: expr, $body: block) => {
        for _ in 0..$n {
            $body
        }
    };
    ($i: ident, $n: expr, $body: block) => {
        for $i in 0..$n {
            $body
        }
    };
    ($i: ident, $a: expr, $b: expr, $body: block) => {
        for $i in $a..=$b {
            $body
        }
    };
    ($i: ident, $a: expr, $b: expr, $c: expr, $body: block) => {
        let mut $i = $a;
        while $i <= $b {
            $body
            $i += $c;
        }
    };
}

macro_rules! rvp {
    ($i: ident, $n: expr, $body: block) => {
        for $i in (0..$n).rev() {
            $body
        }
    };
    ($i: ident, $a: expr, $b: expr, $body: block) => {
        for $i in ($b..=$a).rev() {
            $body
        }
    };
    ($i: ident, $a: expr, $b: expr, $c: expr, $body: block) => {
        let mut $i = $a;
        while $i >= $b {
            $body
            $i -= $c;
        }
    };
}

macro_rules! each {
    ($el: ident, $v: expr, $body: block) => {
        for $el in $v {
            $body
        }
    }
}

#[allow(dead_code)]
fn out<T: std::fmt::Display>(x: T) {
    print!("{}", x);
}
macro_rules! out {
    () => {
        println!();
    };
    ($head:expr $(, $tail:expr)*) => {
        out($head);
        $(
            print!(" ");
            out($tail);
        )*
        println!();
    };
}

#[allow(dead_code)]
fn vout<T: std::fmt::Display>(v: Vec<T>) {
    println!("{}", v.iter().join(" "));
}
macro_rules! vout {
    () => {
        println!();
    };
    ($head:expr $(, $tail:expr)*) => {
        vout($head);
        $(vout($tail);)*
    };
}

#[allow(dead_code)]
fn wout<T: std::fmt::Display>(w: Vec<Vec<T>>) {
    each!(v, w, {
        vout(v);
    });
}

#[allow(dead_code)]
const MOD: i32 = 998244353;
#[allow(dead_code)]
const M0D: i32 = 1e9 as i32 + 7;
#[allow(dead_code)]
const INF: i32 = 1 << 30;
#[allow(dead_code)]
const LINF: i64 = (1 << 61) - 1;
#[allow(dead_code)]
#[allow(non_upper_case_globals)]
const dx: [i32; 9] = [0, -1, 1, 0, 0, -1, -1, 1, 1];
#[allow(dead_code)]
#[allow(non_upper_case_globals)]
const dy: [i32; 9] = [0, 0, 0, -1, 1, -1, 1, -1, 1];
const MULTI: bool = false;

pub fn main() {
    rep!(match MULTI {
        true => {
            let mut num = String::new();
            std::io::stdin().read_line(&mut num).expect("invalid input");
            num.parse().unwrap()
        }
        _ => 1
    }, {
        solve();
    });
}

fn solve() {
dd
}