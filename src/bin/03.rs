use nom::{
    character::complete::{char, u32},
    IResult,
};

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    Some(multiply(input))
}

pub fn part_two(input: &str) -> Option<u32> {
    let total: u32 = input
        .split("do()")
        .map(|x| x.split("don't()").next().unwrap())
        .map(|x| multiply(x))
        .sum();
    Some(total)
}

fn multiply(input: &str) -> u32 {
    input
        .split("mul")
        .skip(1)
        .map(|x| mult(x).ok())
        .filter_map(|x| x)
        .map(|(_, (x, y))| x * y)
        .sum()
}

fn mult(input: &str) -> IResult<&str, (u32, u32)> {
    let (input, _) = char('(')(input)?;
    let (input, x) = u32(input)?;
    let (input, _) = char(',')(input)?;
    let (input, y) = u32(input)?;
    let (input, _) = char(')')(input)?;
    Ok((input, (x, y)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
