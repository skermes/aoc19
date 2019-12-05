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

// This is for sure not the most efficient way to do this - not only am I
// calling digits twice for the same number but I'm then cloning it
// twice too.  Eh.
fn digit_pairs(n: &usize) -> Vec<(usize, usize)> {
    digits(n).iter().cloned().zip(digits(n).iter().cloned().skip(1)).collect()
}

fn has_identical_pair(password: &usize) -> bool {
    digit_pairs(password)
        .iter()
        .any(|(left, right)| left == right)
}

fn no_decreasing_pairs(password: &usize) -> bool {
    digit_pairs(password)
        .iter()
        .all(|(left, right)| left <= right)
}

// We only ever construct a range from the digits within bounds.
#[allow(dead_code)]
fn within_bounds(password: &usize) -> bool {
    password >= &PASSWORD_LOW && password <= &PASSWORD_HIGH
}

// The is-six-digits constraint is implied by being within the input bounds.
// fn is_valid_password(password: &usize) -> bool {
//     let pairs = digit_pairs(password);

//     password >= &PASSWORD_LOW &&
//     password <= &PASSWORD_HIGH &&
//     has_identical_pair(&pairs) &&
//     no_decreasing_pairs(&pairs)
// }

pub struct DayFour {}

impl Problem for DayFour {
    fn part_one(&self, input: &str) -> String {
        let valid_passwords: Vec<usize> = (PASSWORD_LOW..PASSWORD_HIGH + 1)
            .filter(has_identical_pair)
            .filter(no_decreasing_pairs)
            .collect();

        format!("{}", valid_passwords.len())
    }

    fn part_two(&self, input: &str) -> String {
        format!("{}", "Part two not yet implemented.")
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
