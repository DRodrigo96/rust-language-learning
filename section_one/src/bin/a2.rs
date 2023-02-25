

// activity 2
// ==================================================
// --------------------------------------------------

fn sum(a: i8, b: i8) -> i8 {
    return a + b
}

fn displayer(re: i8) {
    println!("sum is: {:?}", re)
}

fn main() {
    let result: i8 = sum(1, 5);
    displayer(result);
}
