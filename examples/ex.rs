use coord::Coord;

fn main() {
    let origo = Coord::new(0, 0);
    let other = Coord::new(1, 1);

    println!("{:?}", origo);
    println!("{:?}", other);
    println!("{:?}", origo.distance_to(&other));
}
