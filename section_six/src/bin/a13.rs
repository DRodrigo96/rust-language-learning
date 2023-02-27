

fn main() -> () {
    let value: Vec<u32> = vec![10, 20, 30, 40];
    
    for v in &value {
        match v {
            30 => println!("thirty"),
            _  => println!("{:?}", v)
        }
    }
    println!("total elements: {:?}", value.len());
    return
}
