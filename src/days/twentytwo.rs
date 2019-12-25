use itertools::Itertools;
use crate::problem::Problem;

type Deck = Vec<usize>;

fn new_deck() -> Deck {
    (0..10007).collect()
}

fn deal_into_new_stack(deck: Deck) -> Deck {
    deck.into_iter().rev().collect()
}

fn cut(mut deck: Deck, n: isize) -> Deck {
    let split = if n > 0 {
        n as usize
    } else {
        deck.len() - (n.abs() as usize)
    };

    let mut bottom = deck.split_off(split);
    bottom.extend(deck);
    bottom
}

fn deal_with_increment(deck: Deck, n: usize) -> Deck {
    let mut dealt = Deck::with_capacity(deck.len());
    dealt.resize(deck.len(), 0);

    for i in 0..deck.len() {
        dealt[(i * n) % deck.len()] = deck[i];
    }

    dealt
}

enum Technique {
    DealNewStack,
    Cut(isize),
    DealWithIncrement(usize)
}

impl Technique {
    // Getting lazy on vacation, don't feel like dealing with regexes and
    // error types.
    fn from_str(s: &str) -> Technique {
        if s.starts_with("cut") {
            Technique::Cut(
                isize::from_str_radix(s.get(4..).unwrap().trim(), 10).unwrap()
            )
        } else if s.starts_with("deal with") {
            Technique::DealWithIncrement(
                usize::from_str_radix(s.get(20..).unwrap().trim(), 10).unwrap()
            )
        } else {
            Technique::DealNewStack
        }
    }

    fn apply(&self, deck: Deck) -> Deck {
        match self {
            Technique::DealNewStack => deal_into_new_stack(deck),
            Technique::Cut(n) => cut(deck, *n),
            Technique::DealWithIncrement(n) => deal_with_increment(deck, *n)
        }
    }
}

pub struct DayTwentyTwo {}

impl Problem for DayTwentyTwo {
    fn part_one(&self, input: &str) -> String {
        input.lines()
            .map(Technique::from_str)
            .fold(new_deck(), |d, t| t.apply(d))
            .iter()
            .position(|&card| card == 2019)
            .unwrap()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        format!("{}", "Part two not yet implemented.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq};

    #[test]
    fn new_stack() {
        let deck = vec![0,1,2,3,4,5,6,7,8,9];
        let newdeck = deal_into_new_stack(deck);
        assert_eq!(vec![9,8,7,6,5,4,3,2,1,0], newdeck);
    }

    #[test]
    fn cut_deck() {
        let deck = vec![0,1,2,3,4,5,6,7,8,9];
        let cutdeck = cut(deck, 3);
        assert_eq!(vec![3,4,5,6,7,8,9,0,1,2], cutdeck);
    }

    #[test]
    fn cut_deck_negative() {
        let deck = vec![0,1,2,3,4,5,6,7,8,9];
        let cutdeck = cut(deck, -4);
        assert_eq!(vec![6,7,8,9,0,1,2,3,4,5], cutdeck);
    }

    #[test]
    fn deal_with_inc() {
        let deck = vec![0,1,2,3,4,5,6,7,8,9];
        let dealdeck = deal_with_increment(deck, 3);
        assert_eq!(vec![0,7,4,1,8,5,2,9,6,3], dealdeck);
    }
}
