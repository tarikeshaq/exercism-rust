// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    position: (i32, i32),
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot {
            position: (x, y),
            direction: d,
        }
    }

    pub fn turn_right(self) -> Self {
        match self.direction() {
            Direction::North => Robot {
                direction: Direction::East,
                ..self
            },
            Direction::East => Robot {
                direction: Direction::South,
                ..self
            },
            Direction::South => Robot {
                direction: Direction::West,
                ..self
            },
            Direction::West => Robot {
                direction: Direction::North,
                ..self
            },
        }
    }

    pub fn turn_left(self) -> Self {
        match self.direction() {
            Direction::North => Robot {
                direction: Direction::West,
                ..self
            },
            Direction::East => Robot {
                direction: Direction::North,
                ..self
            },
            Direction::South => Robot {
                direction: Direction::East,
                ..self
            },
            Direction::West => Robot {
                direction: Direction::South,
                ..self
            },
        }
    }

    pub fn advance(self) -> Self {
        let (x, y) = self.position();
        match self.direction() {
            Direction::North => Robot {
                position: (x, y + 1),
                ..self
            },
            Direction::East => Robot {
                position: (x + 1, y),
                ..self
            },
            Direction::South => Robot {
                position: (x, y - 1),
                ..self
            },
            Direction::West => Robot {
                position: (x - 1, y),
                ..self
            },
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |rbt, c| match c {
            'R' => rbt.turn_right(),
            'L' => rbt.turn_left(),
            'A' => rbt.advance(),
            _ => rbt,
        })
    }

    pub fn position(&self) -> (i32, i32) {
        self.position
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
