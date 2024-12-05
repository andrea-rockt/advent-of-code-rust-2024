advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Result<u32, String> {
    let data = parser::obtain_input(input)?;

    let grid: Grid = Grid::new(data);

    let mut score = 0;

    let to_searh = vec!['X', 'M', 'A', 'S'];

    for from in &grid.iterate() {
        for direction in Direction::iterate() {
            score += match grid.cast_ray(from, direction, 4) {
                Some(data) if data == to_searh => 1,
                _ => 0,
            }
        }
    }

    Ok(score)
}

pub fn part_two(input: &str) -> Result<u32, String> {
    let data = parser::obtain_input(input)?;

    let grid: Grid = Grid::new(data);

    let mut score = 0;

    let mas = vec!['M', 'A', 'S'];
    let sam = vec!['S', 'A', 'M'];

    for from in &grid.iterate() {
        let mut inner = 0;
        for direction in Direction::diagonals() {
            inner += match grid.cast_ray(&grid.move_delta(from, direction.flip()), direction, 3) {
                Some(data) if data == mas || data == sam => 1,
                _ => 0,
            }
        }

        if inner == 2 {
            score += 1;
        }
    }

    Ok(score)
}

struct Grid {
    data: Vec<char>,
    width: usize,
    height: usize,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum Position {
    Inside { x: usize, y: usize },
    Outside,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Direction {
    x: i32,
    y: i32,
}

impl Direction {
    pub fn diagonals() -> Vec<Direction> {
        vec![Direction { x: -1, y: 1 }, Direction { x: 1, y: 1 }]
    }
    pub fn iterate() -> Vec<Direction> {
        [-1, 0, 1]
            .iter()
            .copied()
            .flat_map(|x| [-1, 0, 1].iter().copied().map(move |y| (x, y)))
            .filter(|(x, y)| !((*x == 0) && (*y == 0)))
            .map(|(x, y)| Direction { x, y })
            .collect()
    }

    pub fn flip(&self) -> Direction {
        Direction {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Grid {
    pub fn at(&self, at: &Position) -> Option<char> {
        match at {
            Position::Outside => None,
            Position::Inside { x, y } => Some(self.data[y * self.width + x]),
        }
    }

    pub fn cast_ray(&self, from: &Position, direction: Direction, len: usize) -> Option<Vec<char>> {
        let first_char = self.at(from)?;

        let mut data: Vec<char> = Vec::new();
        data.push(first_char);

        let mut current_pos = *from;

        for _ in 1..len {
            current_pos = self.move_delta(&current_pos, direction);

            match self.at(&current_pos) {
                None => return Some(data),
                Some(character) => data.push(character),
            }
        }

        if data.is_empty() {
            None
        } else {
            Some(data)
        }
    }

    pub fn move_delta(&self, from: &Position, direction: Direction) -> Position {
        match from {
            Position::Outside => Position::Outside,
            Position::Inside {
                x: prev_x,
                y: prev_y,
            } => {
                let new_x = *prev_x as i32 + direction.x;
                let new_y = *prev_y as i32 + direction.y;

                if (new_x >= 0 && new_x < self.width as i32)
                    && (new_y >= 0 && new_y < self.height as i32)
                {
                    Position::Inside {
                        x: new_x as usize,
                        y: new_y as usize,
                    }
                } else {
                    Position::Outside
                }
            }
        }
    }

    pub fn new(in_data: Vec<Vec<char>>) -> Grid {
        Grid {
            data: in_data.iter().flatten().copied().collect(),
            width: in_data.first().map(|x| x.len()).unwrap_or(0),
            height: in_data.len(),
        }
    }

    pub fn iterate(&self) -> Vec<Position> {
        (0..self.height)
            .flat_map(|y| (0..self.width).map(move |x| (x, y)))
            .map(|(x, y)| Position::Inside { x, y })
            .collect()
    }
}

mod parser {

    use nom::bytes::complete::take_till1;
    use nom::character::complete::newline;
    use nom::combinator::map;
    use nom::error::VerboseError;
    use nom::multi::separated_list1;
    use nom::IResult;

    type ParserResult<'a, U> = IResult<&'a str, U, VerboseError<&'a str>>;

    fn parse_row(input: &str) -> ParserResult<Vec<char>> {
        map(take_till1(|x| x == '\n'), |x: &str| x.chars().collect())(input)
    }

    /// Parser for multiple rows, producing Vec<Vec<char>>
    fn parse_rows(input: &str) -> ParserResult<Vec<Vec<char>>> {
        separated_list1(newline, parse_row)(input)
    }

    pub fn obtain_input(file: &str) -> Result<Vec<Vec<char>>, String> {
        let (rest, parsed) = parse_rows(file).map_err(|err| err.to_string())?;
        if !rest.trim().is_empty() {
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
        assert_eq!(result, Ok(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Ok(9));
    }
}
