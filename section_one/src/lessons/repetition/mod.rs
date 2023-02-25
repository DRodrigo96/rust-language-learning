

pub fn standard_loop() -> bool {
    
    let mut a: i32 = 0;
    
    loop {
        if a == 5 {
            println!("breaking standard loop");
            break
        }
        println!("[loop] current value: {:?}", a);
        a = a +1;
    }
    return true;
}

pub fn while_loop() -> bool {
    
    let mut a: i32 = 0;
    
    while a != 5 {
        println!("[while] current value: {:?}", a);
        a += 1;
    }
    println!("while loop done");
    return true;
}
