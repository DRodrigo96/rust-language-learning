

// Decision making with "match"
// ==================================================
// --------------------------------------------------

fn exercise_a(a: bool) {
    
    match a {
        true => println!("its {:?}", a),
        false => println!("its {:?}", a),
    }
    return
}

fn exercise_b(b: i64) {
    
    match b {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other ({:?})", b)
    }
    return
}

fn main() {
    
    exercise_a(true);
    exercise_b(2);
}
