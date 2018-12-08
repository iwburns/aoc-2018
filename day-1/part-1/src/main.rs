fn main() {
    let input = include_str!("./input.txt");
    let total = get_total(input);
    println!("{}", total);
}

fn get_total(input: &str) -> i32 {
    input
        .lines()
        .map(|l| l.parse::<i32>().expect("couldn't parse i32 from line"))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "+1\n+1\n+1";
        assert_eq!(get_total(input), 3);
    }

    #[test]
    fn test_2() {
        let input = "+1\n+1\n-2";
        assert_eq!(get_total(input), 0);
    }

    #[test]
    fn test_3() {
        let input = "-1\n-2\n-3";
        assert_eq!(get_total(input), -6);
    }
}
