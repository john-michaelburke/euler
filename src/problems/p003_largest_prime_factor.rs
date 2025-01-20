/*
The prime factors of 13195 are 5, 7, 13, and 29.

What is the largest prime factor of the number 600851475143 ?
*/

use std::cmp::max;

const DEFAULT_TARGET: u64 = 600851475143;

pub fn solve(target: Option<u64>) {
    let orig_target = if let Some(target) = target {
        target
    } else {
        DEFAULT_TARGET
    };
    let mut target = orig_target;
    let mut max_prime = 0;
    let mut cur_div = 2;
    loop {
        if cur_div >= target {
            break;
        }
        if target % cur_div == 0 {
            target /= cur_div;
            max_prime = max(max_prime, cur_div);
        } else {
            cur_div += 1;
        }
    }
    println!("{orig_target}, {max_prime}, {cur_div}, {target}");
}
