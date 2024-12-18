use std::{fmt::Display, usize};

#[derive(PartialEq, Eq, Clone)]
pub struct Vec2d {
    pub x: i32,
    pub y: i32,
}

#[derive(PartialEq, Eq, Clone)]
pub enum Direction {
    EAST,
    SOUTH,
    WEST,
    NORTH
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::EAST => write!(f, ">"),
            Direction::SOUTH => write!(f, "v"),
            Direction::WEST => write!(f, "<"),
            Direction::NORTH => write!(f, "^"),
        }
    }
}

impl Direction {
   pub fn from_char(ch: char) -> Option<Direction> {
       println!("{}", Direction::EAST);
       match ch {
           '>' => Some(Direction::EAST),
           'v' => Some(Direction::SOUTH),
           '<' => Some(Direction::WEST),
           '^' => Some(Direction::NORTH),
           _ => None
       }
   }

   pub fn values() -> Vec<Direction> {
       vec![Direction::EAST, Direction::SOUTH, Direction::WEST, Direction::NORTH]
   }

   pub fn turn(&self, clockwise: bool) -> Direction {
       let directions = Direction::values();
       for i in 0..(directions.len() as i32) {
           if *self == directions[i as usize] {
               let next = if clockwise {i+1} else {i-1};
               return directions[next.rem_euclid(4) as usize].clone();
           }
       }
       self.clone()
   }

   pub fn get_offset(&self) -> Vec2d {
       match *self {
           Direction::EAST => Vec2d { x: 1, y: 0 },
           Direction::SOUTH  => Vec2d { x: 0, y: 1 },
           Direction::WEST  => Vec2d { x:-1, y: 0 },
           Direction::NORTH    => Vec2d { x: 0, y:-1 },
       }
   }
}

impl Vec2d {
    pub fn access_arr<T: Clone>(&self, vec_2d: &Vec<Vec<T>>) -> Option<T> {
        if self.y < 0 || self.y >= vec_2d.len() as i32 ||
            self.x < 0 || self.x >= vec_2d[0].len() as i32 {
            return None;
        }
        return Some(vec_2d[self.y as usize][self.x as usize].clone());
    }

    pub fn put_arr<T>(&self, vec_2d: &mut Vec<Vec<T>>, val: T) {
        if self.y >= 0 && self.y < vec_2d.len() as i32 &&
            self.x >= 0 && self.x < vec_2d[0].len() as i32 {
            vec_2d[self.y as usize][self.x as usize] = val;
        }
    }

    pub fn from_usize(r: usize, c: usize) -> Vec2d {
        Vec2d { x: c as i32, y: r as i32 }
    }

    pub fn offset(&self, d: &Direction) -> Vec2d {
        match d {
           Direction::EAST => Vec2d{ x: self.x + 1, y: self.y     },
           Direction::SOUTH  => Vec2d{ x: self.x,     y: self.y + 1 },
           Direction::WEST  => Vec2d{ x: self.x - 1, y: self.y     },
           Direction::NORTH    => Vec2d{ x: self.x,     y: self.y - 1 },
        }
    }

    pub fn wrap<T>(&self, vec_2d: &Vec<Vec<T>>) -> Vec2d {
        Vec2d {
            x: self.x.rem_euclid(vec_2d[0].len() as i32),
            y: self.y.rem_euclid(vec_2d.len() as i32)
        }
    }
}
