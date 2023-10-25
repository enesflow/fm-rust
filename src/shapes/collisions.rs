pub trait Collidable<T> {
    fn collide(&self, other: &T) -> bool;
    fn collide_with(&self, others: &[T]) -> bool {
        for other in others {
            if self.collide(other) {
                return true;
            }
        }
        return false;
    }
}

pub struct PointIter {
    pub points: Vec<(f64, f64)>,
    pub index: usize,
}

impl Iterator for PointIter {
    type Item = (f64, f64);
    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        self.index += 1;
        self.points.get(index).map(|p| *p)
    }
}

impl From<Vec<(f64, f64)>> for PointIter {
    fn from(points: Vec<(f64, f64)>) -> Self {
        Self { points, index: 0 }
    }
}

pub trait Points {
    fn points(&self) -> PointIter;
}

pub trait Contains {
    fn contains_point(&self, point: (f64, f64)) -> bool;
}

impl<T, V> Collidable<T> for V
// because we are saying "for" here, our &self is V
where
    T: Points,
    V: Contains,
{
    fn collide(&self, other: &T) -> bool {
        for point in other.points() {
            if self.contains_point(point) {
                return true;
            }
        }
        return false;
    }
}
