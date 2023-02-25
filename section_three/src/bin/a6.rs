

fn while_displayer() {
    let mut x: i8 = 5;
    while x > 0 {
        println!("current: {:?}", x);
        x -= 1
    }
    println!("done!");
    return
}

fn main() {
    while_displayer();
}
