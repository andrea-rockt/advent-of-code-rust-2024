advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Result<u32, String> {
    let input = parser::obtain_input(input)?;

    Ok(input
        .iter()
        .map(|x| match x {
            Instructions::MultiplyInstruction(l, r) => (l * r) as u32,
            _ => 0_u32,
        })
        .sum())
}

pub fn part_two(input: &str) -> Result<u32, String> {
    let mut enabled = true;
    let mut score: u32 = 0;

    let input = parser::obtain_input(input)?;

    for instruction in input {
        match instruction {
            Instructions::Do => enabled = true,
            Instructions::Dont => enabled = false,
            Instructions::MultiplyInstruction(l, r) => {
                if enabled {
                    score += (l * r) as u32
                }
            }
        }
    }

    Ok(score)
}

#[derive(Debug, Clone, Copy)]
enum Instructions {
    MultiplyInstruction(i32, i32),
    Do,
    Dont,
}

mod parser {

    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{anychar, char, digit1};
    use nom::combinator::{map, map_res, verify};
    use nom::error::VerboseError;
    use nom::multi::many0;
    use nom::sequence::tuple;
    use nom::IResult;

    use crate::Instructions;

    type Res<'a, U> = IResult<&'a str, U, VerboseError<&'a str>>;
    pub fn parse_integer(input: &str) -> Res<i32> {
        let parse_integer_no_restriction = map_res(digit1, |c: &str| c.parse::<i32>());
        verify(parse_integer_no_restriction, |x| *x < 1000)(input)
    }

    pub fn parse_multiply(line: &str) -> Res<Option<crate::Instructions>> {
        let raw_mul = tuple((
            tag("mul"),
            tag("("),
            parse_integer,
            char(','),
            parse_integer,
            tag(")"),
        ));

        return map(raw_mul, |(_, _, l, _, r, _)| {
            Some(Instructions::MultiplyInstruction(l, r))
        })(line);
    }

    pub fn parse_do(line: &str) -> Res<Option<crate::Instructions>> {
        return map(tag("do()"), |_| Some(Instructions::Do))(line);
    }

    pub fn parse_dont(line: &str) -> Res<Option<crate::Instructions>> {
        return map(tag("don't()"), |_| Some(Instructions::Dont))(line);
    }

    pub fn parse_any_char(line: &str) -> Res<Option<crate::Instructions>> {
        map(anychar, |_| None::<crate::Instructions>)(line)
    }

    pub fn parse_file(file: &str) -> Res<Vec<crate::Instructions>> {
        map(
            many0(alt((parse_multiply, parse_do, parse_dont, parse_any_char))),
            |f| f.iter().copied().flatten().collect(),
        )(file)
    }

    pub fn obtain_input(file: &str) -> Result<Vec<crate::Instructions>, String> {
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
        let result = part_one(
            advent_of_code::template::read_file("examples", DAY)
                .lines()
                .next()
                .expect("we should have two lines")
        );
        assert_eq!(result, Ok(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(
            advent_of_code::template::read_file("examples", DAY)
                .lines()
                .nth(1)
                .expect("we should have two lines")
        );
        assert_eq!(result, Ok(48));
    }

    #[test]
    fn test_multiply_parser() {
        let res = parser::parse_multiply("mul(1,43)");

        println!("{:?}", res)
    }
}
