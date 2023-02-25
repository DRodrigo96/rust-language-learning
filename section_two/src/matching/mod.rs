

pub fn matching() {
    
    let m: bool = true;
    let i: i8 = 2;
    let name: &str = "demo";
    
    match m {
        true => println!("it's true"),
        false => println!("it's false"),
    }
    
    match i {
        1 => println!("its {:?}", i),
        2 => println!("its {:?}", i),
        3 => println!("its {:?}", i),
        _ => println!("its {:?}", i)
    }
    
    match name {
        "aaa"  => println!("its {:?}", name),
        "demo" => println!("its {:?}", name),
        "ccc"  => println!("its {:?}", name),
        _      => println!("its {:?}", name),
    }
}
