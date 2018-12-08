use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let total = get_checksum(input);
    println!("{}", total);
}

fn get_checksum(input: &str) -> u32 {
    let (twos, threes) = input
        .lines()
        .map(into_counts)
        .fold((0, 0), |acc, curr| (acc.0 + curr.0, acc.1 + curr.1));
    twos * threes
}

fn into_counts(line: &str) -> (u32, u32) {
    let char_counts = line.chars().fold(HashMap::new(), |mut acc, curr| {
        *acc.entry(curr).or_insert(0) += 1;
        acc
    });

    let (twos, threes) = char_counts
        .values()
        .fold((0, 0), |(twos, threes), count| match count {
            2 => (1, threes),
            3 => (twos, 1),
            _ => (twos, threes),
        });

    (twos, threes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab";
        assert_eq!(get_checksum(input), 12);
    }
}
