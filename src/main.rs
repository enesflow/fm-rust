mod shapes;
use std::{fmt::Display, str::FromStr};

use crate::shapes::rectangle::Rectangle;
use anyhow::Result;
use shapes::{
    circle::Circle,
    collisions::{Collidable, Contains, PointIter, Points},
};

enum Shape {
    Rectangle(Rectangle),
    Circle(Circle),
}

impl FromStr for Shape {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (name, data) = s.split_once(" ").unwrap_or(("", ""));
        match name {
            "rect" => return Ok(Shape::Rectangle(data.parse()?)),
            "circle" => return Ok(Shape::Circle(data.parse()?)),
            _ => return Err(anyhow::anyhow!("Invalid shape")),
        }
    }
}

impl Points for Shape {
    fn points(&self) -> PointIter {
        match self {
            Shape::Rectangle(rect) => rect.points(),
            Shape::Circle(circle) => circle.points(),
        }
    }
}

impl Contains for Shape {
    fn contains_point(&self, point: (f64, f64)) -> bool {
        match self {
            Shape::Rectangle(rect) => rect.contains_point(point),
            Shape::Circle(circle) => circle.contains_point(point),
        }
    }
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Shape::Rectangle(rect) => write!(f, "{}", rect),
            Shape::Circle(circle) => write!(f, "{}", circle),
        }
    }
}

fn main() -> Result<()> {
    let shapes = std::fs::read_to_string("shapes")?
        .lines()
        .filter_map(|x| x.parse::<Shape>().ok())
        .collect::<Vec<_>>();
    shapes
        .iter()
        .skip(1)
        .zip(shapes.iter().take(shapes.len() - 1))
        .filter(|(a, b)| a.collide(*b))
        .for_each(|(a, b)| println!("{} collides with {}", a, b));

    return Ok(());
}
