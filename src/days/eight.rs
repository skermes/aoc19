use itertools::Itertools;
use std::collections::HashMap;

use crate::problem::Problem;

const IMAGE_WIDTH: usize = 25;
const IMAGE_HEIGHT: usize = 6;
const IMAGE_PIXELS: usize = IMAGE_WIDTH * IMAGE_HEIGHT;

const BLACK: char = '0';
const WHITE: char = '1';
const BLACK_STR: &str = " ";
const WHITE_STR: &str = "#";

// For some reason char() plus itertools' chunks() is not working for me and
// I'm angry about it.
fn str_chunks(s: &str, size: usize) -> Vec<&str> {
    if size >= s.len() {
        vec![s]
    } else {
        // Splitting the string in reverse like this so that we can do it
        // append-only.  THis would mean that the first chunk may be undersized
        // if s.len() % size != 0, but we happen to only be using this for
        // proper chunk sizes.
        let (front, back) = s.split_at(s.len() - size);
        let mut chunks = str_chunks(front, size);
        chunks.push(back);
        chunks
    }
}

fn count_chars(s: &str) -> HashMap<char, usize> {
    let mut counts = HashMap::new();

    for c in s.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    counts
}

pub struct DayEight {}

impl Problem for DayEight {
    fn name(&self) -> String {
        "Space Image Format".to_string()
    }

    fn part_one(&self, input: &str) -> String {
        let chunks = str_chunks(input.trim(), IMAGE_PIXELS);
        let counts = chunks.iter().map(|chunk| count_chars(chunk));

        // I feel like I should be able to use min_by_key here but good lord
        // does the compiler not want to let me.
        let mut least_zeroes = std::usize::MAX;
        let mut ones_and_twos = 0;
        for count in counts {
            if count.get(&'0').or(Some(&0)).unwrap() < &least_zeroes {
                least_zeroes = *count.get(&'0').or(Some(&0)).unwrap();
                ones_and_twos = count.get(&'1').or(Some(&0)).unwrap() * count.get(&'2').or(Some(&0)).unwrap()
            }
        }

        format!("{}", ones_and_twos)
    }

    fn part_two(&self, input: &str) -> String {
        let layers = str_chunks(input.trim(), IMAGE_PIXELS);

        let mut message = "\n".to_string();
        for row in 0..IMAGE_HEIGHT {
            for col in 0..IMAGE_WIDTH {
                let index = row * IMAGE_WIDTH + col;
                for layer in &layers {
                    let c = layer.chars().nth(index).unwrap();
                    if c == BLACK {
                        message.push_str(BLACK_STR);
                        break;
                    } else if c == WHITE {
                        message.push_str(WHITE_STR);
                        break;
                    }
                }
            }
            message.push_str("\n");
        }

        message
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
