use std::convert::TryFrom;
use std::str::FromStr;

fn bad_current_position_err(x: usize, y: usize, node: &Node) -> anyhow::Error {
    anyhow::anyhow!(format!(
        "Bad current_position ({}, {}). Found: {:?}",
        x, y, node
    ))
}

#[derive(Clone, Copy, Debug)]
enum Node {
    CurrentPosition(Direction),
    Visited,
    Empty,
    Obstruction,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl TryFrom<char> for Node {
    type Error = anyhow::Error;

    fn try_from(char: char) -> anyhow::Result<Self> {
        match char {
            '^' => Ok(Node::CurrentPosition(Direction::North)),
            '.' => Ok(Node::Empty),
            '#' => Ok(Node::Obstruction),
            _ => Err(anyhow::anyhow!(format!("unrecognized char: '{}'", char))),
        }
    }
}

type Coords = (usize, usize);

pub struct Map {
    current_position: Coords,
    grid: Vec<Vec<Node>>,
    max_x: usize,
    max_y: usize,
}

impl Map {
    pub fn run_protocol(&mut self) {
        let mut counter = 100_000;
        while let Some(_) = self.try_move() {
            counter -= 1;
            if counter == 0 {
                println!("stopping early. you took 100,000 moves...");
                return;
            }
        }
    }

    pub fn count_visited(&self) -> i64 {
        self.grid
            .iter()
            .flatten()
            .fold(0, |acc, node: &Node| match node {
                Node::Visited => acc + 1,
                Node::CurrentPosition(_) => acc + 1,
                _ => acc,
            })
    }

    fn try_move(&mut self) -> Option<()> {
        let next_direction = self.next_valid_direction()?;
        let (x, y) = self.next_move_for_direction(&next_direction)?;
        self.set_position(x, y, next_direction);
        Some(())
    }

    fn set_position(&mut self, x: usize, y: usize, direction: Direction) {
        let (curr_x, curr_y) = self.current_position;
        self.grid[curr_y][curr_x] = Node::Visited;
        self.grid[y][x] = Node::CurrentPosition(direction);
        self.current_position = (x, y);
    }

    fn next_valid_direction(&self) -> Option<Direction> {
        let mut directions = self.clockwise_directions_from(
            self.current_direction()
                .expect("could not get current direction (???)"),
        );

        while directions.len() > 0 {
            let direction = directions.remove(0);
            let node = self.next_node_for_direction(&direction)?;

            match node {
                Node::Obstruction => continue,
                _ => return Some(direction),
            }
        }

        None
    }

    fn clockwise_directions_from(&self, direction: Direction) -> Vec<Direction> {
        let mut all_directions = vec![
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ];

        while all_directions[0] != direction {
            all_directions.rotate_left(1);
        }

        all_directions
    }

    fn next_node_for_direction(&self, direction: &Direction) -> Option<Node> {
        self.next_move_for_direction(direction)
            .map(|(x, y)| self.grid[y][x])
    }

    fn next_move_for_direction(&self, direction: &Direction) -> Option<Coords> {
        let (curr_x, curr_y) = self.current_position;
        match direction {
            Direction::North => {
                if curr_y > 0 {
                    Some((curr_x, curr_y - 1))
                } else {
                    None
                }
            }
            Direction::East => {
                if curr_x < self.max_x {
                    Some((curr_x + 1, curr_y))
                } else {
                    None
                }
            }
            Direction::South => {
                if curr_y < self.max_y {
                    Some((curr_x, curr_y + 1))
                } else {
                    None
                }
            }
            Direction::West => {
                if curr_x > 0 {
                    Some((curr_x - 1, curr_y))
                } else {
                    None
                }
            }
        }
    }

    fn current_direction(&self) -> anyhow::Result<Direction> {
        let (x, y) = self.current_position;
        match &self.grid[y][x] {
            Node::CurrentPosition(direction) => Ok(*direction),
            node => Err(bad_current_position_err(x, y, node)),
        }
    }
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let mut current_position = None;
        let mut grid: Vec<Vec<Node>> = vec![];

        for (y, line) in s.lines().enumerate() {
            let row: Vec<Node> = line
                .chars()
                .enumerate()
                .map(|(x, char)| {
                    let node = Node::try_from(char)?;
                    if matches!(node, Node::CurrentPosition(_)) {
                        current_position = Some((x, y))
                    }

                    Ok(node)
                })
                .collect::<anyhow::Result<Vec<_>>>()?;

            grid.push(row);
        }

        let max_x = grid[0].len() - 1;
        let max_y = grid.len() - 1;

        match current_position {
            Some((x, y)) => Ok(Map {
                grid,
                current_position: (x, y),
                max_x,
                max_y,
            }),
            None => Err(anyhow::anyhow!("Could not find a current position")),
        }
    }
}
