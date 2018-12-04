use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");
    let total = get_total(input);
    println!("{}", total);
}

fn get_total(input: &str) -> i32 {
    let mut frequencies = HashSet::new();

    let deltas = input
        .lines()
        .map(|l| l.parse::<i32>().expect("couldn't parse number from line"))
        .cycle();

    let mut total = 0;

    for delta in deltas {
        frequencies.insert(total);

        total += delta;

        if frequencies.contains(&total) {
            break;
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "+1\n-1";
        assert_eq!(get_total(input), 0);
    }

    #[test]
    fn test_2() {
        let input = "+3\n+3\n+4\n-2\n-4";
        assert_eq!(get_total(input), 10);
    }

    #[test]
    fn test_3() {
        let input = "+7\n+7\n-2\n-7\n-4";
        assert_eq!(get_total(input), 14);
    }
}
