advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Result<u32, String> {
    let Input(rules, updates) = parser::obtain_input(input)?;

    let mut score: u32 = 0;

    for Update(update) in updates {
        let mut valid = true;

        for OrderingRule(first, second) in &rules {
            let maybe_first_index = update.iter().position(|e| *first == *e);
            let maybe_second_index = update.iter().position(|e| *second == *e);

            if let (Some(first_index), Some(second_index)) = (maybe_first_index, maybe_second_index)
            {
                valid &= first_index < second_index;
            }
        }

        if valid {
            score += update[update.len() / 2];

            println!("{:?} is valid", update)
        }
    }

    Ok(score)
}

pub fn part_two(input: &str) -> Result<u32, String> {
    let Input(rules, updates) = parser::obtain_input(input)?;

    let mut score: u32 = 0;

    let mut invalid: Vec<Vec<u32>> = Vec::new();

    for Update(mut update) in updates {
        let mut valid = true;

        for OrderingRule(first, second) in &rules {
            let maybe_first_index = update.iter().position(|e| *first == *e);
            let maybe_second_index = update.iter().position(|e| *second == *e);

            if let (Some(first_index), Some(second_index)) = (maybe_first_index, maybe_second_index)
            {
                valid &= first_index < second_index;
            }
        }

        if !valid {
            invalid.push(update.clone());
            println!("{:?} is invalid", update)
        }
    }

    for mut update in invalid {

        let mut changed = true;
        while changed {
            for OrderingRule(first, second) in &rules {
                let maybe_first_index = update.iter().position(|e| *first == *e);
                let maybe_second_index = update.iter().position(|e| *second == *e);

                if let (Some(first_index), Some(second_index)) =
                    (maybe_first_index, maybe_second_index)
                {
                    if first_index > second_index {
                        update.swap(first_index, second_index);
                        changed = true;
                        break;
                    } else {
                        changed &= false;
                    }
                }
            }

            println!("updated {:?}", update);
        }
        score += update[update.len() / 2];
    }

    Ok(score)
}

#[derive(Clone)]
struct OrderingRule(u32, u32);

#[derive(Clone)]
struct Update(Vec<u32>);

#[derive(Clone)]
struct Input(Vec<OrderingRule>, Vec<Update>);

mod parser {

    use nom::character::complete;
    use nom::combinator::map;
    use nom::error::VerboseError;
    use nom::multi::separated_list1;
    use nom::sequence::{terminated, tuple};
    use nom::IResult;

    type ParserResult<'a, U> = IResult<&'a str, U, VerboseError<&'a str>>;

    fn parse_rule(input: &str) -> ParserResult<crate::OrderingRule> {
        map(
            tuple((complete::u32, complete::char('|'), complete::u32)),
            |(l, _, r)| crate::OrderingRule(l, r),
        )(input)
    }

    fn parse_all_rules(input: &str) -> ParserResult<Vec<crate::OrderingRule>> {
        terminated(
            separated_list1(complete::newline, parse_rule),
            complete::newline,
        )(input)
    }

    fn parse_update(input: &str) -> ParserResult<crate::Update> {
        map(
            separated_list1(complete::char(','), complete::u32),
            crate::Update,
        )(input)
    }

    fn parse_all_updates(input: &str) -> ParserResult<Vec<crate::Update>> {
        terminated(
            separated_list1(complete::newline, parse_update),
            complete::newline,
        )(input)
    }

    fn parse_all(input: &str) -> ParserResult<crate::Input> {
        map(
            tuple((parse_all_rules, complete::newline, parse_all_updates)),
            |(x, _, y)| crate::Input(x, y),
        )(input)
    }

    pub fn obtain_input(file: &str) -> Result<crate::Input, String> {
        let (rest, parsed) = parse_all(file).map_err(|err| err.to_string())?;
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

        let expected = Ok(143);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        let expected = Ok(123);
        assert_eq!(result, expected);
    }
}
