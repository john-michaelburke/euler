/*
A palindromic number reads the same both ways. The largest palindrome made from the product of two
-digit numbers is  9009 = 91 x 99.

Find the largest palindrome made from the product of two 3-digit numbers.
*/

const DEFAULT_DIGITS: [u32; 2] = [3, 3];

fn check_palindrome(value: u64) -> bool {
    let string_value = value.to_string();
    let mut is_palindrome = true;
    for i in 0..((string_value.len()) / 2) {
        let left = string_value.chars().nth(i).unwrap();
        let right = string_value
            .chars()
            .nth(string_value.len() - i - 1)
            .unwrap();
        // println!("{left} vs {right}");
        if left != right {
            is_palindrome = false;
            break;
        }
    }
    is_palindrome
}

pub fn solve(digits: Option<[u32; 2]>) {
    let digits = if let Some(digits) = digits {
        digits
    } else {
        DEFAULT_DIGITS
    };
    let (max_a, max_b) = (10u64.pow(digits[0]) - 1, 10u64.pow(digits[1]) - 1);
    let mut cur_a = max_a;
    let mut cur_b = max_b;
    let mut max_palindrome = 0;
    let mut max_palindrome_a_and_b: [u64; 2] = [0, 0];
    loop {
        let multiple = cur_a * cur_b;
        if check_palindrome(multiple) {
            if multiple > max_palindrome {
                max_palindrome = multiple;
                max_palindrome_a_and_b = [cur_a, cur_b];
            }
            cur_a -= 1;
            cur_b = max_b;
        } else {
            cur_b -= 1;
        }
        if cur_a == 0 {
            break;
        }
    }
    let (digit_left, digit_right) = (max_palindrome_a_and_b[0], max_palindrome_a_and_b[1]);
    println!("Left: {digit_left}, Right: {digit_right}, Palindrome: {max_palindrome}");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_palindromes() {
        assert!(check_palindrome(9009));
        assert!(!check_palindrome(9109));
        assert!(check_palindrome(90109));
        assert!(!check_palindrome(90119));
    }
}
