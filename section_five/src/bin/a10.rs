


fn show_size(var: u32) -> () {
    let size: bool = var > 100;
    match size {
        true  => println!("it's big ({:?})", size),
        false => println!("it's small ({:?})", size)
    }
    return
}

fn main() -> () {
    show_size(120);
}
