use crate::problem::Problem;

fn mass2fuel(mass: usize) -> usize {
    // Integer division should round down automatically for us.
    let fuel: isize = ((mass / 3) as isize) - 2;
    if fuel < 0 {
        0
    } else {
        fuel as usize
    }
}

fn fuel_for_mass_and_fuel(mass: usize) -> usize {
    let additional_fuel = mass2fuel(mass);
    if additional_fuel == 0 {
        0
    } else {
        additional_fuel + fuel_for_mass_and_fuel(additional_fuel)
    }
}

pub struct DayOne {}

impl Problem for DayOne {
    fn name(&self) -> String {
        "The Tyranny of the Rocket Equation".to_string()
    }

    fn part_one(&self, input: &str) -> String {
        let total_fuel: usize = input.split_whitespace()
            .map(|mstr| usize::from_str_radix(mstr, 10).unwrap())
            .map(mass2fuel)
            .sum();
        format!("{}", total_fuel)
    }

    fn part_two(&self, input: &str) -> String {
        let total_fuel: usize = input.split_whitespace()
            .map(|mstr| usize::from_str_radix(mstr, 10).unwrap())
            .map(fuel_for_mass_and_fuel)
            .sum();
        format!("{}", total_fuel)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mass2fuel_examples() {
        assert_eq!(2, mass2fuel(12));
        assert_eq!(2, mass2fuel(14));
        assert_eq!(654, mass2fuel(1969));
        assert_eq!(33583, mass2fuel(100756));
    }

    #[test]
    fn mass2fuel_small() {
        assert_eq!(0, mass2fuel(2));
    }

    #[test]
    fn fuel_for_fuel_examples() {
        assert_eq!(2, fuel_for_mass_and_fuel(14));
        assert_eq!(966, fuel_for_mass_and_fuel(1969));
        assert_eq!(50346, fuel_for_mass_and_fuel(100756));
    }
}
