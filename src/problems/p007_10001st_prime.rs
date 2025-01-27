/*
By listing the first six prime numbers: 2,3,5,7,11, and 13, we can see that the 6th prime is 13.

What is the 10001st prime number?
*/

use std::collections::HashSet;

const DEFAULT_END: u64 = 10001;

pub fn solve(end: Option<u64>) -> u64 {
    let end = if let Some(end) = end {
        end
    } else {
        DEFAULT_END
    };
    let mut lookup: HashSet<u64> = HashSet::new();
    let mut cur: u64 = 2;

    loop {
        let mut maybe_cur_prime = cur;
        let mut cur_div = 2;
        loop {
            if lookup.contains(&maybe_cur_prime) || cur_div >= maybe_cur_prime {
                break;
            }
            if maybe_cur_prime % cur_div == 0 {
                maybe_cur_prime /= cur_div;
            } else {
                cur_div += 1;
            }
        }
        if maybe_cur_prime == cur {
            lookup.insert(cur);
        }
        if lookup.len() == end as usize {
            break;
        }
        cur += 1;
    }
    println!("The {end}st prime is {cur}.");
    cur
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example() {
        assert_eq!(solve(Some(6)), 13);
    }
}
