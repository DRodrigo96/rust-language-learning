


fn retrieve_tuple(x: f32, y:f32) -> (f32, f32) {
    return (x, y)
}

fn main() {
    let xy: (f32, f32) = retrieve_tuple(-32.123, 5.0);
    let (_x, y): (f32, f32) = xy;
    
    if y > 5.0 {
        println!("y: >5");
        return
    }
    if y < 5.0 {
        println!("y: <5");
        return
    }
    println!("y: =5");
    return
}
