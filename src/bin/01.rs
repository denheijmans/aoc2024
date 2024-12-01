use std::iter::zip;

use nom::{
    character::complete::{multispace1, newline, u32},
    multi::separated_list1,
    IResult,
};

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right) = parse(input);
    left.sort();
    right.sort();
    let diff = zip(left, right).map(|(l, r)| l.abs_diff(r)).sum();
    Some(diff)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left, right) = parse(input);
    let similarity = left
        .iter()
        .map(|l| *l * right.iter().filter(|r| **r == *l).count() as u32)
        .sum();
    Some(similarity)
}

fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    let (_, locations) = locations(input).unwrap();
    locations.iter().cloned().unzip()
}

fn locations(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    let (input, list) = separated_list1(newline, pair)(input)?;
    Ok((input, list))
}

fn pair(input: &str) -> IResult<&str, (u32, u32)> {
    let (input, left) = u32(input)?;
    let (input, _) = multispace1(input)?;
    let (input, right) = u32(input)?;
    Ok((input, (left, right)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
