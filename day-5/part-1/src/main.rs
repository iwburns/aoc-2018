#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Unit {
    value: char,
    polarity: bool,
}

impl From<char> for Unit {
    fn from(c: char) -> Self {
        assert!(c.is_ascii());

        Unit {
            value: c.to_ascii_lowercase(),
            polarity: c.is_ascii_uppercase(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Polymer {
    units: Vec<Unit>,
}

impl From<&str> for Polymer {
    fn from(s: &str) -> Self {
        Polymer {
            units: s.trim().chars().map(Unit::from).collect(),
        }
    }
}

impl Polymer {
    pub fn reduce(&mut self) {
        let mut reacted = self.react_adjacent_elements();
        while reacted.len() != self.units.len() {
            self.units = reacted;
            reacted = self.react_adjacent_elements();
        }
    }

    pub(crate) fn react_adjacent_elements(&self) -> Vec<Unit> {
        let mut reduced = Vec::with_capacity(self.units.len());

        let mut left = self.units.iter();
        let mut right = self.units.iter().skip(1);

        loop {
            match (left.next(), right.next()) {
                (Some(&l), Some(&r)) => {
                    if l.value == r.value && l.polarity != r.polarity {
                        left.next();
                        right.next();
                    } else {
                        reduced.push(l);
                    }
                }
                (Some(&l), _) => {
                    reduced.push(l);
                }
                (_, _) => {
                    break;
                }
            }
        }

        reduced
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let mut polymer = Polymer::from(input);
    polymer.reduce();
    println!("{}", polymer.units.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn react_adjacent_elements_empty() {
        let input = "";
        let polymer = Polymer::from(input);

        let reacted = polymer.react_adjacent_elements();
        let expected: Vec<Unit> = Vec::new();

        assert_eq!(reacted, expected);
    }

    #[test]
    fn react_adjacent_elements_1() {
        let input = "aA";
        let polymer = Polymer::from(input);

        let reacted = polymer.react_adjacent_elements();
        let expected: Vec<Unit> = Vec::new();

        assert_eq!(reacted, expected);
    }

    #[test]
    fn react_adjacent_elements_2() {
        let input = "abBA";
        let mut polymer = Polymer::from(input);

        let reacted = polymer.react_adjacent_elements();
        let expected: Vec<Unit> = vec![Unit::from('a'), Unit::from('A')];

        assert_eq!(reacted, expected);

        polymer.units = reacted;
        let reacted = polymer.react_adjacent_elements();
        let expected: Vec<Unit> = Vec::new();

        assert_eq!(reacted, expected);
    }

    #[test]
    fn react_adjacent_elements_no_match() {
        let input = "abAB";
        let polymer = Polymer::from(input);

        let reacted = polymer.react_adjacent_elements();
        let expected: Vec<Unit> = vec![
            Unit::from('a'),
            Unit::from('b'),
            Unit::from('A'),
            Unit::from('B'),
        ];

        assert_eq!(reacted, expected);
    }

    #[test]
    fn react_adjacent_elements_no_match_2() {
        let input = "aabAAB";
        let polymer = Polymer::from(input);

        let reacted = polymer.react_adjacent_elements();
        let expected: Vec<Unit> = vec![
            Unit::from('a'),
            Unit::from('a'),
            Unit::from('b'),
            Unit::from('A'),
            Unit::from('A'),
            Unit::from('B'),
        ];

        assert_eq!(reacted, expected);
    }

    #[test]
    fn reduce_empty() {
        let input = "";
        let mut polymer = Polymer::from(input);

        polymer.reduce();

        let expected: Vec<Unit> = Vec::new();

        assert_eq!(polymer.units, expected);
    }

    #[test]
    fn reduce_1() {
        let input = "aA";
        let mut polymer = Polymer::from(input);

        polymer.reduce();

        let expected: Vec<Unit> = Vec::new();

        assert_eq!(polymer.units, expected);
    }

    #[test]
    fn reduce_2() {
        let input = "abBA";
        let mut polymer = Polymer::from(input);

        polymer.reduce();
        let expected: Vec<Unit> = Vec::new();

        assert_eq!(polymer.units, expected);
    }

    #[test]
    fn reduce_no_match() {
        let input = "abAB";
        let mut polymer = Polymer::from(input);

        polymer.reduce();
        let expected: Vec<Unit> = vec![
            Unit::from('a'),
            Unit::from('b'),
            Unit::from('A'),
            Unit::from('B'),
        ];

        assert_eq!(polymer.units, expected);
    }

    #[test]
    fn reduce_no_match_2() {
        let input = "aabAAB";
        let mut polymer = Polymer::from(input);

        polymer.reduce();
        let expected: Vec<Unit> = vec![
            Unit::from('a'),
            Unit::from('a'),
            Unit::from('b'),
            Unit::from('A'),
            Unit::from('A'),
            Unit::from('B'),
        ];

        assert_eq!(polymer.units, expected);
    }

    #[test]
    fn reduce_long() {
        let input = "dabAcCaCBAcCcaDA";
        let mut polymer = Polymer::from(input);

        polymer.reduce();

        let mut expected_polymer = Polymer::from("dabCBAcaDA");

        assert_eq!(polymer.units, expected_polymer.units);

        expected_polymer.reduce();

        assert_eq!(polymer.units, expected_polymer.units);
    }
}
