use itertools::Itertools;
use crate::problem::Problem;

fn index_to_pattern_val(index: usize, repeats: usize) -> isize {
    match ((index + 1) / repeats) % 4 {
        0 => 0,
        1 => 1,
        2 => 0,
        _ => -1
    }
}

fn next_phase_val(input: &[usize], index: usize) -> usize {
    input.iter()
        .zip((0..input.len()).map(|i| index_to_pattern_val(i, index)))
        .map(|(inpt, pat)| (*inpt as isize) * pat)
        .sum::<isize>()
        .abs() as usize % 10
}

fn next_phase(input: &[usize]) -> Vec<usize> {
    (0..input.len())
        .map(|i| next_phase_val(input, i + 1))
        .collect()
}

fn nth_phase(input: &[usize], n: usize) -> Vec<usize> {
    (0..n).fold(input.to_vec(), |acc, _| next_phase(&acc))
}

fn digits(input: &str) -> Vec<usize> {
    input.chars().map(|c| c.to_digit(10).unwrap() as usize).collect()
}

// For part two we have a very large list which would take an unreasonable
// amount of time, but the value we care about is close to the end of the list.
// This lets us cheat.
// In the back half of a phase, each new number is just the sum of it and all
// the numbers after it mod 10.  This means we can get the back part of a new
// phase by just doing a running sum going backwards and mod tenning the whole
// way. Here, we assume the input has already been truncated to only the part
// of the input we care about.
fn next_phase_cheating(input: &[usize]) -> Vec<usize> {
    let mut next = Vec::with_capacity(input.len());
    next.resize_with(input.len(), Default::default);
    let mut sum = 0;
    for (i, val) in input.iter().rev().enumerate() {
        sum = (sum + val) % 10;
        next[input.len() - i - 1] = sum;
    }
    next
}

fn nth_phase_cheating(input: &[usize], n: usize) -> Vec<usize> {
    (0..n).fold(input.to_vec(), |acc, _| next_phase_cheating(&acc))
}

pub struct DaySixteen {}

impl Problem for DaySixteen {
    fn part_one(&self, input: &str) -> String {
        nth_phase(&digits(input), 100).iter().take(8).join("").to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let base_input = digits(input);
        let skip_amount = base_input[0] * 1_000_000
                        + base_input[1] * 100_000
                        + base_input[2] * 10_000
                        + base_input[3] * 1_000
                        + base_input[4] * 100
                        + base_input[5] * 10
                        + base_input[6];
        let cheating_input: Vec<usize> = base_input.iter()
            .cycle()
            .skip(skip_amount)
            .take(base_input.len() * 10_000 - skip_amount)
            .map(|x| *x)
            .collect();

        println!("{} {}", skip_amount, cheating_input.len());

        nth_phase_cheating(&cheating_input, 100).iter().take(8).join("").to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
