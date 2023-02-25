

enum Color {
    _Black,
    _White,
    _Yellow,
    Red
}

fn print_color(co: Color) {
    match co {
        Color::_Black  => println!("black"),
        Color::_White  => println!("white"),
        Color::_Yellow => println!("yellow"),
        Color::Red     => println!("red"),
    }
    return
}

fn main() {
    print_color(Color::Red);
}
