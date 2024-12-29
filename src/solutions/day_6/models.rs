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

#[derive(Clone, Copy, Debug)]
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
    finished: bool,
    grid: Vec<Vec<Node>>,
    max_x: usize,
    max_y: usize,
}

impl Map {
    pub fn run_protocol(&mut self) {}

    fn try_move(&mut self) {}

    fn next_valid_direction(&self) -> Option<Direction> {}

    fn next_node_for_direction(&self, direction: &Direction) -> Option<Node> {
        self.next_move_for_direction(direction)
            .map(|(x, y)| self.grid[x][y])
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
        match &self.grid[x][y] {
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

        match current_position {
            Some((x, y)) => Ok(Map {
                finished: false,
                grid,
                current_position: (x, y),
                max_x: grid[0].len() - 1,
                max_y: grid.len() - 1,
            }),
            None => Err(anyhow::anyhow!("Could not find a current position")),
        }
    }
}
