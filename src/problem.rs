pub trait Problem {
    fn name(&self) -> String {
        "???".to_string()
    }

    fn part_one(&self, input: &str) -> String;
    fn part_two(&self, input: &str) -> String;
}
