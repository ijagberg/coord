use num::{Integer, Num, ToPrimitive};

#[derive(Debug, Clone, PartialEq, Hash, Default, Eq, Copy)]
pub struct Coord<T> {
    pub x: T,
    pub y: T,
}

impl<T> Coord<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Coord<T>
where
    T: Num + ToPrimitive + Copy,
{
    /// Calculate the Euclidean distance (straight line) between this Coord and `other`
    pub fn distance_to(&self, other: &Self) -> f64 {
        let x_diff = self.x - other.x;
        let y_diff = self.y - other.y;
        ((x_diff * x_diff) + (y_diff * y_diff))
            .to_f64()
            .unwrap()
            .sqrt()
    }

    /// Calculate the Manhattan distance between this Coord and `other`
    pub fn manhattan_distance(&self, other: &Self) -> f64 {
        let x_diff = self.x - other.x;
        let y_diff = self.y - other.y;
        num::abs(x_diff.to_f64().unwrap()) + num::abs(y_diff.to_f64().unwrap())
    }
}

impl<T> Coord<T>
where
    T: Integer + ToPrimitive + Copy,
{
    /// Returns a slice containing the 8 surrounding neighbors to this Coord
    pub fn neighbors(&self) -> [Self; 8] {
        [
            self.up(),
            self.up_right(),
            self.right(),
            self.down_right(),
            self.down(),
            self.down_left(),
            self.left(),
            self.up_left(),
        ]
    }

    /// Returns a slice containing the 4 surrounding cardinal neighbors to this Coord
    pub fn cardinal_neighbors(&self) -> [Self; 4] {
        [self.up(), self.right(), self.down(), self.left()]
    }

    /// Returns the Coord above this Coord
    pub fn up(&self) -> Self {
        Self::new(self.x, self.y + T::one())
    }

    /// Returns the Coord above and to the right of this Coord
    pub fn up_right(&self) -> Self {
        Self::new(self.x + T::one(), self.y + T::one())
    }

    /// Returns the Coord to the right of this Coord
    pub fn right(&self) -> Self {
        Self::new(self.x + T::one(), self.y)
    }

    /// Returns the Coord below and to the right of this Coord
    pub fn down_right(&self) -> Self {
        Self::new(self.x + T::one(), self.y - T::one())
    }

    /// Returns the Coord below this Coord
    pub fn down(&self) -> Self {
        Self::new(self.x, self.y - T::one())
    }

    /// Returns the Coord below and to the left of this Coord
    pub fn down_left(&self) -> Self {
        Self::new(self.x - T::one(), self.y - T::one())
    }

    /// Returns the Coord to the left of this Coord
    pub fn left(&self) -> Self {
        Self::new(self.x - T::one(), self.y)
    }

    /// Returns the Coord above and to the left of this Coord
    pub fn up_left(&self) -> Self {
        Self::new(self.x - T::one(), self.y + T::one())
    }
}

impl<T> From<(T, T)> for Coord<T> {
    fn from(tuple: (T, T)) -> Self {
        Coord::new(tuple.0, tuple.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neighbor() {
        let origo = Coord::new(0, 0);

        assert_eq!(
            [
                Coord::new(0, 1),
                Coord::new(1, 1),
                Coord::new(1, 0),
                Coord::new(1, -1),
                Coord::new(0, -1),
                Coord::new(-1, -1),
                Coord::new(-1, 0),
                Coord::new(-1, 1)
            ],
            origo.neighbors()
        );

        assert_eq!(
            [
                Coord::new(0, 1),
                Coord::new(1, 0),
                Coord::new(0, -1),
                Coord::new(-1, 0)
            ],
            origo.cardinal_neighbors()
        );
    }

    #[test]
    fn euclidean_distance() {
        assert_eq!(
            1.4142135623730951,
            Coord::new(0, 0).distance_to(&Coord::new(1, 1))
        );
        assert_eq!(
            1.4142135623730951,
            Coord::new(0., 0.).distance_to(&Coord::new(-1., -1.))
        );
    }

    #[test]
    fn manhattan_distance() {
        assert_eq!(
            6,
            Coord::new(0, 0).manhattan_distance(&Coord::new(3, 3)) as u64
        );
        assert_eq!(
            6,
            Coord::new(0, 0).manhattan_distance(&Coord::new(-3, -3)) as u64
        );
        assert_eq!(
            2.5,
            Coord::new(0., 0.).manhattan_distance(&Coord::new(0.0, -2.5))
        );
    }
}
