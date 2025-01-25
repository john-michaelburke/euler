/*
The sum of the squares of the first ten natural numbers is,
1**2 + 2**2 + ...+ 10**2 = 385

The square of the sum of the first ten natural numbers is,
(1+2+...+10)**2 = 55**2 = 3025

Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is
3025-385 = 2640

Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

*/
const DEFAULT_END: u64 = 100;

pub fn solve(end: Option<u64>) {
    let end = if let Some(end) = end {
        end
    } else {
        DEFAULT_END
    };
    let sum: u64 = (1..end + 1).sum();
    let squared_sum = sum * sum;
    let sum_of_squares = (1..end + 1).fold(0, |acc, x| acc + x * x);
    let squared_sum_minus_sum_of_squares = squared_sum - sum_of_squares;
    println!("{squared_sum_minus_sum_of_squares}");
}
