/*
If we list all the natural numbers below
 that are multiples of
 or
, we get
 and
. The sum of these multiples is
.

Find the sum of all the multiples of
 or
 below
.
*/

const DEFAULT_END: usize = 1000;
pub fn solve(end: Option<usize>) {
    let end = if let Some(end) = end {
        end
    } else {
        DEFAULT_END
    };
    let sum: usize = (1..end).filter(|i| (i % 3 == 0) || (i % 5 == 0)).sum();
    println!("Multiples of 3 or 5 with end at {end} is {sum}.");
}
