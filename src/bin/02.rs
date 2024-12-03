advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Result<u32, String> {
    let data = parser::obtain_input(input)?;

    let score = data
        .iter()
        .map(|report| report.is_valid())
        .filter(|x| x.is_none())
        .count();

    Ok(score as u32)
}

pub fn part_two(input: &str) -> Result<u32, String> {
    let data = parser::obtain_input(input)?;

    let score = data
        .iter()
        .map(|report| report.is_valid_with_dampen())
        .filter(|x| x.is_none())
        .count();

    Ok(score as u32)
}

#[derive(Debug)]
pub struct Report(pub Vec<i32>);

impl Report {
    pub fn is_valid(&self) -> Option<usize> {
        let each_level_difference: Vec<i32> =
            self.0.windows(2).map(|data| data[0] - data[1]).collect();

        let position = each_level_difference
            .windows(2)
            .map(|data| {
                data[0].signum() == data[1].signum()
                    && data[0].abs() >= 1
                    && data[0].abs() <= 3
                    && data[1].abs() >= 1
                    && data[1].abs() <= 3
            })
            .position(|x| !x)
            .map(|x| x + 1);

        println!("{:?}", position);

        position
    }

    pub fn is_valid_with_dampen(&self) -> Option<usize> {

        for idx in 0..self.0.len() {
            let mut new_vector = self.0.clone();
            new_vector.remove(idx);
            let res = Report(new_vector).is_valid();
            if res.is_none() {
                return res;
            }
        }

        Some(self.0.len() )
    }
}

mod parser {

    use nom::character::complete::{digit1, line_ending, space1};
    use nom::combinator::{map, map_res};
    use nom::error::VerboseError;
    use nom::multi::separated_list1;
    use nom::sequence::terminated;
    use nom::IResult;

    type Res<'a, U> = IResult<&'a str, U, VerboseError<&'a str>>;
    pub fn parse_integer(input: &str) -> Res<i32> {
        map_res(digit1, |c: &str| c.parse::<i32>())(input)
    }

    pub fn parse_line(line: &str) -> Res<crate::Report> {
        return map(separated_list1(space1, parse_integer), crate::Report)(line);
    }

    pub fn parse_file(file: &str) -> Res<Vec<crate::Report>> {
        terminated(separated_list1(line_ending, parse_line), line_ending)(file)
    }

    pub fn obtain_input(file: &str) -> Result<Vec<crate::Report>, String> {
        let (rest, parsed) = parse_file(file).map_err(|err| err.to_string())?;

        if !rest.is_empty() {
            return Err("Not all input has been consumed".to_owned());
        }

        Ok(parsed)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Ok(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Ok(4));
    }
}
