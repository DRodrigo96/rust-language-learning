

fn display_number() {
    
    let mut i: i64 = 0;
    loop {
        i += 1;
        println!("number {:?}", i);
        if i == 4 {
            break
        }
    }
    return
}

fn main() {
    display_number();
}
