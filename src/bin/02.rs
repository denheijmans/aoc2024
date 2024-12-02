use nom::{
    character::complete::{i32, newline, space1},
    multi::separated_list1,
    IResult,
};

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let (_, reports) = reports(input).unwrap();
    let safe_reports = reports.iter().filter(|x| is_safe(*x)).count() as u32;
    Some(safe_reports)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, reports) = reports(input).unwrap();
    let safe_reports = reports
        .iter()
        .filter(|x| is_safe(*x) || is_safe_after_removal(*x))
        .count() as u32;
    Some(safe_reports)
}

fn is_safe_after_removal(report: &Vec<i32>) -> bool {
    for i in 0..report.len() {
        let mut modified_report = report.clone();
        modified_report.remove(i);
        if is_safe(&modified_report) {
            return true;
        }
    }
    return false;
}

fn is_safe(report: &Vec<i32>) -> bool {
    let derivative: Vec<i32> = report.windows(2).map(|w| w[1] - w[0]).collect();
    let strictly_monotonic = derivative.windows(2).map(|w| w[0] * w[1]).all(|x| x > 0);
    let in_bounds = derivative.iter().all(|&x| x.abs() <= 3);
    strictly_monotonic && in_bounds
}

fn reports(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    let (input, list) = separated_list1(newline, report)(input)?;
    Ok((input, list))
}

fn report(input: &str) -> IResult<&str, Vec<i32>> {
    let (input, list) = separated_list1(space1, i32)(input)?;
    Ok((input, list))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
