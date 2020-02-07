use num::Integer;

#[derive(Debug, Clone, PartialEq)]
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
    T: Integer + Copy,
{
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

    pub fn cardinal_neighbors(&self) -> [Self; 4] {
        [self.up(), self.right(), self.down(), self.left()]
    }

    pub fn up(&self) -> Self {
        Self::new(self.x, self.y + T::one())
    }

    pub fn up_right(&self) -> Self {
        Self::new(self.x + T::one(), self.y + T::one())
    }

    pub fn right(&self) -> Self {
        Self::new(self.x + T::one(), self.y)
    }

    pub fn down_right(&self) -> Self {
        Self::new(self.x + T::one(), self.y - T::one())
    }

    pub fn down(&self) -> Self {
        Self::new(self.x, self.y - T::one())
    }

    pub fn down_left(&self) -> Self {
        Self::new(self.x - T::one(), self.y - T::one())
    }

    pub fn left(&self) -> Self {
        Self::new(self.x - T::one(), self.y)
    }

    pub fn up_left(&self) -> Self {
        Self::new(self.x - T::one(), self.y + T::one())
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
}
