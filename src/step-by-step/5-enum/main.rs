enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    let dir = Direction::North;
    match dir {
        Direction::North => println!("We are heading north!"),
        Direction::South => println!("We are heading south!"),
        Direction::East => println!("We are heading east!"),
        Direction::West => println!("We are heading west!"),
    }
}
