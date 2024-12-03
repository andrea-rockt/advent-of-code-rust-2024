use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Result<u32, String> {
    let (mut first_vector, mut second_vector) = parser::obtain_input(input)?;

    first_vector.sort();
    second_vector.sort();

    let res = first_vector
        .iter()
        .zip(second_vector.iter())
        .fold(0, |acc, (a, b)| acc + a.abs_diff(*b));

    Ok(res)
}

pub fn part_two(input: &str) -> Result<u32, String> {
    let (first_vector, second_vector) = parser::obtain_input(input)?;

    let mut hash_map = HashMap::<u32, u32>::new();

    let zero: u32 = 0;

    for ele in second_vector {
        let current = hash_map.get(&ele).unwrap_or(&zero) + 1;
        hash_map.insert(ele, current);
    }

    Ok(first_vector.iter().fold(0, |acc, element| {
        acc + (element * hash_map.get(element).unwrap_or(&zero))
    }))
}

mod parser {

    use nom::character::complete::{digit1, line_ending, space1};
    use nom::combinator::{map, map_res};
    use nom::error::VerboseError;
    use nom::multi::separated_list1;
    use nom::sequence::{terminated, tuple};
    use nom::IResult;

    type Res<'a, U> = IResult<&'a str, U, VerboseError<&'a str>>;

    #[derive(Debug)]
    pub struct Line(u32, u32);

    pub fn parse_unsigned_integer(input: &str) -> Res<u32> {
        map_res(digit1, |c: &str| c.parse::<u32>())(input)
    }

    pub fn parse_line(line: &str) -> Res<Line> {
        return map(
            tuple((
                terminated(parse_unsigned_integer, space1),
                parse_unsigned_integer,
            )),
            |(l, r)| Line(l, r),
        )(line);
    }

    pub fn parse_file(file: &str) -> Res<Vec<Line>> {
        separated_list1(line_ending, parse_line)(file)
    }

    pub fn obtain_input(file: &str) -> Result<(Vec<u32>, Vec<u32>), String> {
        let (rest, parsed) = parse_file(file).map_err(|err| err.to_string())?;

        if !rest.trim().is_empty() {
            return Err("Not all input has been consumed".to_owned());
        }

        let first_vector = &mut Vec::<u32>::with_capacity(parsed.len());
        let second_vector = &mut Vec::<u32>::with_capacity(parsed.len());

        for index in 0..parsed.len() {
            first_vector.push(parsed[index].0);
            second_vector.push(parsed[index].1);
        }

        Ok((first_vector.clone(), second_vector.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Ok(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Ok(31));
    }
}
