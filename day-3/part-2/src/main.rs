use std::collections::HashMap;

#[derive(Debug)]
struct Claim {
    id: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl From<&str> for Claim {
    fn from(string: &str) -> Self {
        let parts: Vec<&str> = string.split(|c| c == '@' || c == ':').collect();

        let id = parts[0]
            .trim()
            .trim_matches(|c| c == '#')
            .parse()
            .unwrap_or(0);
        let coords: Vec<&str> = parts[1].trim().split(',').collect();
        let size: Vec<&str> = parts[2].trim().split('x').collect();

        Claim {
            id,
            x: coords[0].parse().unwrap_or(0),
            y: coords[1].parse().unwrap_or(0),
            width: size[0].parse().unwrap_or(0),
            height: size[1].parse().unwrap_or(0),
        }
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let id = find_non_overlapping_claim_id(input);
    println!("{}", id);
}

fn find_non_overlapping_claim_id(input: &str) -> u32 {
    let mut cloth = HashMap::new();

    let claims: Vec<Claim> = input.lines().map(Claim::from).collect();

    for claim in claims.iter() {
        claim_cloth_area(claim, &mut cloth);
    }

    for claim in claims.iter() {
        if !has_overlaps(claim, &cloth) {
            return claim.id;
        }
    }

    0
}

fn claim_cloth_area(claim: &Claim, cloth: &mut HashMap<(u32, u32), u32>) {
    let x_min = claim.x;
    let x_max = claim.x + claim.width;
    let y_min = claim.y;
    let y_max = claim.y + claim.height;

    for x in x_min..x_max {
        for y in y_min..y_max {
            *cloth.entry((x, y)).or_insert(0) += 1;
        }
    }
}

fn has_overlaps(claim: &Claim, cloth: &HashMap<(u32, u32), u32>) -> bool {
    let x_min = claim.x;
    let x_max = claim.x + claim.width;
    let y_min = claim.y;
    let y_max = claim.y + claim.height;

    for x in x_min..x_max {
        for y in y_min..y_max {
            let overlaps = cloth.get(&(x, y)).unwrap_or(&0);
            if overlaps > &1 {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_claim() {
        let claim_str = "#123 @ 3,2: 5x4";
        let claim = Claim::from(claim_str);
        assert_eq!(claim.id, 123);
        assert_eq!(claim.x, 3);
        assert_eq!(claim.y, 2);
        assert_eq!(claim.width, 5);
        assert_eq!(claim.height, 4);
    }
}
