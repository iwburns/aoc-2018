fn main() {
    let input = include_str!("./input.txt");
    let total = get_text(input);
    println!("{}", total);
}

fn get_text(input: &str) -> String {
    for line_a in input.lines() {
        for line_b in input.lines() {
            let (distance, common_text) = process_lines(line_a, line_b);
            if distance == 1 {
                return common_text;
            }
        }
    }

    String::from("match not found")
}

fn process_lines(line_a: &str, line_b: &str) -> (u32, String) {
    let (distance, common) =
        line_a
            .chars()
            .zip(line_b.chars())
            .fold((0, String::new()), |(dist, mut text), (a, b)| {
                if a == b {
                    text.push(a);
                    return (dist, text);
                }
                (dist + 1, text)
            });

    (distance, common)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz";
        assert_eq!(get_text(input), "fgij");
    }
}
