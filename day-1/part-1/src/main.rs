fn main() {
    let input = include_str!("./input.txt");
    let total = get_total(input);
    println!("{}", total);
}

fn get_total(input: &str) -> i32 {
    input.lines().map(into_number).sum()
}

fn into_number(line: &str) -> i32 {
    let (sign, num) = line.split_at(1);

    let amount: i32 = num.parse().expect("couldn't parse number from line");

    match sign {
        "+" => amount,
        "-" => -amount,
        _ => unreachable!()
    }
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
