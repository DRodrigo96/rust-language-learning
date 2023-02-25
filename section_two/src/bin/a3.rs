

// Logic with if & else
// ==================================================
// --------------------------------------------------

fn exercise_a(a: bool) {
    
    if a {
        println!("hello");
        return
    }
    println!("goodbye");
    return
}

fn exercise_b(b: i64) {
    
    if b == 5 {
        println!("=5");
        return
    }
    if b > 5 {
        println!(">5");
        return
    }
    println!("<5");
    return
}

fn main() {
    
    exercise_a(false);
    exercise_b(4);
}
