use itertools::Itertools;

use crate::problem::Problem;

const PASSWORD_LOW: usize = 146810;
const PASSWORD_HIGH: usize = 612564;

fn digits(n: &usize) -> Vec<usize> {
    if n < &10 {
        vec![*n]
    } else {
        let mut rest = digits(&(n / 10));
        rest.push(n % 10);
        rest
    }
}

fn not_decreasing(n: &usize) -> bool {
    let mut last_digit = 0;
    for digit in digits(n) {
        if digit < last_digit {
            return false;
        } else {
            last_digit = digit;
        }
    }

    true
}

fn two_same_adjacent(n: &usize) -> bool {
    for (key, group) in &digits(n).into_iter().group_by(|x| *x) {
        // This is a dumb way to count the elements of an iterator, but there's
        // no built-in len method so needs must...
        if group.sum::<usize>() >= (2 * key) {
            return true;
        }
    }
    false
}

fn exactly_two_same_adjacent(n: &usize) -> bool {
    for (key, group) in &digits(n).into_iter().group_by(|x| *x) {
        if group.sum::<usize>() == (2 * key) {
            return true;
        }
    }
    false
}

pub struct DayFour {}

impl Problem for DayFour {
    fn part_one(&self, input: &str) -> String {
        let valid_passwords: Vec<usize> = (PASSWORD_LOW..PASSWORD_HIGH + 1)
            .filter(not_decreasing)
            .filter(two_same_adjacent)
            .collect();

        format!("{}", valid_passwords.len())
    }

    fn part_two(&self, input: &str) -> String {
        let valid_passwords: Vec<usize> = (PASSWORD_LOW..PASSWORD_HIGH + 1)
            .filter(not_decreasing)
            .filter(exactly_two_same_adjacent)
            .collect();

        format!("{}", valid_passwords.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn int_to_digits() {
        assert_eq!(vec![3], digits(&3));
        assert_eq!(vec![1, 0], digits(&10));
        assert_eq!(vec![6, 5, 4, 3, 2, 1], digits(&654321));
    }
}
