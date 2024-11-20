
pub fn part_one(input: &str) -> Option<u32> {
    return Some(32);
}

pub fn part_two(input: &str) -> Option<u32> {
    return Some(32);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    part_one(input);
    part_two(input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}