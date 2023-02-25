

pub fn looping() {
    
    let mut i: i64 = 10;
    loop {
        println!("current number: {:?}", i);
        i -= 1;
        if i == 0 {
            println!("breaking ({:?})", i);
            break
        }
    }
    return
}

