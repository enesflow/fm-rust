use std::{fmt::Display, str::FromStr};

use crate::shapes::area::Area;

use super::collisions::{Contains, Points};

pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Points for Rectangle {
    fn points(&self) -> super::collisions::PointIter {
        return vec![
            (self.x, self.y),
            (self.x + self.width, self.y),
            (self.x + self.width, self.y + self.height),
            (self.x, self.y + self.height),
        ]
        .into();
    }
}

impl Contains for Rectangle {
    fn contains_point(&self, (x, y): (f64, f64)) -> bool {
        if self.x <= x && x <= self.x + self.width && self.y <= y && y <= self.y + self.height {
            return true;
        }
        return false;
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: 320.0,
            height: 240.0,
        }
    }
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Rectangle x: {}, y: {}, width: {}, height: {}",
            self.x, self.y, self.width, self.height
        )
    }
}

impl FromStr for Rectangle {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect::<Vec<_>>();
        if parts.len() != 4 {
            return Err(anyhow::anyhow!("Invalid number of parts"));
        }
        return Ok(Self {
            x: parts[0].parse()?,
            y: parts[1].parse()?,
            width: parts[2].parse()?,
            height: parts[3].parse()?,
        });
    }
}
