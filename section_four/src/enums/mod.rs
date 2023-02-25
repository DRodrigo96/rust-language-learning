

enum Direction {
    Left,
    _Right,
    _Up,
    _Down,
}

pub fn enums() {
    let dir: Direction = Direction::Left;
    match dir {
        Direction::Left   => println!("direction: left"),
        Direction::_Right => println!("direction: rigth"),
        Direction::_Up    => println!("direction: up"),
        Direction::_Down  => println!("direction: down"),
    }
    return
}
