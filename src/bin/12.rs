use cached::proc_macro::cached;
use itertools::Itertools;
advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<usize> {
    let lines = input.lines();
    let mut sum = 0;
    lines.for_each(|line| {
        let (row, pattern) = line.split_once(" ").unwrap();
        let x = arrangements(
            row.chars().collect(),
            pattern
                .split(",")
                .map(|s| s.parse::<usize>().unwrap())
                .collect(),
        );
        sum += x;
    });
    Some(sum)
}

#[cached]
fn arrangements(pixels_in: Vec<char>, groups: Vec<usize>) -> usize {
    let mut pixels = pixels_in.clone();
    if pixels.len() == 0 {
        return if groups.len() == 0 { 1 } else { 0 };
    } else if pixels[0] == '.' {
        pixels.remove(0);
        return arrangements(pixels, groups);
    } else if pixels[0] == '?' {
        let p = pixels.iter().map(|s| s.to_string()).join("");
        return arrangements(p.replacen("?", ".", 1).chars().collect(), groups.clone())
            + arrangements(p.replacen("?", "#", 1).chars().collect(), groups.clone());
    } else if pixels[0] == '#' {
        if groups.len() == 0 {
            return 0;
        }
        if pixels.len() < groups[0] {
            return 0;
        }
        for c in 0..groups[0] {
            if pixels[c] == '.' {
                return 0;
            }
        }
        if groups.len() > 1 {
            if pixels.len() < groups[0] + 1 || pixels[groups[0]] == '#' {
                return 0;
            }
            return arrangements(pixels[(groups[0] + 1)..].to_vec(), groups[1..].to_vec());
        } else {
            return arrangements(pixels[groups[0]..].to_vec(), groups[1..].to_vec());
        }
    }
    panic!("Parsing issue");
}
pub fn part_two(input: &str) -> Option<usize> {
    let lines = input.lines();
    let mut sum = 0;
    lines.for_each(|line| {
        let (row, pattern) = line.split_once(" ").unwrap();
        let mut r = (row.to_string() + "?").repeat(5);
        let mut p = (pattern.to_string() + ",").repeat(5);
        r = r[0..r.len() - 1].to_string();
        p = p[0..p.len() - 1].to_string();
        let x = arrangements(
            r.chars().collect(),
            p.split(",").map(|s| s.parse::<usize>().unwrap()).collect(),
        );
        sum += x;
    });
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
