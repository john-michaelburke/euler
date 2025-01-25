/*
2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any
 remainder.

What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
*/

const DEFAULT_SEQUENCE: [u64; 2] = [1, 20];

pub fn solve(sequence: Option<[u64; 2]>) -> u64 {
    let [start, end] = if let Some(sequence) = sequence {
        sequence
    } else {
        DEFAULT_SEQUENCE
    };

    let mut smallest_multiple = end;

    loop {
        if (start..end).all(|x| smallest_multiple % x == 0) {
            println!("Smallest multiple which when divided by numbers in the range {start} to {end} have no remainder, {smallest_multiple}.");
            return smallest_multiple;
        } else {
            smallest_multiple += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example() {
        assert_eq!(solve(Some([1, 10])), 2520);
    }
}
