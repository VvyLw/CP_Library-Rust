#[allow(unused_imports)]
use itertools::Itertools;

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
    let t = match MULTI {
        true => {
            let mut num = String::new();
            std::io::stdin().read_line(&mut num).expect("invalid input");
            num.parse().unwrap()
        }
        _ => 1
    };
    for _ in 0..t {
        solve();
    }
}

fn solve() {

}