/*
The sum of the primes below 10 is 2 + 3 + 5 + 7 + 17.

Find the sum of all primes below two million.
*/

const DEFAULT_END: usize = 2_000_000;

pub fn solve(end: Option<usize>) {
    let end = if let Some(end) = end {
        end
    } else {
        DEFAULT_END
    };
    let mut cur: usize = 2;
    let mut possible_primes = vec![true; end];
    while (cur * cur) <= end {
        if possible_primes[cur] {
            for i in ((cur * cur)..(end)).step_by(cur) {
                possible_primes[i] = false;
            }
        }
        cur += 1;
    }
    let sum: u64 = (2..(end))
        .filter(|i| possible_primes[*i])
        .map(|i| i as u64)
        .sum();
    println!("The sum of primes under {end} is {sum}.");
}
