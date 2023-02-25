

pub fn tuples() {
    let coord_xy: (f32, f32) = (-34.121, 32.232);
    println!("coordenadas A: ({:?},{:?})", coord_xy.0, coord_xy.1);
    
    let (coord_x, coord_y): (f64, f64) = (-40.520, -76.232);
    println!("coordenadas B: ({:?},{:?})", coord_x, coord_y);
    
    let (name, age): (&str, u8) = ("John Doe", 32);
    println!("Name-Age: {:?},{:?}", name, age);
    
    let favorites: (&str, u8, char, &str, f32) = ("red", 14, 'T', "pizza", 0.05);
    println!(
        "color: {:?}, number: {:?}, letter: {:?}, food: {:?}, decimal: {:?}",
        favorites.0, favorites.1, favorites.2, favorites.3, favorites.4
    );
    return
}
