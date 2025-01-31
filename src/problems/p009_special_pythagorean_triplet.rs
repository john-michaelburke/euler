/*
A pythagorean triplet is a set of three natural numbers, a < b < c, for which,
a**2 + b**2 = c**2.

For example, 3**2 + 4**2 = 9 + 16 = 25 = 5**2.

There exists exactly one Pythagorean triplet for which a + b + c = 1000.
Find the product abc.
*/

const DEFAULT_TARGET: u64 = 1000;

pub fn solve(target: Option<u64>) {
    let target = if let Some(target) = target {
        target
    } else {
        DEFAULT_TARGET
    };

    let mut a = 0;
    let mut b = 1;
    let mut c = 2;
    let mut is_done = false;
    while !is_done {
        // c loop
        c += 1;
        b = 2;
        a = 1;
        while !is_done {
            // b loop
            b += 1;
            a = 1;
            if b >= c {
                break;
            }
            while !is_done {
                // a loop
                a += 1;
                if a >= b {
                    break;
                }
                if (a < b) && (b < c) && ((a + b + c) == target) && ((a * a + b * b) == c * c) {
                    is_done = true;
                }
            }
        }
    }
    let prod = a * b * c;
    println!("a: {a}, b: {b}, c: {c}, target: {target}, prod: {prod}");
}
