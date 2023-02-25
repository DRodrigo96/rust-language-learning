

// control_flow
// ==================================================
// --------------------------------------------------

pub fn flowing() -> bool {
    let a: i32 = 10;
    let long: bool = true;
    
    if long {
        if a > 99 {
            if a > 200 {
                println!("HUGE number: {:?}", a);
            } else {
                println!("BIG number: {:?}", a);
            }
        } else {
            println!("SMALL number: {:?}", a);
        }
    } else {
        if a > 200 {
            println!("HUGE number: {:?}", a);
        } else if a > 99 {
            println!("BIG number: {:?}", a);
        } else {
            println!("SMALL number: {:?}", a);
        }
    }
    return true
}
